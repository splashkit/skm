#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`

clang++ -g -std=c++14 -L"$DYLIB_PATH" -lSplashKit -L"${APP_PATH}/lib" -lSplashKitCpp -I "${APP_PATH}/include" -rpath @loader_path -rpath "$DYLIB_PATH" -rpath /usr/local/lib $*