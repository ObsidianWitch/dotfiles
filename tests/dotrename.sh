#!/bin/bash

set -o errexit -o nounset -o xtrace

DOTFILESP="$(realpath "$(dirname "$0")/..")"
PATH="$DOTFILESP/user/bin:$PATH"

tmpdir=$(mktemp -d)
trap 'rm -r "$tmpdir"' EXIT

echo 'a' > "$tmpdir/a"
dotrename "$tmpdir/a"
[[ -f "$tmpdir/3f786850e387550fdab836ed7e6dc881de23001b" ]]

echo 'b' > "$tmpdir/b.b"
dotrename "$tmpdir/b.b"
[[ -f "$tmpdir/89e6c98d92887913cadf06b2adb97f26cde4849b.b" ]]

echo 'c' > "$tmpdir/c.c.c"
dotrename "$tmpdir/c.c.c"
[[ -f "$tmpdir/2b66fd261ee5c6cfc8de7fa466bab600bcfe4f69.c.c" ]]

longstr=$(printf '%*s' 240 | tr ' ' 'd')
echo 'c' > "$tmpdir/d.$longstr"
! dotrename "$tmpdir/d.$longstr" 2>/dev/null
[[ -f "$tmpdir/d.$longstr" ]]
