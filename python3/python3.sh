#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ ! -z $PYTHONPATH ]; then
    PYTHONPATH="$APP_PATH:$PYTHONPATH"
else
    PYTHONPATH="$APP_PATH"
fi

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    DYLIB_PATH=`cd "$APP_PATH"/../lib/win64; pwd`
    PATH="$DYLIB_PATH:$PATH" PYTHONPATH="$PYTHONPATH" python3 $*
elif [ `uname` = "Darwin" ]; then
    DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`
    DYLD_LIBRARY_PATH="$DYLIB_PATH" PYTHONPATH="$PYTHONPATH" python3 $*
else
    DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`
    PYTHONPATH="$PYTHONPATH" python3 $*
fi