#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)

SKM_PATH=$(cd "$APP_PATH/../.." && pwd)

HAS_PYTHON3=false
HAS_DOTNET=false

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

echo
echo "Installing SplashKit library in default global locations..."
echo "This may require root access, please enter your password when prompted."
echo

if [ "$IS_WINDOWS" = true ]; then
    PRIVILEGED=""
else
    PRIVILEGED="sudo"
fi

echo "Detecting operating system for global paths..."

if [ "$SK_OS" = "macos" ]; then
    LIB_FILE_SRC="${SKM_PATH}/lib/macos/libSplashKit.dylib"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"
    LIB_FILE_DEST="${LIB_DEST}/libSplashKit.dylib"
    LIB_FILE_DEST_LOWER="${LIB_DEST}/libsplashkit.dylib"
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE_SRC="${SKM_PATH}/lib/linux/libSplashKit.so"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"
    LIB_FILE_DEST="${LIB_DEST}/libSplashKit.so"
    LIB_FILE_DEST_LOWER="${LIB_DEST}/libsplashkit.so"
elif [ "$SK_OS" = "win64" ]; then
    LIB_FILE_SRC="${SKM_PATH}/lib/win64/SplashKit.dll"
    if [[ $(uname) == *ARM64 ]]; then
        LIB_DEST="/clangarm64/lib"
        INC_DEST="/clangarm64/include"
    else
        LIB_DEST="/mingw64/lib"
        INC_DEST="/mingw64/include"
    fi
    LIB_FILE_DEST="${LIB_DEST}/SplashKit.dll"
    LIB_FILE_DEST_LOWER="${LIB_DEST}/splashkit.dll"
else
    echo "Unable to detect operating system..."
    exit 1
fi

# --------------------------------------------
# Copying Splashkit files to global locations
# --------------------------------------------

if [ ! -d "${LIB_DEST}" ]; then
    echo "Creating directory ${LIB_DEST}"
    $PRIVILEGED mkdir -p "${LIB_DEST}"
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory ${LIB_DEST}"
        exit 1
    fi
fi

if [ ! -d "${INC_DEST}/splashkit" ]; then
    echo "Creating directory ${INC_DEST}/splashkit"
    $PRIVILEGED mkdir -p "${INC_DEST}/splashkit"
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory ${INC_DEST}/splashkit"
        exit 1
    fi
fi

# Check if library source file exists
if [ ! -f "${LIB_FILE_SRC}" ]; then
    echo "Building library..."
    if [ "$SK_OS" = "macos" ]; then
        "${INSTALL_PATH}/macos/install/install.sh"
    elif [ "$SK_OS" = "linux" ]; then
        "${SKM_PATH}/linux/install/install.sh"
    elif [ "$SK_OS" = "win64" ]; then
        "${SKM_PATH}/windows/install/install.sh"
    else
        echo "Unable to detect operating system..."
        exit 1
    fi
fi

if [ "$SK_OS" = "linux" ]; then
    # Create symbolic link to library
    echo "Linking library files into ${LIB_DEST}"
    if [ ! -f "$LIB_FILE_DEST" ]; then
        # Link library file if not already linked
        $PRIVILEGED ln -s "$LIB_FILE_SRC" "$LIB_FILE_DEST"
        if [ ! $? -eq 0 ]; then
            echo "Failed to create symbolic link to $LIB_FILE_DEST"
            exit 1
        fi
    fi
    # Link lowercase version if not already linked
    if [ ! -f "${LIB_FILE_DEST_LOWER}" ]; then
        $PRIVILEGED ln -s "${LIB_FILE_SRC}" "$LIB_FILE_DEST_LOWER"
        if [ ! $? -eq 0 ]; then
            echo "Failed to create symbolic link to $LIB_FILE_DEST_LOWER"
            exit 1
        fi
    fi
else
    # Copy files
    echo "Copying library file to ${LIB_DEST}"
    $PRIVILEGED cp -f "$LIB_FILE_SRC" "$LIB_DEST"
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit library to $LIB_DEST"
        exit 1
    fi
fi

# # We cant install but it should be on the path anyway...
# # If $WIN_OUT_DIR is set, we are on Windows and need to copy the dll to the System32 or System64 directory
# if [ ! -z "$WIN_OUT_DIR" ]; then
#     $PRIVILEGED cp "$LIB_FILE_SRC" "$WIN_OUT_DIR"
#     if [ ! $? -eq 0 ]; then
#         echo "Failed to copy SplashKit library to $WIN_OUT_DIR"
#         exit 1
#     fi
# fi

echo "Copying header files to ${INC_DEST}/splashkit"
$PRIVILEGED cp "${SKM_PATH}/clang++/include/"* "${INC_DEST}/splashkit"
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit C++ headers to ${INC_DEST}/splashkit"
    exit 1
fi

$PRIVILEGED cp "${APP_PATH}"/splash*.h ${INC_DEST}
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit header to ${INC_DEST}"
    exit 1
fi

# --------------------------------
# Language-specific installations
# --------------------------------

# Copy library file to dotnet runtime folders
if [ "$SK_OS" = "macos" ]; then
    if command -v dotnet &>/dev/null; then
        HAS_DOTNET=true
    else
        echo "For C# support: Please install the .NET SDK, then run \"skm global install\" again."
    fi

    if [ "$HAS_DOTNET" = true ]; then
        echo "Detecting dotnet version(s) to set global path..."

        DOTNET_TARGET="Microsoft.NETCore.App"

        # Read the output of "dotnet --list-runtimes" command, line by line
        while IFS= read -r line; do
            if [[ "$line" == *"$DOTNET_TARGET"* ]]; then
                DOTNET_VERSION=$(echo "$line" | awk '{print $2}')
                DOTNET_PATH=$(echo "$line" | sed 's/^.*\[//' | sed 's/]$//')
                DOTNET_PATH="$DOTNET_PATH/$DOTNET_VERSION"

                if [ -d "$DOTNET_PATH" ]; then
                    echo "Copying library file to $DOTNET_PATH"
                    $PRIVILEGED cp -f "$LIB_FILE_SRC" "$DOTNET_PATH"
                    if [ ! $? -eq 0 ]; then
                        echo "Failed to copy "$LIB_FILE_SRC" to $DOTNET_PATH"
                        exit 1
                    fi
                fi
            fi
        done < <(dotnet --list-runtimes)

        # DOTNET_PATH=`$PRIVILEGED find /usr/local -name Microsoft.NETCore.App`
        # if [ ! -d "$DOTNET_PATH" ]; then
        #     DOTNET_PATH=`$PRIVILEGED find /opt/homebrew -name Microsoft.NETCore.App`
        # if [ ! -d "$DOTNET_PATH" ]; then
        #     DOTNET_PATH=`$PRIVILEGED find ~/.dotnet -name Microsoft.NETCore.App`
        # fi
        # fi

        # for f in $DOTNET_PATH/*; do
        #     if [ -d "$f" ]; then
        #         echo "Copying "$LIB_FILE_SRC" to $f"
        #         $PRIVILEGED cp -f "$LIB_FILE_SRC" "$f"
        #         if [ ! $? -eq 0 ]; then
        #             echo "Failed to copy "$LIB_FILE_SRC" to $f"
        #             exit 1
        #         fi
        #     fi
        # done
    fi
fi

# Check if python3 installed
if command -v python3 &>/dev/null; then
    # Check for brew python on macOS
    if [ "$SK_OS" = "macos" ] && ! command -v brew &>/dev/null && ! command -v conda &>/dev/null; then
        HAS_PYTHON3=false
        echo "For Python support: Please install python3 using Homebrew or Anaconda3, then run \"skm global install\" again."
    else
        HAS_PYTHON3=true
    fi
else
    echo "For Python support: Please install python3, then run \"skm global install\" again."
fi

# Get python3 directory for each OS if installed
if [ "$HAS_PYTHON3" = true ]; then
    echo "Detecting python3 version to set global path..."
    PYTHON_VERSION=$(python3 -c 'import platform; major, minor, patch = platform.python_version_tuple(); print(major + "." + minor);')

    # Python3 global install path
    if [ "$SK_OS" = "macos" ]; then
        if [ "$(which python3)" = "/opt/homebrew/bin/python3" ]; then
            PYTHON_LIB="/opt/homebrew/lib/python${PYTHON_VERSION}/site-packages"
        elif [ "$(which python3)" = "/opt/anaconda3/bin/python3" ]; then
            PYTHON_LIB="/opt/anaconda3/lib/python${PYTHON_VERSION}/site-packages"
        fi
    elif [ "$SK_OS" = "linux" ]; then
        PYTHON_LIB="/usr/lib/python${PYTHON_VERSION}"
    elif [ "$SK_OS" = "win64" ]; then
        if [[ $(uname) == *ARM64 ]]; then
            PYTHON_LIB="/clangarm64/lib/python${PYTHON_VERSION}"
        else
            PYTHON_LIB="/mingw64/lib/python${PYTHON_VERSION}"
        fi
    fi

    # Copy splashkit python file to global location
    if [ -d "$PYTHON_LIB" ]; then
        echo "Copying splashkit.py to "${PYTHON_LIB}""
        $PRIVILEGED cp "${SKM_PATH}/python3/splashkit.py" "${PYTHON_LIB}"
        if [ ! $? -eq 0 ]; then
            echo "Failed to copy splashkit.py to ${PYTHON_LIB}"
            exit 1
        fi
    fi
fi

# ------------------
# Updating the Path
# ------------------

if [ "$SK_OS" = "linux" ]; then
    echo "Updating library config cache"
    # Add /usr/local/lib to ld search paths using conf file
    touch "${SKM_PATH}/linux/splashkit.conf"
    echo "/usr/local/lib" >>"${SKM_PATH}/linux/splashkit.conf"
    $PRIVILEGED mv "${SKM_PATH}/linux/splashkit.conf" "/etc/ld.so.conf.d/splashkit.conf"
    $PRIVILEGED ldconfig
elif [ "$SK_OS" = "macos" ]; then
    echo "Setting library location..."
    $PRIVILEGED install_name_tool -id /usr/local/lib/libSplashKit.dylib /usr/local/lib/libSplashKit.dylib
elif [ "$SK_OS" = "win64" ]; then
    export PATH="$LIB_DEST:$PATH"
fi

# ---------------------
# Installation testing
# ---------------------

echo "Testing install..."

if command -v clang++ &>/dev/null; then
    COMPILER_EXE=clang++
elif command -v g++ &>/dev/null; then
    COMPILER_EXE=g++
else
    echo "No C/C++ compiler found, skipping test"
    exit 0
fi

${COMPILER_EXE} "${APP_PATH}/test.cpp" -l SplashKit -o "${APP_PATH}/test"
if [ ! $? -eq 0 ]; then
    echo "Failed to compile test program"
    exit 1
fi

echo
"${APP_PATH}/test"
if [ ! $? -eq 0 ]; then
    echo "Failed to run test program"
    exit 1
fi

rm "${APP_PATH}/test"

echo #"Done"
