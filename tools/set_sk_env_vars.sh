#!/bin/bash

if [ "$(uname -o 2>>/dev/null)" = "Msys" ]; then
    if [ "$MSYSTEM" = "MINGW64" ] && [[ $(uname) != *ARM64 ]]; then
        export SK_OS="win64"
    elif [ "$MSYSTEM" = "CLANGARM64" ] && [[ $(uname) == *ARM64 ]]; then
        export SK_OS="win64"
    else
        echo "Unable to detect Windows version..."
        if [[ $(uname) == *ARM64 ]]; then
            echo "Please run in CLANGARM64 terminal"
        else
            echo "Please run in MINGW64 terminal"
        fi
        exit 1
    fi
elif [ $(uname) = "Darwin" ]; then
    export SK_OS="macos"
elif [ $(uname) = "Linux" ]; then
    export SK_OS="linux"
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [ $SK_OS = "win64" ]; then
    export DYLIB_PATH=$(cd "$SKM_PATH/lib/win64" && pwd -W)
    export DYLIB_PATH_MSYS=$(cd "$SKM_PATH/lib/win64" && pwd)
    export IS_WINDOWS=true
elif [ $SK_OS = "macos" ]; then
    export DYLIB_PATH=$(cd "$SKM_PATH/lib/macos" && pwd)
    export IS_WINDOWS=false
else
    if [ -d "$SKM_PATH/lib/linux" ]; then
        export DYLIB_PATH=$(cd "$SKM_PATH/lib/linux" && pwd)
    else
        echo "Unable to locate SplashKit library - please run ${bold}skm linux install${normal}"
    fi
    export IS_WINDOWS=false
fi
