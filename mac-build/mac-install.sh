#!/usr/bin/env bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

INSTALL_MACOS_PATH=~/.splashkit/splashkit-macos
INSTALL_SKM_PATH=~/.splashkit/skm

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
    env -i git clone -b develop --single-branch $GIT_MACOS_REPO $INSTALL_MACOS_PATH
    env -i git clone -b master --single-branch $GIT_SKM_REPO $INSTALL_SKM_PATH
  else
    echo 'Git not found, installing git...'
    installGit
fi

unzip ~/.splashkit/skm/mac-build/skm.zip -d ~/.splashkit/skm/mac-build > ~/.splashkit/install.log

ln -sf ~/.splashkit/skm/mac-build/skm.app/Contents/MacOS/skm /usr/local/bin
