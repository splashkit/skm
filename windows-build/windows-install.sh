#!/bin/bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm
GIT_WINDOWS_REPO=https://github.com/splashkit/splashkit-windows

HOME_PATH=~
FOLDER_PATH="${HOME_PATH}/.splashkit"
INSTALL_PATH="${FOLDER_PATH}/splashkit-windows"
INSTALL_SKM_PATH="${FOLDER_PATH}/skm"

if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then 
    setx MSYS2_PATH_TYPE inherit
    echo Please restart your terminal and rerun this script.
else
    # echo "export PATH=\""$HOME/.splashkit/splashkit-windows/lib:\$PATH\""" >> ~/.bash_profile
    export PATH=\""$HOME/.splashkit/splashkit-windows/lib:\$PATH\""

    # Get tools if not installed
    command -v clang >/dev/null 2>&1 || { echo "clang not found, Installing clang." >&2; pacman -S mingw-w64-x86_64-clang mingw-w64-i686-clang --noconfirm;}
    command -v git >/dev/null 2>&1 || { echo "git not found, Installing git." >&2; pacman -S git --noconfirm;}
    command -v unzip >/dev/null 2>&1 || { echo "unzip not found, Installing unzip." >&2; pacman -S unzip --noconfirm;}

    # Clone the repos.
    echo git clone --depth 1 $GIT_WINDOWS_REPO "${INSTALL_PATH}"
    git clone --depth 1 $GIT_WINDOWS_REPO "${INSTALL_PATH}"

    echo git clone -b master --depth 1 --single-branch $GIT_SKM_REPO "${INSTALL_SKM_PATH}"
    git clone -b master --depth 1 --single-branch $GIT_SKM_REPO "${INSTALL_SKM_PATH}"

    # Unzip the SKM app.
    # unzip "$INSTALL_SKM_PATH/windows-build/skm.zip" -d "${INSTALL_PATH}/windows-build" > "${FOLDER_PATH}/install.log"

    # Shouldn't this just use the Linux build?
    # Add SKM app to path
    # echo "export PATH=\""$HOME/.splashkit/skm/windows-build/skm.app/Contents/windows:\$PATH\""" >> ~/.bash_profile
    export PATH=\""$HOME/.splashkit/skm/windows-build/skm-win32-x64:\$PATH\""
    ${SYSTEMROOT}/system32/setx.exe PATH $PATH
fi