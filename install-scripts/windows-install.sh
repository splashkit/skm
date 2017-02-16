#!/bin/bash
GIT_WINDOWS_REPO=https://github.com/splashkit/splashkit-windows

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then
    setx MSYS2_PATH_TYPE inherit
    echo Preinstall Succcessful! Please restart your terminal and rerun this script to install SplashKit.
else
    # Get tools if not installed
    command -v git >/dev/null 2>&1 || { echo "git not found, Installing git." >&2; pacman -S git --noconfirm;}

    # Clone the repo.
    git clone --depth 1 --branch master $GIT_WINDOWS_REPO "${INSTALL_PATH}"

    #Export to path -- for current terminal
    export PATH="$HOME/.splashkit/lib:$PATH"
    export PATH="$HOME/.splashkit/lib/win32:$PATH"
    export PATH="$HOME/.splashkit/lib/win64:$PATH"
    export PATH="$HOME/.splashkit/skm-win32-ia32:$PATH"

    #Export path for new terminals
    export ORIGINAL_PATH="$HOME/.splashkit/lib:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit/lib/win32:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit/lib/win64:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit/skm-win32-ia32:$ORIGINAL_PATH"

    # Set path
    setx PATH "$ORIGINAL_PATH"
fi