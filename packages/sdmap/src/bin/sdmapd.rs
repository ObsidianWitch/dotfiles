use evdev::{*, AbsoluteAxisType as Abs, PropType as Prop};
use libc::input_absinfo;
use sdmap::VKBD_LAYOUT;

trait FromInputAbsinfo { fn from_libc(absinfo: input_absinfo) -> AbsInfo; }
impl FromInputAbsinfo for AbsInfo {
    fn from_libc(absinfo: input_absinfo) -> AbsInfo {
        AbsInfo::new(absinfo.value, absinfo.minimum, absinfo.maximum,
                     absinfo.fuzz, absinfo.flat, absinfo.resolution)
    }
}

struct Sdmapd {
    dev_in: Device,
    absinfos_in: [input_absinfo; 64],
    cache_in: DeviceState,
    dev_keyboard: uinput::VirtualDevice,
    dev_mouse: uinput::VirtualDevice,
    kbd_mode: bool,
    touch: bool
}
impl Sdmapd {
    pub fn new() -> std::io::Result<Self> {
        let path_in = "/dev/input/by-id/usb-Valve_Software_Steam_Controller_123456789ABCDEF-if02-event-joystick";
        let mut dev_in = Device::open(path_in)?;
        dev_in.grab()?;
        let absinfos_in = dev_in.get_abs_state()?;

        let dev_keyboard = uinput::VirtualDeviceBuilder::new()?
            .name("Steam Deck sdmapd keyboard")
            .with_keys(&AttributeSet::from_iter(
                VKBD_LAYOUT.into_iter().flatten().flatten().chain([
                Key::KEY_LEFTMETA, Key::KEY_UP, Key::KEY_DOWN, Key::KEY_LEFT,
                Key::KEY_RIGHT, Key::KEY_LEFTSHIFT, Key::KEY_LEFTCTRL, Key::KEY_RIGHTALT,
                Key::KEY_LEFTALT, Key::KEY_TAB, Key::KEY_COMPOSE, Key::KEY_PAGEUP,
                Key::KEY_PAGEDOWN, Key::KEY_HOME, Key::KEY_END, Key::KEY_ENTER,
                Key::KEY_ESC, Key::KEY_BACKSPACE, Key::KEY_SPACE
            ])))?
            .build()?;
        let dev_mouse = uinput::VirtualDeviceBuilder::new()?
            .name("Steam Deck sdmapd mouse")
            .with_keys(&AttributeSet::from_iter([
                Key::BTN_RIGHT, Key::BTN_LEFT, Key::BTN_MIDDLE,  Key::BTN_TOUCH,
                Key::BTN_TOOL_FINGER
            ]))?
            .with_absolute_axis(&UinputAbsSetup::new(
                Abs::ABS_X,
                AbsInfo::from_libc(absinfos_in[Abs::ABS_HAT1X.0 as usize])
            ))?
            .with_absolute_axis(&UinputAbsSetup::new(
                Abs::ABS_Y,
                AbsInfo::from_libc(absinfos_in[Abs::ABS_HAT1Y.0 as usize])
            ))?
            .with_properties(&AttributeSet::from_iter([Prop::POINTER]))?
            .build()?;

        Ok(Self {
            absinfos_in,
            cache_in: dev_in.cached_state().clone(),
            dev_in,
            dev_keyboard,
            dev_mouse,
            kbd_mode: true,
            touch: false,
        })
    }

    // Create a new Key event.
    fn new_key(&self, key: Key, value: i32) -> InputEvent {
        InputEvent::new(EventType::KEY, key.0, value)
    }

    // Create a new Abs event.
    fn new_abs(&self, abs: Abs, value: i32) -> InputEvent {
        InputEvent::new(EventType::ABSOLUTE, abs.0, value)
    }

    // Map absolute events to trackpad events (ABS_X/Y).
    // ref: https://www.kernel.org/doc/Documentation/input/event-codes.txt
    fn abs2trackpad(&mut self, evt_in: InputEvent, abs_out: Abs, factor: i32)
    -> Vec<InputEvent> {
        let touched = if evt_in.value() == 0 { 0 }
                 else if !self.touch { 1 }
                 else { -1 };
        let mut vec = if touched >= 0 {
            self.touch = touched != 0;
            vec!(self.new_key(Key::BTN_TOUCH, touched),
                 self.new_key(Key::BTN_TOOL_FINGER, touched))
        } else {
            vec!()
        };
        vec.push(self.new_abs(abs_out, evt_in.value() * factor));
        vec
    }

    // Map the minimum and maximum values of a joystick axis to the `key_min` and
    // `key_max` key events.
    fn joy2keys(&self, evt_in: InputEvent, key_min: Key, key_max: Key)
    -> Vec<InputEvent> {
        let absinfo = self.absinfos_in[evt_in.code() as usize];
        if evt_in.value().abs() <= absinfo.resolution {
            vec!(self.new_key(key_min, 0), self.new_key(key_max, 0))
        } else if evt_in.value() == absinfo.minimum {
            vec!(self.new_key(key_min, 1))
        } else if evt_in.value() == absinfo.maximum {
            vec!(self.new_key(key_max, 1))
        } else {
            vec!()
        }
    }

    fn vkbd_keypos(&self) -> (usize, usize) {
        let absinfo = self.absinfos_in[Abs::ABS_HAT0X.0 as usize];
        let absvals = self.cache_in.abs_vals().unwrap();
        let y = (
            (absvals[Abs::ABS_HAT0Y.0 as usize].value - absinfo.maximum).abs()
            * VKBD_LAYOUT.len() as i32
        ) / ((absinfo.maximum * 2) + 1);
        let x = (
            (absvals[Abs::ABS_HAT0X.0 as usize].value + absinfo.maximum)
            * VKBD_LAYOUT[0].len() as i32
        ) / ((absinfo.maximum * 2) + 1);
        (x as usize, y as usize)
    }

    // Map a physical key to a key of the virtual keyboard depending on the current
    // value of ABS_HAT0{X,Y}. If ABS_HAT0{X,Y} == (0, 0), send the `fallback_key`.
    fn key2vkbd(&self, evt_in: InputEvent, ki: usize, fallback_key: Key)
    -> Vec<InputEvent> {
        let absvals = self.cache_in.abs_vals().unwrap();
        if evt_in.value() == 0 {
            vec!()
        } else if absvals[Abs::ABS_HAT0X.0 as usize].value != 0
               && absvals[Abs::ABS_HAT0Y.0 as usize].value != 0
        {
            let keypos = self.vkbd_keypos();
            let key = VKBD_LAYOUT[keypos.1][keypos.0][ki];
            vec!(self.new_key(key, 1), self.new_key(key, 0))
        } else {
            vec!(self.new_key(fallback_key, 1), self.new_key(fallback_key, 0))
        }
    }

    fn kbd_map(&mut self, evt_in: InputEvent) -> Vec<InputEvent> {
        if evt_in.code() == Key::BTN_TR2.0 {
            vec!(self.new_key(Key::KEY_LEFTMETA, evt_in.value()))
        } else if evt_in.code() == Key::BTN_DPAD_UP.0 {
            vec!(self.new_key(Key::KEY_UP, evt_in.value()))
        } else if evt_in.code() == Key::BTN_DPAD_DOWN.0 {
            vec!(self.new_key(Key::KEY_DOWN, evt_in.value()))
        } else if evt_in.code() == Key::BTN_DPAD_LEFT.0 {
            vec!(self.new_key(Key::KEY_LEFT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_DPAD_RIGHT.0 {
            vec!(self.new_key(Key::KEY_RIGHT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY1.0 {
            vec!(self.new_key(Key::KEY_LEFTSHIFT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY3.0 {
            vec!(self.new_key(Key::KEY_LEFTCTRL, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY2.0 {
            vec!(self.new_key(Key::KEY_RIGHTALT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY4.0 {
            vec!(self.new_key(Key::KEY_LEFTALT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_SELECT.0 {
            vec!(self.new_key(Key::KEY_TAB, evt_in.value()))
        } else if evt_in.code() == Key::BTN_START.0 {
            vec!(self.new_key(Key::KEY_COMPOSE, evt_in.value()))
        } else if evt_in.code() == Abs::ABS_Y.0 {
            self.joy2keys(evt_in, Key::KEY_PAGEUP, Key::KEY_PAGEDOWN)
        } else if evt_in.code() == Abs::ABS_X.0 {
            self.joy2keys(evt_in, Key::KEY_HOME, Key::KEY_END)
        } else if evt_in.code() == Key::BTN_SOUTH.0 {
            self.key2vkbd(evt_in, 0, Key::KEY_ENTER)
        } else if evt_in.code() == Key::BTN_EAST.0 {
            self.key2vkbd(evt_in, 1, Key::KEY_ESC)
        } else if evt_in.code() == Key::BTN_NORTH.0 {
            self.key2vkbd(evt_in, 2, Key::KEY_BACKSPACE)
        } else if evt_in.code() == Key::BTN_WEST.0 {
            self.key2vkbd(evt_in, 3, Key::KEY_SPACE)
        } else {
            vec!()
        }
    }

    fn mouse_map(&mut self, evt_in: InputEvent) -> Vec<InputEvent> {
        if evt_in.code() == Key::BTN_TL.0 {
            vec!(self.new_key(Key::BTN_RIGHT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TR.0 {
            vec!(self.new_key(Key::BTN_LEFT, evt_in.value()))
        } else if evt_in.code() == Key::BTN_TL2.0 {
            vec!(self.new_key(Key::BTN_MIDDLE, evt_in.value()))
        } else if evt_in.code() == Abs::ABS_HAT1X.0 {
            self.abs2trackpad(evt_in, Abs::ABS_X, 1)
        } else if evt_in.code() == Abs::ABS_HAT1Y.0 {
            self.abs2trackpad(evt_in, Abs::ABS_Y, -1)
        } else {
            vec!()
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.cache_in = self.dev_in.cached_state().clone();
            if self.cache_in.key_vals().unwrap().iter()
                   .eq([Key::BTN_BASE, Key::BTN_MODE])
            {
                self.kbd_mode = !self.kbd_mode;
                if self.kbd_mode {
                    self.dev_in.grab()?;
                } else {
                    self.dev_in.ungrab()?;
                }
            }

            let events_in: Vec<InputEvent> = self.dev_in.fetch_events()?.collect();
            let events_kbd: Vec<InputEvent> = events_in.iter().cloned()
                .flat_map(|evt_in| self.kbd_map(evt_in))
                .collect();
            self.dev_keyboard.emit(&events_kbd)?;
            let events_mouse: Vec<InputEvent> = events_in.into_iter()
                .flat_map(|evt_in| self.mouse_map(evt_in))
                .collect();
            self.dev_mouse.emit(&events_mouse)?;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut sdmapd = Sdmapd::new()?;
    sdmapd.run()?;
    Ok(())
}
