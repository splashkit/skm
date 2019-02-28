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
    echo "Looks like you already have splashkit!"
    echo "To uninstall run \"rm -rf ${INSTALL_PATH}\""
    exit 1
fi

git clone --depth 1 --branch master $GIT_SKM_REPO "${INSTALL_PATH}"

# Add SKM app to path without needing sudo
if [ -f ~/.bash_profile ]; then
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bash_profile
fi

if [ -f ~/.bashrc ]; then
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bashrc
fi

if [ -f ~/.zshrc ]; then
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.zshrc
fi

export PATH="$INSTALL_PATH:$PATH"

if [[ `uname` = MINGW32* ]]; then
    #Export to path -- for current terminal
    export PATH="$HOME/.splashkit/lib:$PATH"
    export PATH="$HOME/.splashkit/lib/win32:$PATH"
    export PATH="$HOME/.splashkit:$PATH"

    #Export path for new terminals
    export ORIGINAL_PATH="$HOME/.splashkit/lib:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit/lib/win32:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit:$ORIGINAL_PATH"

    # Set path
    setx PATH "$ORIGINAL_PATH"
fi

if [[ `uname` = MINGW64* ]] || [[ `uname` = MSYS* ]]; then
    #Export to path -- for current terminal
    export PATH="$HOME/.splashkit/lib:$PATH"
    export PATH="$HOME/.splashkit/lib/win64:$PATH"
    export PATH="$HOME/.splashkit:$PATH"

    #Export path for new terminals
    export ORIGINAL_PATH="$HOME/.splashkit/lib:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit/lib/win64:$ORIGINAL_PATH"
    export ORIGINAL_PATH="$HOME/.splashkit:$ORIGINAL_PATH"

    # Set path
    setx PATH "$ORIGINAL_PATH"
fi

if [[ `uname` = Linux ]]; then
    # Add SKM app to path without needing sudo
    echo "SKM and SplashKit depends on the following libraries:
    * sdl2 development library
    * sdl2 gfx development library
    * sdl2 mixer development library
    * sdl2 ttf development library
    * sdl2 net development library
    * sdl2 image development library
    * ncurses development library
    * png development library
    * curl4 openssl development library
    * bz2 development library
    * flac, libvorbis libmikmod development libraries
    * webp development library
    * freetype development library
    * CMAKE

    Please ensure these dependencies are installed using your package manager.
    For example:

    sudo apt install libsdl2-dev libsdl2-gfx-dev libsdl2-mixer-dev libsdl2-ttf-dev libsdl2-net-dev libsdl2-image-dev libncurses-dev libpng-dev libcurl4-openssl-dev libbz2-dev libflac-dev libvorbis-dev libmikmod-dev libwebp-dev libfreetype6-dev cmake

"
fi

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

echo "SplashKit Successfully installed! Please restart your terminal..."
