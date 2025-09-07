#!/bin/bash

GIT_SKM_REPO=https://github.com/splashkit/skm.git

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

# For MSYS2 users: Check if MINGW64 shell is being used
if [[ $(uname) != MINGW64* ]] && [[ $(uname) != Linux ]] && [[ $(uname) != Darwin ]]; then
    SHELL_NAME=$(uname | cut -d '_' -f1,1)
    echo
    echo "The $SHELL_NAME terminal is not supported."
    echo "Please use the \"MINGW64\" terminal for coding with SplashKit..."
    echo
    read -p "Would you like to install SplashKit in the MINGW64 terminal now? (Y/N): " -n 1 -r </dev/tty
    echo ""
    if [[ $REPLY =~ [Yy]$ ]]; then
        # Run install command in MINGW64 terminal
        MINGW64_PATH=$(
            cd "$APP_PATH/../../.."
            pwd -W
        )
        start "" $MINGW64_PATH/mingw64.exe bash -c "bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/skm-install.sh); bash"
        exit 0
    else
        echo
        echo To start coding with SplashKit:
        echo
        echo "  1. Open the MINGW64 terminal."
        echo "  2. Copy/paste the following command to install SplashKit: bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/skm-install.sh)"
        echo
        exit 1
    fi
fi

function report_missing_git() {
    if [[ $(uname) = Darwin ]]; then
        echo "Developer tools not installed, please run: \"xcode-select --install\" in the terminal and then reinstall."
    elif [[ $(uname) = MINGW64* ]]; then
        echo "Git not found. Please run \"pacman -S git --noconfirm;\" in the terminal and then reinstall"
    elif [[ $(uname) = Linux ]]; then
        echo "Please install git using your package manager For example: sudo apt install git"
    fi
    exit 1
}

command -v git >/dev/null 2>&1 || report_missing_git

if [ -d "${INSTALL_PATH}" ]; then
    echo "Looks like you already have splashkit!"
    if command -v skm &>/dev/null; then
        echo "To uninstall run \"skm uninstall\""
    else
        echo "To uninstall run \"rm -rf ${INSTALL_PATH}\""
    fi
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

if [ "$IS_WINDOWS" = true ]; then
    PRIVILEGED=""
else
    PRIVILEGED="sudo"
fi

# Add SKM app to path
# Add to .bashrc if using bash
if [[ ${SHELL} = "/bin/bash" ]] || [ ${SHELL} = "/usr/bin/bash" -a $(uname) = Linux ]; then

    echo "Adding SplashKit to the PATH..."
    # Check for read and write permissions
    if [ $(stat -c %A ~/.bashrc | cut -c2) != "r" ] && [ $(stat -c %A ~/.bashrc | cut -c3) != "w" ]; then
        echo "Updating .bashrc permissions, please enter your password when prompted."
        $PRIVILEGED chmod u+rw,a-x ~/.bashrc
    fi
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >>~/.bashrc
    source ~/.bashrc
fi

# Add to .zshrc if using zsh
if [[ ${SHELL} = "/bin/zsh" ]] || [[ ${SHELL} = "/usr/bin/zsh" ]]; then
    echo "Adding SplashKit to the PATH..."
    # Check for read and write permissions
    if [[ $(uname) = Darwin ]]; then
        # macos version of stat command
        if [ $(stat -F ~/.zshrc | cut -c2) != "r" ] && [ $(stat -F ~/.zshrc | cut -c3) != "w" ]; then
            echo "Updating .zshrc permissions, please enter your password when prompted."
            $PRIVILEGED chmod u+rw,a-x ~/.zshrc
        fi
    else
        # linux version of stat command
        if [ $(stat -c %A ~/.zshrc | cut -c2) != "r" ] && [ $(stat -c %A ~/.zshrc | cut -c3) != "w" ]; then
            echo "Updating .zshrc permissions, please enter your password when prompted."
            $PRIVILEGED chmod u+rw,a-x ~/.zshrc
        fi
    fi
    echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >>~/.zshrc
    source ~/.zshrc
fi

export PATH="$INSTALL_PATH:$PATH"

if [[ $(uname) = MINGW64* ]]; then
    SHELL_PATH="/mingw64/bin"
    # List of PATHS added in splashkit install
    SK_PATHS=("$(cd $SHELL_PATH && pwd -W)" "$(cd ~/.splashkit && pwd -W)" "$(cd ~/.splashkit/lib/win64 && pwd -W)")

    # Get Windows path and remove splashkit-added path elements
    ORIGINAL_WIN_PATH=$(/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe -Command "[System.Environment]::GetEnvironmentVariable('PATH','User')")

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
    /c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe -Command "[System.Environment]::SetEnvironmentVariable('PATH',\"$NEW_WIN_PATH\",'User')"

    if [[ $MSYS2_PATH_TYPE != 'inherit' ]]; then
        /c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe -Command "[System.Environment]::SetEnvironmentVariable('MSYS2_PATH_TYPE',\"inherit\",'User')"
        # echo Updated! Please restart your terminal and rerun this script to install SplashKit.
    fi
fi

if ! command -v skm &>/dev/null; then
    echo "\"skm\" command not found..."
    echo "Open a new terminal window and run the command "skm" to check if SplashKit has been added to the PATH successfully."
    echo "If the "skm" is found, please run the following commands in the new terminal window to complete the SplashKit installation:"
    if [[ $(uname) = MINGW64* ]]; then
        echo "  skm windows install"
    elif [[ $(uname) = Linux ]]; then
        echo "  skm linux install"
    elif [[ $(uname) = Darwin ]]; then
        OSX_VERSION=$(sw_vers -productVersion)
        if ! awk "BEGIN{ exit ($OSX_VERSION < 12.3) }"; then
            echo "  skm macos install"
        else
            echo "  skm global install"
        fi
    fi
    echo
    echo "If the "skm" command is not found in the new terminal window, please follow the troubleshooting steps in the link below:"
    if [[ $(uname) = MINGW64* ]]; then
        echo "https://splashkit.io/troubleshoot/windows-msys2/issue-2-skm-not-found"
    elif [[ $(uname) = Linux ]]; then
        echo "https://splashkit.io/troubleshoot/linux/issue-2-skm-not-found"
    elif [[ $(uname) = Darwin ]]; then
        echo "https://splashkit.io/troubleshoot/macos/issue-3-skm-not-found"
    fi
    exit 1
fi

# if [[ `uname` = Linux ]]; then
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

# fi

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

# Run the next install step
if [[ $(uname) = MINGW64* ]]; then
    "${INSTALL_PATH}/windows/install/install.sh"
elif [[ $(uname) = Linux ]]; then
    "${INSTALL_PATH}/linux/install/install.sh"
elif [[ $(uname) = Darwin ]]; then
    OSX_VERSION=$(sw_vers -productVersion)
    if ! awk "BEGIN{ exit ($OSX_VERSION < 12.3) }"; then
        "${INSTALL_PATH}/macos/install/install.sh"
    else
        # Install brew packages
        "${INSTALL_PATH}/macos/install/install_deps.sh"
        # Then do global install
        "${INSTALL_PATH}/global/install/skm_global_install.sh"
    fi
fi

echo "SplashKit Successfully installed! Please restart your terminal..."
