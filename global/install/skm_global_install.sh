#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

HAS_PYTHON3=false

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

echo "Attempting to install the SplashKit C++ libraries to /usr/local/lib/splashkit"
echo "This may require root access, please enter your password when prompted."
echo

if [ "$IS_WINDOWS" = true ]; then
    PRIVILEGED=""
else
    PRIVILEGED="sudo"
fi

echo "Detecting operating system"

if [ "$SK_OS" = "macos" ]; then
    LIB_FILE="${SKM_PATH}/lib/macos/libSplashKit.dylib"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"

    # Check if python3 installed on macOS
    if command -v python3 &> /dev/null && command -v brew &> /dev/null; then
        HAS_PYTHON3=true
    else
        echo "For Python support: Please install python3 using brew (Homebrew), then run this script again."
    fi
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="${SKM_PATH}/lib/linux/libSplashKit.so"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"

    # Check if python3 installed on Linux/WSL
    if command -v python3 &> /dev/null; then
        HAS_PYTHON3=true
    else
        echo "For Python support: Please install python3, then run this script again."
    fi
elif [ "$SK_OS" = "win64" ]; then
    # WIN_OUT_DIR="${WINDIR}/System32"
    if [ "$MSYSTEM" = "MINGW64" ]; then
        LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
        LIB_DEST="/mingw64/lib"
        INC_DEST="/mingw64/include"
    elif [ "$MSYSTEM" = "CLANG64" ]; then
        LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
        LIB_DEST="/clang64/lib"
        INC_DEST="/clang64/include"
    # elif [ "$MSYSTEM" = "CLANGARM64" ]; then
    #     LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    #     LIB_DEST="/clangarm64/lib"
    #     INC_DEST="/clangarm64/include"
    fi

    # Check if python3 installed on Windows
    if command -v python3 &> /dev/null; then
        HAS_PYTHON3=true
    else
        echo "For Python support: Please install python3, then run this script again."
    fi
else
    echo "Unable to detect operating system..."
    exit 1
fi

# Get python3 directory for each OS if installed
if [ "$HAS_PYTHON3" = true ]; then
    PYTHON_VERSION=`python3 -c 'import platform; major, minor, patch = platform.python_version_tuple(); print(major + "." + minor);'`
    
    echo "Detecting python3 version to set global path"

    if [ "$SK_OS" = "macos" ]; then
        # macOS Python3 global install path
        PYTHON_LIB="/opt/homebrew/lib/python${PYTHON_VERSION}/site-packages"
    elif [ "$SK_OS" = "linux" ]; then
        # Linux/WSL Python3 global install path
        PYTHON_LIB="/usr/lib/python${PYTHON_VERSION}"
    elif [ "$SK_OS" = "win64" ]; then
        # Windows Python3 global install path
        if [ "$MSYSTEM" = "MINGW64" ]; then
            PYTHON_LIB="/mingw64/lib/python${PYTHON_VERSION}"
        elif [ "$MSYSTEM" = "CLANG64" ]; then
            PYTHON_LIB="/clang64/lib/python${PYTHON_VERSION}"
        # elif [ "$MSYSTEM" = "CLANGARM64" ]; then
        #     PYTHON_LIB="/clangarm64/lib/python${PYTHON_VERSION}"
        fi
    fi
fi

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

echo "Copying files to "${LIB_DEST}""
$PRIVILEGED cp -f "$LIB_FILE" "$LIB_DEST"
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit library to $LIB_DEST"
    exit 1
fi

echo "Copying files to ${INC_DEST}/splashkit"
$PRIVILEGED cp "${SKM_PATH}/clang++/include/"* "${INC_DEST}/splashkit"
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit C++ headers to ${INC_DEST}/splashkit"
    exit 1
fi

$PRIVILEGED cp "${APP_PATH}/splashkit.h" ${INC_DEST}
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit header to ${INC_DEST}"
    exit 1
fi

# Copy splashkit python file to global location
if [ "$HAS_PYTHON3" = true ]; then
    echo "Copying splashkit.py to "${PYTHON_LIB}""
    $PRIVILEGED cp "${SKM_PATH}/python3/splashkit.py" "${PYTHON_LIB}"
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy splashkit.py to ${PYTHON_LIB}"
        exit 1
    fi
fi

# We cant install but it should be on the path anyway...
#
# # If $WIN_OUT_DIR is set, we are on Windows and need to copy the dll to the System32 or System64 directory
# if [ ! -z "$WIN_OUT_DIR" ]; then
#     $PRIVILEGED cp "$LIB_FILE" "$WIN_OUT_DIR"
#     if [ ! $? -eq 0 ]; then
#         echo "Failed to copy SplashKit library to $WIN_OUT_DIR"
#         exit 1
#     fi
# fi

if [ "$SK_OS" = "linux" ]; then
    echo "Updating library config cache"
    # Add /usr/local/lib to ld search paths using conf file
    touch "${SKM_PATH}/linux/splashkit.conf"
    echo "/usr/local/lib" >> "${SKM_PATH}/linux/splashkit.conf"
    $PRIVILEGED cp -n "${SKM_PATH}/linux/splashkit.conf /etc/ld.so.conf.d/splashkit.conf"
    rm "${SKM_PATH}/linux/splashkit.conf"
    $PRIVILEGED ldconfig
elif [ "$SK_OS" = "macos" ]; then
    echo "Setting library location"
    sudo install_name_tool -id /usr/local/lib/libSplashKit.dylib /usr/local/lib/libSplashKit.dylib
fi

echo "Testing install"

if command -v clang++ &> /dev/null; then
    COMPILER_EXE=clang++
elif command -v g++ &> /dev/null; then
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

"${APP_PATH}/test"
if [ ! $? -eq 0 ]; then
    echo "Failed to run test program"
    exit 1
fi

rm "${APP_PATH}/test"

echo "Done"
