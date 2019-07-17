#!/bin/bash

layout_laptop() {
    xrandr --output HDMI1 --off \
           --output eDP1  --auto
}

layout_home() {
    xrandr --output HDMI1 --auto --left-of eDP1 \
           --output eDP1  --auto
}

if xrandr | grep -q 'HDMI1 disconnected'; then
    layout_laptop
else
    layout_home
fi
