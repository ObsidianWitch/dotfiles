#!/bin/sh

# i3
rm $HOME/.i3

# Nemo
rm $HOME/.local/share/nemo/actions

# Nemo - restore desktop management
# requirement: libglib2.0-bin (contains gsettings)
gsettings set org.nemo.desktop show-desktop-icons true

# GTK
rm $HOME/.gtkrc-2.0
rm $HOME/.config/gtk-3.0/gtk.css
rm $HOME/.config/gtk-3.0/settings.ini

# X
rm $HOME/.xinitrc
rm $HOME/.xmodmaprc
rm $HOME/.xsessionrc

# Xfce
rm $HOME/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-power-manager.xml
rm $HOME/.config/xfce4/terminal/terminalrc

# Notifications
rm $HOME/.config/dunst/dunstrc

# Screen layouts
rm $HOME/.screenlayout

# MPlayer
rm $HOME/.mplayer

# Atom
rm $HOME/.atom/keymap.cson
rm $HOME/.atom/styles.less

# Chromium
rm $HOME/.config/chromium-flags.conf
