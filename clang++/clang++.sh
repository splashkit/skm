#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "win32" ]; then
    g++ -mwindows -g -Wall -Wl,--as-needed -std=c++14 -L"${APP_PATH}/lib/win32" -static-libstdc++ -static-libgcc -lSplashKit -Wl,-Bstatic -lstdc++ -lpthread -I ${APP_PATH}/include -I ${APP_PATH}/../ -L ${SKM_PATH}/lib/win32 -I "$APP_PATH/src" -I "$APP_PATH/win_include" $* "${APP_PATH}/lib/win32/libSplashKitCpp.a"
elif [ "$SK_OS" = "win64" ]; then
    #g++ -g -Wall -std=c++14 -L"${APP_PATH}/lib/win64" -static-libstdc++ -static-libgcc -lSplashKit -Wl,-Bstatic -lstdc++ -lpthread -I ${APP_PATH}/include -I ${APP_PATH}/../ -L ${SKM_PATH}/lib/win64 -I "$APP_PATH/src" -I "$APP_PATH/win_include" "$APP_PATH/src/*.cpp" $*
    g++ -mwindows -g -Wall -Wl,--as-needed -std=c++14 -L"${APP_PATH}/lib/win64" -static-libstdc++ -static-libgcc -lSplashKit -Wl,-Bstatic -lstdc++ -lpthread -I ${APP_PATH}/include -I ${APP_PATH}/../ -L ${SKM_PATH}/lib/win64 -I "$APP_PATH/src" -I "$APP_PATH/win_include" $* "${APP_PATH}/lib/win64/libSplashKitCpp.a"
elif [ "$SK_OS" = "macos" ]; then
    clang++ -g -Wall -std=c++14 -L"$DYLIB_PATH" -lSplashKit -L"${APP_PATH}/lib/macos" -lSplashKitCpp -I "${APP_PATH}/include" -rpath @loader_path -rpath "$DYLIB_PATH" -rpath /usr/local/lib $*
elif [ "$SK_OS" = "linux" ]; then
    clang++ -g -Wall -std=c++14 -L"$DYLIB_PATH" -lSplashKit -L"${APP_PATH}/lib/linux" -I ${APP_PATH}/include -Wl,-rpath=$ORIGIN -Wl,-rpath="${DYLIB_PATH}" -Wl,-rpath=/usr/local/lib $* -lSplashKitCPP
else
    echo "Unable to detect operating system..."
    exit 1
fi