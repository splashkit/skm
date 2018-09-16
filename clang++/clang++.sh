#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH"/..; pwd`

DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    #g++ -g -Wall -std=c++14 -L"${APP_PATH}/lib/win64" -static-libstdc++ -static-libgcc -lSplashKit -Wl,-Bstatic -lstdc++ -lpthread -I ${APP_PATH}/include -I ${APP_PATH}/../ -L ${SKM_PATH}/lib/win64 -I "$APP_PATH/src" -I "$APP_PATH/win_include" "$APP_PATH/src/*.cpp" $*
    g++ -g -Wall -Wl,--as-needed -std=c++14 -L"${APP_PATH}/lib/win64" -static-libstdc++ -static-libgcc -lSplashKit -Wl,-Bstatic -lstdc++ -lpthread -I ${APP_PATH}/include -I ${APP_PATH}/../ -L ${SKM_PATH}/lib/win64 -I "$APP_PATH/src" -I "$APP_PATH/win_include" $* "${APP_PATH}/lib/win64/libSplashKitCpp.a"
else
    clang++ -g -Wall -std=c++14 -L"$DYLIB_PATH" -lSplashKit -L"${APP_PATH}/lib" -lSplashKitCpp -I "${APP_PATH}/include" -rpath @loader_path -rpath "$DYLIB_PATH" -rpath /usr/local/lib $*
fi