#!/usr/bin/env bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm-windows
GIT_WINDOWS_REPO=https://github.com/splashkit/splashkit-windows

INSTALL_PATH=~/.splashkit/splashkit-windows
INSTALL_SKM_PATH=~/.splashkit/skm

# Get gcc if not installed

command -v clang >/dev/null 2>&1 || { echo "I require clang but it's not installed. Installing clang." >&2; pacman -S mingw-w64-x86_64-clang --noconfirm;}

# if which clang >/dev/null; then
#   pacman -S mingw-w64-x86_64-clang --noconfirm
# fi

# Get git if not installed
if which git >/dev/null; then
  pacman -S git --noconfirm
fi

# Get unzip if not installed, needed for skm app.
if which unzip >/dev/null; then
  pacman -S unzip --noconfirm
fi

sleep 5

# Clone the repos.
git clone --depth 1 $GIT_WINDOWS_REPO $INSTALL_PATH

sleep 5

git clone -b master --depth 1 --single-branch $GIT_SKM_REPO $INSTALL_SKM_PATH

# Unzip the SKM app.
unzip $INSTALL_PATH/skm.zip -d $INSTALL_PATH/mac-build > ~/.splashkit/install.log

# Add SKM app to path
ln -sf $INSTALL_PATH/skm.app/Contents/MacOS/skm /usr/local/bin