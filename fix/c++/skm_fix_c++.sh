#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

INCLUDE_PATH=`cd "$SKM_PATH/clang++/include"; pwd`

if [ ! -d include ]; then
    mkdir include
else
    if [ -e include/splashkit ]; then
        rm -rf include/splashkit
    fi
fi

ln -f -s "${INCLUDE_PATH}" ./include/splashkit

if [ "$IS_WINDOWS" = true ]; then
    GPP_PATH=`which g++ | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    GPP_PATH=`cd "$GPP_PATH"; pwd -W`
    GPP_PATH="$GPP_PATH/g++.exe"

    GDB_PATH=`which gdb | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    GDB_PATH=`cd "$GDB_PATH"; pwd -W`
    GDB_PATH="$GDB_PATH/gdb.exe"

    BASH_PATH=`which bash | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    BASH_PATH=`cd "$BASH_PATH"; pwd -W`
    BASH_PATH="$BASH_PATH/bash.exe"

    SKM_PATH=`cd "$APP_PATH"/../../; pwd`

    sed -i "s|\"terminal.integrated.shell.windows\":.*\".*:.*\"|\"terminal.integrated.shell.windows\": \"$BASH_PATH\"|g" ./.vscode/settings.json
    sed -i "s|\"miDebuggerPath\":.*\".*:.*\"|\"miDebuggerPath\": \"$GDB_PATH\"|g" ./.vscode/launch.json
    sed -i "s|\"value\": \"\${env:PATH};.*\"|\"value\": \"\${env:PATH};$DYLIB_PATH\"|g" ./.vscode/launch.json
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$SKM_PATH\"|g" ./.vscode/tasks.json
    sed -i "s|\"args\":.*\[\"--login\",\".*skm\"|\"args\": \[\"--login\",\"$SKM_PATH/skm\"|g" ./.vscode/tasks.json
else
    GPP_PATH=`which clang++`

    if [ -z "$VAR" ]; then
        GPP_PATH=`which g++`
    fi
fi

if [ $SK_OS = "macos" ]; then
    sed -i '' "s|\"compilerPath\":.*\".*:.*\"|\"compilerPath\": \"$GPP_PATH\"|g" ./.vscode/c_cpp_properties.json
else
    sed -i "s|\"compilerPath\":.*\".*:.*\"|\"compilerPath\": \"$GPP_PATH\"|g" ./.vscode/c_cpp_properties.json
fi