#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$IS_WINDOWS" = true ]; then
    APP_PATH_W=`cd "$APP_PATH"; pwd -W`
    fpc -S2 -Sh -Cg -Fu"${APP_PATH_W}" -k"-L${DYLIB_PATH}" -k"-lSplashKit.dll" $*
elif [ "$SK_OS" = "macos" ]; then
    ppcx64 -Tdarwin -S2 -Sh -XR/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk -WM10.11 -Cg -Fu"${APP_PATH}" -k"-L${DYLIB_PATH}" -k"-lSplashKit" -k"-rpath @loader_path -rpath ${DYLIB_PATH}" $*
elif [ "$SK_OS" = "linux" ]; then
    ppcx64 -S2 -Sh -Cg -Fu"${APP_PATH}" -k"-L${DYLIB_PATH}" -k"-lSplashKit" -k"-rpath=\$ORIGIN -rpath='${DYLIB_PATH}' -rpath=/usr/local/lib" $*
else
    echo "Unable to detect operating system..."
fi
