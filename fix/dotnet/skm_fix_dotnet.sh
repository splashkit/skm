#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$IS_WINDOWS" = true ]; then
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$DYLIB_PATH\"|g" ./.vscode/launch.json
elif [ "$SK_OS" = "win64" ]; then
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$DYLIB_PATH\"|g" ./.vscode/launch.json
elif [ `uname` = "Darwin" ]; then
    sed -i '' "s|\"DYLD_LIBRARY_PATH\":.*\".*\"|\"DYLD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
else
    sed -i "s|\"LD_LIBRARY_PATH\":.*\".*\"|\"LD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
fi
