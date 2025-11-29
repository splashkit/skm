#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)

SKM_PATH=$(cd "$APP_PATH/.." && pwd)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

cd "$APP_PATH"
git stash

BRANCH_NAME=$(git branch --show-current)
git checkout $BRANCH_NAME

git pull --force

echo "Detecting operating system"

if [ "$SK_OS" = "macos" ]; then
    LIB_DEST="/usr/local/lib/libSplashKit.dylib"
elif [ "$SK_OS" = "linux" ]; then
    LIB_DEST="/usr/local/lib/libSplashKit.so"
elif [ "$SK_OS" = "win64" ]; then
    if [[ $(uname) == *ARM64 ]]; then
        LIB_DEST="/clangarm64/lib/SplashKit.dll"
    else
        LIB_DEST="/mingw64/lib/SplashKit.dll"
    fi
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [ "$SK_OS" = "macos" ]; then
    OSX_VERSION=$(sw_vers -productVersion)
    if ! awk "BEGIN{ exit ($OSX_VERSION < 12.3) }"; then
        echo "Rebuilding library"
        "${SKM_PATH}/macos/install/install.sh"
    fi
elif [ "$SK_OS" = "linux" ]; then
    echo "Rebuilding library"
    "${SKM_PATH}/linux/install/install.sh"
fi

if [ -f "${LIB_DEST}" ]; then
    if [[ $(uname) == *ARM64 ]]; then
        echo "Rebuilding library"
        "${SKM_PATH}/windows/install/install.sh"
    fi
    echo "Reinstalling globally"
    "${SKM_PATH}/global/install/skm_global_install.sh"
fi
