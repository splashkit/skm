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

if [ "$GIT_EXISTS" = true ] ; then
    echo 'Git found'
    git clone --depth 1 $GIT_MACOS_REPO $INSTALL_MACOS_PATH
    git clone -b master --depth 1 --single-branch $GIT_SKM_REPO $INSTALL_SKM_PATH
  else
    echo 'Git not found, can''t install splashkit'
fi

unzip ~/.splashkit/skm/mac-build/skm.zip -d ~/.splashkit/skm/mac-build > ~/.splashkit/install.log

echo "export PATH=\""$HOME/.splashkit/skm/mac-build/skm.app/Contents/MacOs:$PATH\""" >> ~/.bash_profile
source ~/.bash_profile

echo "SplashKit Successfully installed! Please restart your terminal..."
