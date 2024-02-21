#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

cd "$APP_PATH"
git stash
git checkout master
git pull --force

echo "Detecting operating system"

if [ "$SK_OS" = "macos" ]; then
    LIB_DEST="/usr/local/lib/libSplashKit.dylib"
elif [ "$SK_OS" = "linux" ]; then
    LIB_DEST="/usr/local/lib/libSplashKit.so"
elif [ "$SK_OS" = "win32" ]; then
    LIB_DEST="/mingw32/lib/SplashKit.dll"
elif [ "$SK_OS" = "win64" ]; then
    LIB_DEST="/mingw32/lib/SplashKit.dll"
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [ "$SK_OS" = "linux" ]; then
    echo "Rebuilding library"
    skm linux install
fi

if [ -f "${LIB_DEST}" ]; then
    echo "Reinstalling globally"
    skm global install
fi
