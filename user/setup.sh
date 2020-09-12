#!/bin/bash

set -o errexit -o nounset

DOTFILESP="$(realpath "$(dirname "$0")/..")"
DOTUSERP="$DOTFILESP/user"
PATH="$DOTFILESP/user/bin:$PATH"

# bin
dotln "$DOTUSERP/bin" "$HOME/.local/dotbin"

# lib
dotln "$DOTUSERP/lib" "$HOME/.local/dotlib"

# shell
dotln "$DOTUSERP/shell/profile" "$HOME/.profile"
dotln "$DOTUSERP/shell/bashrc" "$HOME/.bashrc"
dotln "$DOTUSERP/shell/inputrc" "$HOME/.inputrc"
dotln "$DOTUSERP/shell/bashrc.d" "$HOME/.bashrc.d"

# git
dotln "$DOTUSERP/git" "$HOME/.config/"

# i3
dotln "$DOTUSERP/i3" "$HOME/.config/"

# GTK
dotln "$DOTUSERP/gtk/gtk-2.0/gtkrc-2.0" "$HOME/.gtkrc-2.0"
dotln "$DOTUSERP/gtk/gtk-3.0" "$HOME/.config/"

# power manager
dotcp "$DOTUSERP/xfce4/power-manager.xml" \
      "$HOME/.config/xfce4/xfconf/xfce-perchannel-xml/"

# file manager
dbus-launch dconf load / < "$DOTUSERP/nemo/dconf"
dotln "$DOTUSERP/nemo/actions" "$HOME/.local/share/nemo/"

# notifications
dotln "$DOTUSERP/dunst" "$HOME/.config/"

# xorg
dotln "$DOTUSERP/xorg/xinitrc" "$HOME/.xinitrc"

# xdg
dotln "$DOTUSERP/xdg/user-dirs.conf" "$HOME/.config/"

# mpv
dotln "$DOTUSERP/mpv" "$HOME/.config/"

# terminal
dotln "$DOTUSERP/kitty" "$HOME/.config/"

# blender
if command -v blender; then
    blender --background --python "$DOTUSERP/blender/userprefs.py"
fi > /dev/null
dotln "$DOTUSERP/blender/blenderimport.desktop" "$HOME/.local/share/applications/"

# cache
dotln '/dev/null' "$HOME/.python_history"
