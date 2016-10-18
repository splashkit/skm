#!/usr/bin/env bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm-windows
GIT_WINDOWS_REPO=https://github.com/splashkit/splashkit-windows

INSTALL_PATH=~/.splashkit/splashkit-windows
INSTALL_SKM_PATH=~/.splashkit/skm

# Get gcc if not installed

command -v clang >/dev/null 2>&1 || { echo "clang not found, Installing clang." >&2; pacman -S mingw-w64-x86_64-clang mingw-w64-i686-clang --noconfirm;}

# if which clang >/dev/null; then
#   pacman -S mingw-w64-x86_64-clang --noconfirm
# fi

command -v git >/dev/null 2>&1 || { echo "git not found, Installing git." >&2; pacman -S mingw-w64-x86_64-clang --noconfirm;}


# Get git if not installed
# if which git >/dev/null; then
#   pacman -S git --noconfirm
# fi

command -v unzip >/dev/null 2>&1 || { echo "unzip not found, Installing unzip." >&2; pacman -S mingw-w64-x86_64-clang --noconfirm;}

# Get unzip if not installed, needed for skm app.
# if which unzip >/dev/null; then
#   pacman -S unzip --noconfirm
# fi

sleep 5

# Clone the repos.
echo git clone --depth 1 $GIT_WINDOWS_REPO "$INSTALL_PATH"
git clone --depth 1 $GIT_WINDOWS_REPO "$INSTALL_PATH"

sleep 5

echo git clone -b master --depth 1 --single-branch $GIT_SKM_REPO "$INSTALL_SKM_PATH"
git clone -b master --depth 1 --single-branch $GIT_SKM_REPO "$INSTALL_SKM_PATH"



sleep 5

# Unzip the SKM app.
unzip $INSTALL_PATH/skm.zip -d $INSTALL_PATH/mac-build > ~/.splashkit/install.log

sleep 5

# Add SKM app to path
ln -sf $INSTALL_PATH/skm.app/Contents/MacOS/skm /bin