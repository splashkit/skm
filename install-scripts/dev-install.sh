#!/bin/bash
GIT_SKM_REPO=https://github.com/splashkit/skm

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

if [[ `uname` = MINGW* ]] || [[ `uname` = MSYS* ]]; then
    if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then
        setx MSYS2_PATH_TYPE inherit
        echo Updated e! Please restart your terminal and rerun this script to install SplashKit.
    fi
fi

function report_missing_git () {
    if [[ `uname` = Darwin ]]; then
        echo "Developer tools not installed, please run: \"xcode-select --install\" in the terminal and then reinstall."
    elif [[ `uname` = MINGW* ]] || [[ `uname` = MSYS* ]]; then
        echo "Git not found. Please run \"pacman -S git --noconfirm;\" in the terminal and then reinstall"
    elif [[ `uname` = Linux ]]; then
        echo "Please install git using your package manager For example: sudo apt install git"
    fi
    exit 1
}

command -v git >/dev/null 2>&1 || report_missing_git

if [ -d "${INSTALL_PATH}" ]; then
    echo "Deleting old skm"
    rm -rf "${INSTALL_PATH}"
fi

# TODO: Update branch when testing new feature!
git clone --depth 1 --branch new/rewrite-skm-bash $GIT_SKM_REPO "${INSTALL_PATH}"

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

echo "Make sure ${INSTALL_PATH} is on the PATH, and other dev tools and libraries are installed"

echo "SplashKit Successfully installed! Please restart your terminal..."
