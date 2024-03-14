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

if [ "$1" = "new" -a "$#" -eq 1 ]; then
    EXTRA="console"
else
    EXTRA=""
fi

restore_skm_dotnet () {
    mkdir -p ./lib
    ln -fs "${APP_PATH}/SplashKit.cs" ./lib/SplashKit.cs
    if [ "$SK_OS" = "macos" ]; then
        cp -r -n "${APP_PATH}/files/" .
    else
        cp -r -n "${APP_PATH}/files/" -T .
    fi

    "$SKM_PATH"/fix/dotnet/skm_fix_dotnet.sh
}

run_dotnet () {
    if [ "$SK_OS" = "win64" ]; then
        PATH="$DYLIB_PATH:$PATH" dotnet $* $EXTRA
    elif [ "$SK_OS" = "macos" ]; then
        DYLD_LIBRARY_PATH="$DYLIB_PATH" dotnet $* $EXTRA
    elif [ "$SK_OS" = "linux" ]; then
        LD_LIBRARY_PATH="$DYLIB_PATH:$LD_LIBRARY_PATH" dotnet $* $EXTRA
    else
        echo "Unable to detect operating system..."
        exit 1
    fi
}

case $1 in
    new)
    run_dotnet $*
    # Check if program.cs exists... do did it succeed
    if [ -f "Program.cs" ]; then
        if [ "$PROGRAM_EXISTS" -ne 1 ]; then
            rm "Program.cs"
        fi

        restore_skm_dotnet
    fi
    ;;

    restore)
    restore_skm_dotnet
    # If Program.cs did not exist at start... remove it as we added it on restore!
    if [ "$PROGRAM_EXISTS" -ne 1 ]; then
        rm "Program.cs"
    fi
    ;;

    *)
    run_dotnet $*
    ;;
esac

# Cause dotnet to avoid reporting success - as dotnet does not return error codes
exit 1
