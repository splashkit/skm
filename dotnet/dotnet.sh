#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

DYLIB_PATH=`cd "$APP_PATH"/../lib; pwd`

if [ -f "Program.cs" ]; then
    PROGRAM_EXISTS=1
else
    PROGRAM_EXISTS=0
fi

dotnet $*

function restore_skm_dotnet {
    if [ -f "Program.cs" ]; then
        if [ "$PROGRAM_EXISTS" -ne 1 ]; then
            rm "Program.cs"
        fi

        mkdir -p ./lib
        ln -fs "${APP_PATH}/SplashKit.cs" ./lib/SplashKit.cs
        cp -r -n "${APP_PATH}/files/" .
    fi
}

case $1 in
    new)
    if [ "$#" -ge 2 ]; then
        restore_skm_dotnet
    fi
    ;;

    restore)
    restore_skm_dotnet
    ;;
esac
