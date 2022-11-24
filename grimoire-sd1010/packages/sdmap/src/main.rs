use evdev::{uinput, InputEvent, Key};

fn key2key(evt_in: InputEvent, key: Key) -> InputEvent {
    return InputEvent::new(evdev::EventType::KEY, key.code(), evt_in.value());
}

fn kbd_map(evt_in: InputEvent) -> Vec<InputEvent> {
    // note: couldn't find a way to use pattern matching :(
    if evt_in.code() == Key::BTN_TL.0 {
        vec!(key2key(evt_in, Key::BTN_RIGHT))
    } else if evt_in.code() == Key::BTN_TR.0 {
        vec!(key2key(evt_in, Key::BTN_LEFT))
    } else if evt_in.code() == Key::BTN_TL2.0 {
        vec!(key2key(evt_in, Key::BTN_MIDDLE))
    } else if evt_in.code() == Key::BTN_TR2.0 {
        vec!(key2key(evt_in, Key::KEY_LEFTMETA))
    } else if evt_in.code() == Key::BTN_DPAD_UP.0 {
        vec!(key2key(evt_in, Key::KEY_UP))
    } else if evt_in.code() == Key::BTN_DPAD_DOWN.0 {
        vec!(key2key(evt_in, Key::KEY_DOWN))
    } else if evt_in.code() == Key::BTN_DPAD_LEFT.0 {
        vec!(key2key(evt_in, Key::KEY_LEFT))
    } else if evt_in.code() == Key::BTN_DPAD_RIGHT.0 {
        vec!(key2key(evt_in, Key::KEY_RIGHT))
    } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY1.0 {
        vec!(key2key(evt_in, Key::KEY_LEFTSHIFT))
    } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY3.0 {
        vec!(key2key(evt_in, Key::KEY_LEFTCTRL))
    } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY2.0 {
        vec!(key2key(evt_in, Key::KEY_RIGHTALT))
    } else if evt_in.code() == Key::BTN_TRIGGER_HAPPY4.0 {
        vec!(key2key(evt_in, Key::KEY_LEFTALT))
    } else if evt_in.code() == Key::BTN_SELECT.0 {
        vec!(key2key(evt_in, Key::KEY_TAB))
    } else if evt_in.code() == Key::BTN_START.0 {
        vec!(key2key(evt_in, Key::KEY_COMPOSE))
    } else {
        vec!()
    }
}

fn main() -> std::io::Result<()> {
    let path_in = "/dev/input/by-id/usb-Valve_Software_Steam_Controller_123456789ABCDEF-if02-event-joystick";
    let mut dev_in = evdev::Device::open(path_in)?;
    dev_in.grab()?;

    let mut kbd_mode = true;

    let mut dev_keyboard = uinput::VirtualDeviceBuilder::new()?
        .name("Steam Deck sdmapd keyboard")
        .with_keys(&evdev::AttributeSet::from_iter([
            Key::BTN_RIGHT, Key::BTN_LEFT, Key::BTN_MIDDLE, Key::KEY_LEFTMETA,
            Key::KEY_UP, Key::KEY_DOWN, Key::KEY_LEFT, Key::KEY_RIGHT,
            Key::KEY_LEFTSHIFT, Key::KEY_LEFTCTRL, Key::KEY_RIGHTALT,
            Key::KEY_LEFTALT, Key::KEY_TAB, Key::KEY_COMPOSE
        ]))?
        .build()?;


    let mut dev_gamepad = uinput::VirtualDeviceBuilder::new()?
        .name("Steam Deck sdmapd gamepad")
        .with_keys(dev_in.supported_keys().unwrap())?
        .build()?;


    loop {
        let new_events: Vec<InputEvent> = dev_in.fetch_events()?.collect();
        // the cached state is updated by fetching events just before
        let cached_keys: Vec<Key> = dev_in.cached_state().key_vals().unwrap()
                                          .iter().collect();
        println!("{:?}", cached_keys); // DEBUG

        if cached_keys.eq(&[Key::BTN_BASE, Key::BTN_MODE]) {
            kbd_mode = !kbd_mode;
            continue;
        }

        for evt_in in new_events {
            if kbd_mode {
                let evts_out = kbd_map(evt_in);
                println!("KEYBOARD {:?} => {:?}", evt_in, evts_out); // DEBUG
                dev_keyboard.emit(&evts_out)?;
            } else {
                println!("GAMEPAD {:?}", evt_in); // DEBUG
                dev_gamepad.emit(&[evt_in])?;
            }
        }
    }
}
