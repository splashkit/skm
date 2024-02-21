#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

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
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="${SKM_PATH}/lib/linux/libSplashKit.so"
    LIB_DEST="/usr/local/lib"
    INC_DEST="/usr/local/include"
elif [ "$SK_OS" = "win32" ]; then
    LIB_FILE="${SKM_PATH}/lib/win32/SplashKit.dll"
    WIN_OUT_DIR="${WINDIR}/System32"
    LIB_DEST="/mingw32/lib"
    INC_DEST="/mingw32/include"
elif [ "$SK_OS" = "win64" ]; then
    LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    WIN_OUT_DIR="${WINDIR}/System32"
    LIB_DEST="/mingw64/lib"
    INC_DEST="/mingw64/include"
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [ ! -d "${DEST_LIB}" ]; then
    echo "Creating directory ${DEST_LIB}"
    $PRIVILEGED mkdir -p "${DEST_LIB}"
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory ${DEST_LIB}"
        exit 1
    fi
fi

if [ ! -d "${DEST_INC}/splashkit" ]; then
    echo "Creating directory ${DEST_INC}/splashkit"
    $PRIVILEGED mkdir -p ${DEST_INC}/splashkit
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory ${DEST_INC}/splashkit"
        exit 1
    fi
fi

echo "Copying files to "${DEST_LIB}""

$PRIVILEGED cp -f "$LIB_FILE" "$LIB_DEST"
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit library to $LIB_DEST"
    exit 1
fi

echo "Copying files to ${INC_DEST}/splashkit"
$PRIVILEGED cp "${SKM_PATH}/clang++/include/"* "${INC_DEST}"
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit C++ headers to ${INC_DEST}"
    exit 1
fi

$PRIVILEGED cp "${APP_PATH}/splashkit.h" ${INC_DEST}
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit header to ${INC_DEST}"
    exit 1
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
    $PRIVILEGED ldconfig
fi

echo "Testing install"

if [ "$SK_OS" = "macos" ]; then
    clang++ "${APP_PATH}/test.cpp" -l SplashKit -rpath /usr/local/lib -o "${APP_PATH}/test"
    if [ ! $? -eq 0 ]; then
        echo "Failed to compile test program"
        exit 1
    fi
elif [ "$SK_OS" = "linux" ]; then
    g++ "${APP_PATH}/test.cpp" -l SplashKit -Wl,-rpath=/usr/local/lib -o "${APP_PATH}/test"
    if [ ! $? -eq 0 ]; then
        echo "Failed to compile test program"
        exit 1
    fi
elif [ "$IS_WINDOWS" = true ]; then
    g++ "${APP_PATH}/test.cpp" -lSplashKit -o "${APP_PATH}/test"

    if [ ! $? -eq 0 ]; then
        echo "Failed to compile test program"
        exit 1
    fi
fi

"${APP_PATH}/test"
if [ ! $? -eq 0 ]; then
    echo "Failed to run test program"
    exit 1
fi

rm "${APP_PATH}/test"

echo "Done"
