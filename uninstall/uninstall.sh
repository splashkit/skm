#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

echo "Attempting to uninstall SplashKit libraries"
echo "This may require root access, please enter your password when prompted."
echo

if [ "$IS_WINDOWS" = true ]; then
    PRIVILEGED=""
else
    PRIVILEGED="sudo"
fi

PYTHON_VERSION=`python3 -c 'import platform; major, minor, patch = platform.python_version_tuple(); print(major + "." + minor);'`

# Get directories for each OS
echo "Detecting operating system..."
if [ "$SK_OS" = "macos" ]; then
    LIB_FILE="/usr/local/lib/libSplashKit.dylib"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"

    # macOS Python3 global install
    if command -v python3 &> /dev/null && command -v brew &> /dev/null; then
        PYTHON_LIB="/opt/homebrew/lib/python${PYTHON_VERSION}/site-packages"
    fi

elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="/usr/local/lib/libSplashKit.so"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"

    # Linux/WSL Python3 global install
    if command -v python3 &> /dev/null; then
        PYTHON_LIB="/usr/lib/python${PYTHON_VERSION}"
    fi

elif [ "$SK_OS" = "win64" ]; then
    LIB_FILE="/mingw64/lib/SplashKit.dll"
    LIB_SRC="${SKM_PATH}/lib/win64"
    LIB_DEST="/mingw64/lib"
    INC_DEST="/mingw64/include"

    # Windows (mingw64) Python3 global install
    if command -v python3 &> /dev/null; then
        PYTHON_LIB="/mingw64/lib/python${PYTHON_VERSION}"
    fi

else
    echo "Unable to detect operating system..."
    exit 1
fi

# Remove global library include files in splashkit folder (and folder itself)
if [ -d "${INC_DEST}/splashkit" ]; then
    echo "Remove directory ${INC_DEST}/splashkit"
    $PRIVILEGED rm -rf "${INC_DEST}/splashkit"
    if [ ! $? -eq 0 ]; then
        echo "Failed to remove directory ${INC_DEST}/splashkit"
        exit 1
    fi
fi

# Remove splashkit header file
if [ -f "${INC_DEST}/splashkit.h" ]; then
    echo "Removing splashkit header file from "${LIB_DEST}""
    $PRIVILEGED rm -f "${INC_DEST}/splashkit.h"
    if [ ! $? -eq 0 ]; then
        echo "Failed to remove SplashKit header from ${INC_DEST}"
        exit 1
    fi
fi

# Remove splashkit library file
if [ -f "${LIB_FILE}" ]; then
    if [ "$SK_OS" = "win64" ]; then
        # Remove all splashkit related library files for mingw/msys2
        echo "Removing splashkit files from "${LIB_DEST}""
        cd $LIB_SRC
        find . -type f -exec rm -rf $LIB_DEST/{} \;
        if [ ! $? -eq 0 ]; then
            echo "Failed to remove SplashKit library files from $LIB_DEST"
            exit 1
        fi
    else
        # Remove splashkit library file on macos and linux
        echo "Removing splashkit file from "${LIB_DEST}""
        $PRIVILEGED rm -f "$LIB_FILE"
        if [ ! $? -eq 0 ]; then
            echo "Failed to remove SplashKit library from $LIB_DEST"
            exit 1
        fi
    fi
fi

# Remove splashkit python file from global location if python3 is installed
if [ -f "${PYTHON_LIB}/splashkit.py" ]; then
    echo "Removing splashkit.py to "${PYTHON_LIB}""
    $PRIVILEGED rm -f "${PYTHON_LIB}/splashkit.py"
    if [ ! $? -eq 0 ]; then
        echo "Failed to remove splashkit.py from ${PYTHON_LIB}"
        exit 1
    fi
fi

bold=$(tput bold)
normal=$(tput sgr0)

# Remove splashkit path from .bashrc if using bash
if [[ ${SHELL} = "/bin/bash" ]] || [ ${SHELL} = "/usr/bin/bash" -a `uname` = Linux ] ; then
    echo "Removing ${bold}export PATH=\"$INSTALL_PATH:\$PATH\"${normal} from ~/.bashrc"
    if [ $SK_OS = "macos" ]; then
        sed -i '' "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.bashrc
    else
        sed -i "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.bashrc
    fi
    echo "Removing ${bold}export PATH=\"$INSTALL_PATH:\$PATH\"${normal} from ~/.bash_profile"
    if [ $SK_OS = "macos" ]; then
        sed -i '' "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.bash_profile
    else
        sed -i "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.bash_profile
    fi
fi

# Remove splashkit path from .zshrc if using zsh
if [[ ${SHELL} = "/bin/zsh" ]] || [[ ${SHELL} = "/usr/bin/zsh" ]]; then
    echo "Removing ${bold}export PATH=\"$INSTALL_PATH:\$PATH\"${normal} from ~/.zshrc"
    if [ $SK_OS = "macos" ]; then
        sed -i '' "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.zshrc
    else
        sed -i "\|export PATH=\"$INSTALL_PATH:\$PATH\"|d" ~/.zshrc
    fi
fi

# Remove .splashkit folder
echo "Removing .splashkit folder from "${INSTALL_PATH}""
rm -rf ${INSTALL_PATH}