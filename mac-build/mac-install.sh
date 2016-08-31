#!/bin/bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

INSTALL_MACOS_PATH=~/splashkitTest/splashkit-macos
INSTALL_SKM_PATH=~/splashkitTest/skm

if which clang++ >/dev/null; then
  CLANG_EXISTS=true
else
  CLANG_EXISTS=false
fi

if which git >/dev/null; then
  GIT_EXISTS=true
else
  GIT_EXISTS=false
fi

function installGit() {
  echo Install Git Here
}

if [ "$GIT_EXISTS" = true ] ; then
    echo 'Git found'
    env -i git clone $GIT_MACOS_REPO $INSTALL_MACOS_PATH
    env -i git clone $GIT_SKM_REPO $INSTALL_SKM_PATH
  else
    echo 'Git not found, installing git...'
    installGit
fi

unzip ~/splashkitTest/skm/mac-build/skm.zip -d ~/splashkitTest/skm/mac-build

ln -sf ~/splashkitTest/skm/mac-build/skm.app/Contents/MacOS/skm /usr/local/bin
