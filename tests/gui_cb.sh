#!/bin/bash

DOTFILESP="$(realpath "$(dirname "$0")/..")"
PATH="$DOTFILESP/cli/bin:$PATH"
source dotfail

test_cbtxt() {
    shellcheck "$DOTFILESP/gui/bin/cbtxt"

    local str='Lorem ipsum dolor sit amet, consectetur adipiscing elit.'
    cbtxt "$str"
    [[ "$(xclip -selection clipboard -out)" == "$str" ]] || dotfail
}

test_cbfiles() {
    shellcheck "$DOTFILESP/gui/bin/cbfiles"

    local str="file://$(realpath "$0")"
    cbfiles "$0"
    [[ "$(xclip -selection clipboard -out)" \
        == "$(echo -e "copy\n$str")" ]] || dotfail
}

test_cbtxt
test_cbfiles

exit 0
