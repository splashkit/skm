#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ ! -z $PYTHONPATH ]; then
    PYTHONPATH="$APP_PATH:$PYTHONPATH"
else
    PYTHONPATH="$APP_PATH"
fi

if [ "$SK_OS" = "win32" ]; then
    PATH="$DYLIB_PATH$PATH" PYTHONPATH="$PYTHONPATH" python3 $*
elif [ "$SK_OS" = "win64" ]; then
    PATH="$DYLIB_PATH:$PATH" PYTHONPATH="$PYTHONPATH" python3 $*
elif [ "$SK_OS" = "macos" ]; then
    DYLD_LIBRARY_PATH="$DYLIB_PATH" PYTHONPATH="$PYTHONPATH" python3 $*
elif [ "$SK_OS" = "linux" ]; then
    LD_LIBRARY_PATH="$DYLIB_PATH;$LD_LIBRARY_PATH" PYTHONPATH="$PYTHONPATH" python3 $*
else
    echo "Unable to detect operating system..."
    exit 1
fi