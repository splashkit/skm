#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    DYLD_PATH=`cd $APP_PATH/../../lib/win64; pwd -W`
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$DYLD_PATH\"|g" ./.vscode/launch.json
elif [ `uname` = "Darwin" ]; then
    DYLD_PATH=`cd $APP_PATH/../../lib/macos; pwd`
    sed -i '' "s|\"DYLD_LIBRARY_PATH\":.*\".*\"|\"DYLD_LIBRARY_PATH\": \"$DYLD_PATH\"|g" ./.vscode/launch.json
else
    DYLD_PATH=`cd $APP_PATH/../../lib/linux; pwd`
    sed -i "s|\"LD_LIBRARY_PATH\":.*\".*\"|\"LD_LIBRARY_PATH\": \"$DYLD_PATH\"|g" ./.vscode/launch.json
fi

