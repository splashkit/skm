#!/usr/bin/env bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

INSTALL_MACOS_PATH=~/.splashkit/splashkit-macos
INSTALL_SKM_PATH=~/.splashkit/skm

# Check for clang
if which clang++ >/dev/null; then
  CLANG_EXISTS=true
else
  "SplashKit will install, but we can not find a clang compiler. Please run Xcode developer command."
fi

# Clone the repos if git is found.
if which git >/dev/null; then
    echo 'Git found'
    git clone --depth 1 $GIT_MACOS_REPO $INSTALL_MACOS_PATH
    git clone -b master --depth 1 --single-branch $GIT_SKM_REPO $INSTALL_SKM_PATH
  else
    echo 'Git not found, can''t install splashkit. Please run xcode developer command.'
fi

# Unzip the SKM app.
unzip ~/.splashkit/skm/mac-build/skm.zip -d ~/.splashkit/skm/mac-build > ~/.splashkit/install.log

# Add SKM app to path without needing sudo
echo "export PATH=\""$HOME/.splashkit/skm/mac-build/skm.app/Contents/MacOs:$PATH\""" >> ~/.bash_profile
source ~/.bash_profile

echo "SplashKit Successfully installed! Please restart your terminal..."
