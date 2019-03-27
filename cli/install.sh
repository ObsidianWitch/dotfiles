#!/bin/sh

$PWD/cli/remove.sh

# bin
mkdir -p $HOME/.local/bin
for bin in $PWD/cli/bin/*; do
    if [[ ! -f $bin ]]; then continue; fi
    ln -s $bin $HOME/.local/bin/`basename ${bin%.*}`
done


# shell
ln -s $PWD/cli/shell/bashrc $HOME/.bashrc
ln -s $PWD/cli/shell/prompt.sh $HOME/.prompt.sh
ln -s $PWD/cli/shell/inputrc $HOME/.inputrc
ln -s $PWD/cli/shell/profile $HOME/.profile

# git
ln -s $PWD/cli/git/gitconfig $HOME/.gitconfig
