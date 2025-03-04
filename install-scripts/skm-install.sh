#!/bin/bash

GIT_SKM_REPO=https://github.com/splashkit/skm.git

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

if [[ `uname` = MINGW32* ]]; then
    echo MinGW32 is no longer supported. Please install using the MinGW64 terminal
    exit 1
fi

if [[ `uname` = MINGW* ]] || [[ `uname` = MSYS* ]]; then
    if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then
        setx MSYS2_PATH_TYPE inherit
        echo Updated! Please restart your terminal and rerun this script to install SplashKit.
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

BRANCH_NAME="master" #default

# Check if branch name arg given (1st arg)
if [ "$1" ]; then
    BRANCH_NAME=$1
fi

# Check if git repo URL given (2nd arg)
if [ "$2" ]; then
    GIT_SKM_REPO=$2
fi

# Output arg details if testing
if [[ $BRANCH_NAME != "master" ]] || [[ $GIT_SKM_REPO != "https://github.com/splashkit/skm.git" ]]; then
    echo "SplashKit installation testing.."
    echo "Installing from $GIT_SKM_REPO"
    echo "Using $BRANCH_NAME branch."
fi

git clone --depth 1 --branch $BRANCH_NAME $GIT_SKM_REPO "${INSTALL_PATH}"

# Add SKM app to path without needing sudo

# Add to .bashrc if using bash
if [[ ${SHELL} = "/bin/bash" ]] || [ ${SHELL} = "/usr/bin/bash" -a `uname` = Linux ] ; then
    chmod +x ~/.bash_profile
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bash_profile
    chmod +x ~/.bashrc
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bashrc
fi

# Add to .zshrc if using zsh
if [[ ${SHELL} = "/bin/zsh" ]] || [[ ${SHELL} = "/usr/bin/zsh" ]]; then
    chmod +x ~/.zshrc
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.zshrc
fi

export PATH="$INSTALL_PATH:$PATH"

if [[ `uname` = MINGW64* ]] || [[ `uname` = MSYS* ]]; then
    SHELL_PATH="/mingw64/bin"

    # Section below commented out for further testing
    # Detect MSYS2 shell
    # SHELL_PATH=""
    # if [ "$MSYSTEM" = "MINGW64" ]; then
    #     SHELL_PATH="/mingw64/bin"
    # elif [ "$MSYSTEM" = "CLANG64" ]; then
    #     SHELL_PATH="/clang64/bin"
    # elif [ "$MSYSTEM" = "CLANGARM64" ]; then
    #     SHELL_PATH="/clangarm64/bin"
    # fi
    
    # List of PATHS added in splashkit install
    SK_PATHS=("`cd $SHELL_PATH; pwd -W`" "`cd ~/.splashkit; pwd -W`" "`cd ~/.splashkit/lib/win64; pwd -W`")
    
    # Get Windows path and remove splashkit-added path elements
    ORIGINAL_WIN_PATH=`powershell.exe -Command "[System.Environment]::GetEnvironmentVariable('PATH','User')"`

    # Create string for splashKit paths
    SK_PATHS_STR=""
    for i in ${!SK_PATHS[@]}; do
        SK_PATH="${SK_PATHS[$i]}"
        SK_PATH="${SK_PATH////\\}"
        SK_PATHS_STR+="$SK_PATH;"
    done

    # Create string for new windows path with splashkit paths added
    NEW_WIN_PATH="$SK_PATHS_STR$ORIGINAL_WIN_PATH"

    # Set updated Windows path
    powershell.exe -Command "[System.Environment]::SetEnvironmentVariable('PATH',\"$NEW_WIN_PATH\",'User')"

    echo "Run the following to install the required pacman packages for splashkit:"
    echo "${bold}skm windows install${normal}"
fi

if [[ `uname` = Linux ]]; then
    #     # Add SKM app to path without needing sudo
    #     echo "SKM and SplashKit depends on the following libraries:
    #     * sdl2 development library
    #     * sdl2 gfx development library
    #     * sdl2 mixer development library
    #     * sdl2 ttf development library
    #     * sdl2 net development library
    #     * sdl2 image development library
    #     * ncurses development library
    #     * png development library
    #     * curl4 openssl development library
    #     * bz2 development library
    #     * flac, libvorbis libmikmod development libraries
    #     * webp development library
    #     * freetype development library
    #     * CMAKE

    #     Please ensure these dependencies are installed using your package manager.
    #     For example:

    #     sudo apt install libsdl2-dev libsdl2-gfx-dev libsdl2-mixer-dev libsdl2-ttf-dev libsdl2-net-dev libsdl2-image-dev libncurses-dev libpng-dev libcurl4-openssl-dev libbz2-dev libflac-dev libvorbis-dev libmikmod-dev libwebp-dev libfreetype6-dev cmake

    # "

    echo "Run the following to install the necessary native libraries, compile and install splashkit:"
    echo "${bold}skm linux install${normal}"
fi

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

echo "SplashKit Successfully installed! Please restart your terminal..."
