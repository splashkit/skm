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
    CPP_LIB_FILE="${SKM_PATH}/clang++/lib/macos/libSplashKitCPP.a"
elif [ "$SK_OS" = "linux" ]; then
    LIB_FILE="${SKM_PATH}/lib/linux/libSplashKit.so"
    CPP_LIB_FILE="${SKM_PATH}/clang++/lib/linux/libSplashKitCPP.a"
elif [ "$SK_OS" = "win32" ]; then
    LIB_FILE="${SKM_PATH}/lib/win32/SplashKit.dll"
    CPP_LIB_FILE="${SKM_PATH}/clang++/lib/win32/libSplashKitCPP.a"
    WIN_OUT_DIR="${WINDIR}/System32"
elif [ "$SK_OS" = "win64" ]; then
    LIB_FILE="${SKM_PATH}/lib/win64/SplashKit.dll"
    CPP_LIB_FILE="${SKM_PATH}/clang++/lib/win64/libSplashKitCPP.a"
    WIN_OUT_DIR="${WINDIR}/System32"
else
    echo "Unable to detect operating system..."
    exit 1
fi

$PRIVILEGED cp $LIB_FILE /usr/local/lib
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit library to /usr/local/lib"
    exit 1
fi

# If $WIN_OUT_DIR is set, we are on Windows and need to copy the dll to the System32 or System64 directory
if [ ! -z "$WIN_OUT_DIR" ]; then
    $PRIVILEGED cp "$LIB_FILE" "$WIN_OUT_DIR"
    if [ ! $? -eq 0 ]; then
        echo "Failed to copy SplashKit library to $WIN_OUT_DIR"
        exit 1
    fi
fi

$PRIVILEGED cp "$CPP_LIB_FILE" /usr/local/lib
if [ ! $? -eq 0 ]; then
    echo "Failed to copy SplashKit C++ library to /usr/local/lib"
    exit 1
fi

if [ "$SK_OS" = "macos" ]; then
    echo "Linking dll for dotnet"
    sudo ln -s /usr/local/lib/libSplashKit.dylib /usr/local/lib/libsplashkit.dll.dylib
    if [ ! $? -eq 0 ]; then
        echo "Failed to create /usr/local/lib/libsplashkit.dll.dylib"
        exit 1
    fi
elif [ "$SK_OS" = "linux" ]; then
    echo "Linking dll for dotnet"
    sudo ln -s /usr/local/lib/libSplashKit.so /usr/local/lib/libsplashkit.dll.so
    if [ ! $? -eq 0 ]; then
        echo "Failed to create /usr/local/lib/libsplashkit.dll.so"
        exit 1
    fi
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

echo "Testing install"

if [ "$SK_OS" = "macos" ]; then
    clang++ "${APP_PATH}/test.cpp" -l SplashKitCPP -l SplashKit -rpath /usr/local/lib -o "${APP_PATH}/test"
    if [ ! $? -eq 0 ]; then
        echo "Failed to compile test program"
        exit 1
    fi
elif [ "$SK_OS" = "linux" ]; then
    g++ "${APP_PATH}/test.cpp" -l SplashKitCPP -l SplashKit -Wl,-rpath=/usr/local/lib -o "${APP_PATH}/test"
    if [ ! $? -eq 0 ]; then
        echo "Failed to compile test program"
        exit 1
    fi
elif [ "$IS_WINDOWS" = true ]; then
    g++ "${APP_PATH}/test.cpp" -L /usr/local/lib -lSplashKit -I /usr/local/include/ /usr/local/lib/libSplashKitCPP.a -o "${APP_PATH}/test"

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
