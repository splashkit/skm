#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)

SKM_PATH=$(cd "$APP_PATH/../.." && pwd)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "win64" ]; then
    if command -v pacman &>/dev/null; then
        # Check architecture
        if [[ $(uname) == *ARM64 ]]; then
            SHELL_NAME="CLANGARM64"
        else
            SHELL_NAME="MINGW64"
        fi

        if [ "$MSYSTEM" = "MINGW64" ] && [[ $(uname) != *ARM64 ]]; then
            : # All good - no op and continue
        elif [ "$MSYSTEM" = "CLANGARM64" ] && [[ $(uname) == *ARM64 ]]; then
            : # All good - no op and continue
        else
            echo "Unable to detect Windows version."
            echo "Please run in \"$SHELL_NAME\" terminal (not MSYS2 or UCRT terminals)"
            exit 1
        fi
    else
        echo "Unable to install dependencies in Git Bash terminal"
        echo "Please run in \"$SHELL_NAME\" terminal (not MSYS2 or UCRT terminals)"
        exit 1
    fi
elif [ "$SK_OS" = "macos" ] || [ "$SK_OS" = "linux" ] || (echo "${*}" | grep '\-\-no-os-detect'); then
    echo "Windows install only available on Windows"
    exit 1
else
    echo "Unable to detect operating system..."
    exit 1
fi

"${APP_PATH}/install_deps.sh"

# enable the integrated MINGW64 terminal in VS Code to update the .bash_history file
grep -Fqx "PROMPT_COMMAND='history -a'" ~/.bashrc || echo "PROMPT_COMMAND='history -a'" >>~/.bashrc
source ~/.bashrc

# Build library for ARM windows
if [[ $(uname) == *ARM64 ]]; then
    echo "Configuring SplashKit"
    cd "${SKM_PATH}/source"
    pwd
    cmake -G "Unix Makefiles" .
    if [ $? -ne 0 ]; then
        echo "Configuration failed"
        exit $?
    fi

    echo "Compiling SplashKit..."
    make
    if [ $? -ne 0 ]; then
        echo "Compilation failed"
        exit $?
    fi

    echo "Installing compiled SplashKit library..."
    make install
    if [ $? -ne 0 ]; then
        echo "Install failed"
        exit $?
    fi
fi

echo "SplashKit Installed"

"${SKM_PATH}/global/install/skm_global_install.sh"
