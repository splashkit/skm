#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`
PYTHONPATH="$APP_PATH:$PYTHONPATH"

PATH="$DYLIB_PATH:$PATH" DYLD_LIBRARY_PATH="$DYLIB_PATH" PYTHONPATH="$PYTHONPATH" python3 $*