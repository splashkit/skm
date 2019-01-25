#!/bin/bash

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    if [ "$MSYSTEM" = "MINGW32" ]; then
        export SK_OS="win32"
    elif [ "$MSYSTEM" = "MINGW64" ]; then
        export SK_OS="win64"
    else
        echo "Unable to detect Windows version..."
        echo "Please run in MinGw64 or MinGw32 terminal"
        exit 1
    fi
elif [ `uname` = "Darwin" ]; then
    export SK_OS="macos"
elif [ `uname` = "Linux" ]; then
    export SK_OS="linux"
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [ $SK_OS = "win32" ]; then
    export DYLIB_PATH=`cd "$SKM_PATH/lib/win32"; pwd -W`
    export DYLIB_PATH_MSYS=`cd "$SKM_PATH/lib/win32"; pwd`
    export IS_WINDOWS=true
elif [ $SK_OS = "win64" ]; then
    export DYLIB_PATH=`cd "$SKM_PATH/lib/win64"; pwd -W`
    export DYLIB_PATH_MSYS=`cd "$SKM_PATH/lib/win64"; pwd`
    export IS_WINDOWS=true
elif [ $SK_OS = "macos" ]; then
    export DYLIB_PATH=`cd "$SKM_PATH/lib/macos"; pwd`
    export IS_WINDOWS=false
else
    export DYLIB_PATH=`cd "$SKM_PATH/lib/linux"; pwd`
    export IS_WINDOWS=false
fi