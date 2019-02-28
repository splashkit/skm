#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ -f "Program.cs" ]; then
    PROGRAM_EXISTS=1
else
    PROGRAM_EXISTS=0
fi

if [ "$SK_OS" = "win32" ]; then
    PATH="$DYLIB_PATH;$PATH" dotnet $*
elif [ "$SK_OS" = "win64" ]; then
    PATH="$DYLIB_PATH;$PATH" dotnet $*
elif [ "$SK_OS" = "macos" ]; then
    DYLD_LIBRARY_PATH="$DYLIB_PATH" dotnet $*
elif [ "$SK_OS" = "linux" ]; then
    LD_LIBRARY_PATH="$DYLIB_PATH:$LD_LIBRARY_PATH" dotnet $*
else
    echo "Unable to detect operating system..."
    exit 1
fi

restore_skm_dotnet () {
    if [ -f "Program.cs" ]; then
        if [ "$PROGRAM_EXISTS" -ne 1 ]; then
            rm "Program.cs"
        fi

        mkdir -p ./lib
        ln -fs "${APP_PATH}/SplashKit.cs" ./lib/SplashKit.cs
        if [ "$SK_OS" = "macos" ]; then
            cp -r -n "${APP_PATH}/files/" .
        else
            cp -r -n "${APP_PATH}/files/" -T .
        fi
    fi
}

case $1 in
    new)
    if [ "$#" -ge 2 ]; then
        restore_skm_dotnet
        $SKM_PATH/fix/dotnet/skm_fix_dotnet.sh
    fi
    ;;

    restore)
    restore_skm_dotnet
    ;;
esac
