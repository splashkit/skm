#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`


if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
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

    LIB_PATH=`cd "$APP_PATH/../../lib/win64"; pwd -W`
    
    sed -i "s|\"terminal.integrated.shell.windows\":.*\".*:.*\"|\"terminal.integrated.shell.windows\": \"$BASH_PATH\"|g" ./.vscode/settings.json
    sed -i "s|\"miDebuggerPath\":.*\".*:.*\"|\"miDebuggerPath\": \"$GDB_PATH\"|g" ./.vscode/launch.json
    sed -i "s|\"value\": \"\${env:PATH};.*\"|\"value\": \"\${env:PATH};$LIB_PATH\"|g" ./.vscode/launch.json
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$SKM_PATH\"|g" ./.vscode/tasks.json
    sed -i "s|\"args\":.*\[\"--login\",\".*skm\"|\"args\": \[\"--login\",\"$SKM_PATH/skm\"|g" ./.vscode/tasks.json
else
    GPP_PATH=`which clang++`

    if [ -z "$VAR" ]; then
        GPP_PATH=`which g++`
    fi
fi

sed -i "s|\"compilerPath\":.*\".*:.*\"|\"compilerPath\": \"$GPP_PATH\"|g" ./.vscode/c_cpp_properties.json