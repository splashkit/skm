#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`

if [ `uname` = "Darwin" ]; then
    ppcx64 -S2 -Sh -k"-macosx_version_min 10.11" -Cg -Fu"${APP_PATH}" -k"-L${DYLIB_PATH}" -k"-lSplashKit" -k"-rpath @loader_path -rpath ${APP_PATH} -rpath /usr/local/lib" $*
elif [ `uname` = "Linux" ]; then
    ppcx64 -S2 -Sh -Cg -Fu"${APP_PATH}" -k"-L${DYLIB_PATH}" -k"-lSplashKit" -k"-rpath=\$ORIGIN -rpath='${DYLIB_PATH}' -rpath=/usr/local/lib" $*
elif [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    fpc -S2 -Sh -Cg -Fu"${APP_PATH}" -k"-L${DYLIB_PATH}" -k"-lSplashKit" $*
else
    echo "Unable to detect operating system..."
fi
