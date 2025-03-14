#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

SKM_PATH=`cd "$APP_PATH/.."; pwd`

HAS_PYTHON3=false

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

echo "Attempting to uninstall SplashKit libraries"
echo "This may require root access, please enter your password when prompted."
echo

if [ "$IS_WINDOWS" = true ]; then
    PRIVILEGED=""
else
    PRIVILEGED="sudo"
fi

# Get directories for each OS
echo "Detecting operating system..."
if [ "$SK_OS" = "macos" ]; then
    LIB_FILE="/usr/local/lib/libSplashKit.dylib"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="/usr/local/lib/libSplashKit.so"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"
elif [ "$SK_OS" = "win64" ]; then
    # WIN_OUT_DIR="${WINDIR}/System32"
    LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    LIB_DEST="/mingw64/lib"
    INC_DEST="/mingw64/include"

    # Section below commented out for further testing
    # if [ "$MSYSTEM" = "MINGW64" ]; then
    #     LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    #     LIB_DEST="/mingw64/lib"
    #     INC_DEST="/mingw64/include"
    # elif [ "$MSYSTEM" = "CLANG64" ]; then
    #     LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    #     LIB_DEST="/clang64/lib"
    #     INC_DEST="/clang64/include"
    # elif [ "$MSYSTEM" = "CLANGARM64" ]; then
    #     LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    #     LIB_DEST="/clangarm64/lib"
    #     INC_DEST="/clangarm64/include"
    # fi

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
    # Remove splashkit library file on macos and linux
    echo "Removing splashkit file from "${LIB_DEST}""
    $PRIVILEGED rm -f "$LIB_FILE"
    if [ ! $? -eq 0 ]; then
        echo "Failed to remove SplashKit library from $LIB_DEST"
        exit 1
    fi
fi

# Check if python3 installed
if command -v python3 &> /dev/null; then
    # Check for brew python on macOS
    if [ "$SK_OS" = "macos" ] && ! command -v brew &> /dev/null; then
        HAS_PYTHON3=false
    else
        HAS_PYTHON3=true
    fi
else
    echo "For Python support: Please install python3, then run this script again."
fi

# Get python3 directory for each OS if installed
if [ "$HAS_PYTHON3" = true ]; then
    echo "Detecting python3 version to set global path for removing splashkit.py file"
    
    PYTHON_VERSION=`python3 -c 'import platform; major, minor, patch = platform.python_version_tuple(); print(major + "." + minor);'`

    # Python3 global install path
    if [ "$SK_OS" = "macos" ]; then
        PYTHON_LIB="/opt/homebrew/lib/python${PYTHON_VERSION}/site-packages"
    elif [ "$SK_OS" = "linux" ]; then
        PYTHON_LIB="/usr/lib/python${PYTHON_VERSION}"
    elif [ "$SK_OS" = "win64" ]; then
        PYTHON_LIB="/mingw64/lib/python${PYTHON_VERSION}"

        # Section below commented out for further testing
        # if [ "$MSYSTEM" = "MINGW64" ]; then
        #     PYTHON_LIB="/mingw64/lib/python${PYTHON_VERSION}"
        # elif [ "$MSYSTEM" = "CLANG64" ]; then
        #     PYTHON_LIB="/clang64/lib/python${PYTHON_VERSION}"
        # elif [ "$MSYSTEM" = "CLANGARM64" ]; then
        #     PYTHON_LIB="/clangarm64/lib/python${PYTHON_VERSION}"
        # fi
    fi

    # Remove splashkit python file from global location if python3 is installed
    if [ -f "${PYTHON_LIB}/splashkit.py" ]; then
        echo "Removing splashkit.py from "${PYTHON_LIB}""
        $PRIVILEGED rm -f "${PYTHON_LIB}/splashkit.py"
        if [ ! $? -eq 0 ]; then
            echo "Failed to remove splashkit.py from ${PYTHON_LIB}"
            exit 1
        fi
    fi
fi

if [ "$SK_OS" = "macos" ]; then
    if command -v dotnet &> /dev/null; then
        if [ "$HAS_DOTNET" = true ]; then
            DOTNET_PATH=`sudo find /usr/local -name Microsoft.NETCore.App`
            for f in $DOTNET_PATH/*; do
                if [ -d "$f" ]; then
                    echo "Removing "libSplashKit.dylib" from $f"
                    $PRIVILEGED rm -f "$f/libSplashKit.dylib"
                    if [ ! $? -eq 0 ]; then
                        echo "Failed to remove "libSplashKit.dylib" from $f"
                        exit 1
                    fi
                fi
            done
        fi
    fi
fi

# Remove conf file to link libraries added to /usr/local/lib directory
if [ "$SK_OS" = "linux" ]; then
    echo "Removing splashkit.conf file from /etc/ld.so.conf.d/"
    $PRIVILEGED rm -f "/etc/ld.so.conf.d/splashkit.conf"
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

# Remove splashkit paths from Windows User PATH if using Windows(MSYS2)
if [ $SK_OS = "win64" ]; then
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

    # Update paths in SK_PATHS to replace / with \
    for i in ${!SK_PATHS[@]}; do
        SK_PATH="${SK_PATHS[$i]}"
        SK_PATH="${SK_PATH////\\}"
        SK_PATHS[$i]=$SK_PATH
    done

    # Get Windows path and remove splashkit-added path elements
    ORIGINAL_WIN_PATH=`powershell.exe -Command "([System.Environment]::GetEnvironmentVariable('PATH','User').Split(';') | Where-Object { (\\$_ -ne '${SK_PATHS[0]}' -and \\$_ -ne '${SK_PATHS[1]}' -and \\$_ -ne '${SK_PATHS[2]}') }) -join ';'"`

    # Set updated Windows path
    powershell.exe -Command "[System.Environment]::SetEnvironmentVariable('PATH',\"$ORIGINAL_WIN_PATH\",'User')"
fi

# Remove .splashkit folder
echo "Removing .splashkit folder from "${INSTALL_PATH}""
rm -rf ${INSTALL_PATH}