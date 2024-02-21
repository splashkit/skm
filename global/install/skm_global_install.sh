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

if [ ! -d /usr/local/lib ]; then
    echo "Creating directory /usr/local/lib"
    $PRIVILEGED mkdir -p /usr/local/lib
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory /usr/local/lib"
        exit 1
    fi
fi

if [ ! -d /usr/local/include/splashkit ]; then
    echo "Creating directory /usr/local/include/splashkit"
    $PRIVILEGED mkdir -p /usr/local/include/splashkit
    if [ ! $? -eq 0 ]; then
        echo "Failed to create directory /usr/local/include/splashkit"
        exit 1
    fi
fi

echo "Copying files to /usr/local/lib"

if [ "$SK_OS" = "macos" ]; then
    LIB_FILE="${SKM_PATH}/lib/macos/libSplashKit.dylib"
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="${SKM_PATH}/lib/linux/libSplashKit.so"
elif [ "$SK_OS" = "win32" ]; then
    LIB_FILE="${SKM_PATH}/lib/win32/SplashKit.dll"
    WIN_OUT_DIR="${WINDIR}/System32"
elif [ "$SK_OS" = "win64" ]; then
    LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    WIN_OUT_DIR="${WINDIR}/System32"
else
    echo "Unable to detect operating system..."
    exit 1
fi

$PRIVILEGED cp -f $LIB_FILE /usr/local/lib
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit library to /usr/local/lib"
    exit 1
fi

if [ "$SK_OS" = "linux" ]; then
    echo "Updating library config cache"
    $PRIVILEGED ldconfig
fi

echo "Copying files to /usr/local/include/splashkit"
$PRIVILEGED cp "${SKM_PATH}/clang++/include/"* /usr/local/include/splashkit
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit C++ headers to /usr/local/include/splashkit"
    exit 1
fi

$PRIVILEGED cp "${APP_PATH}/splashkit.h" /usr/local/include
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit header to /usr/local/include"
    exit 1
fi

if [ "$SK_OS" = "win32" ]; then
    # Copy to msys include folders
    echo "Copying files to /mingw32/include/splashkit"
    $PRIVILEGED cp "${SKM_PATH}/clang++/include/"* /mingw32/include/splashkit
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit C++ headers to /mingw32/include/splashkit"
        exit 1
    fi

    $PRIVILEGED cp "${APP_PATH}/splashkit.h" /mingw32/include
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit header to /mingw32/include"
        exit 1
    fi
elif [ "$SK_OS" = "win64" ]; then
    # Copy to msys include folders
    echo "Copying files to /mingw64/include/splashkit"
    $PRIVILEGED cp "${SKM_PATH}/clang++/include/"* /mingw64/include/splashkit
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit C++ headers to /mingw64/include/splashkit"
        exit 1
    fi

    $PRIVILEGED cp "${APP_PATH}/splashkit.h" /mingw64/include
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit header to /mingw64/include"
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
