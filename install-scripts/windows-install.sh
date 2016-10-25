#!/bin/bash
GIT_WINDOWS_REPO=https://github.com/splashkit/splashkit-windows

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then 
    setx MSYS2_PATH_TYPE inherit
    echo Please restart your terminal and rerun this script.
else
    # Get tools if not installed
    command -v clang >/dev/null 2>&1 || { echo "clang not found, Installing clang." >&2; pacman -S mingw-w64-x86_64-clang mingw-w64-i686-clang --noconfirm;}
    command -v git >/dev/null 2>&1 || { echo "git not found, Installing git." >&2; pacman -S git --noconfirm;}
    command -v unzip >/dev/null 2>&1 || { echo "unzip not found, Installing unzip." >&2; pacman -S unzip --noconfirm;}

    # Clone the repo.
    git clone --depth 1 $GIT_WINDOWS_REPO "${INSTALL_PATH}"

    #Export to path
    export PATH="$HOME/.splashkit/lib:$PATH"
    export PATH="$HOME/.splashkit/lib/win32:$PATH"
    export PATH="$HOME/.splashkit/lib/win64:$PATH"

    export PATH="$HOME/.splashkit/skm-win32-x64:$PATH"
    export PATH="$HOME/.splashkit/skm-win32-ia32:$PATH"

    # Set path
    setx PATH "$PATH"
fi