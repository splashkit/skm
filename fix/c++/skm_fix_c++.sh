#!/bin/sh

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    GPP_PATH=`which g++ | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    GPP_PATH=`cd "$GPP_PATH"; pwd -W`
    GPP_PATH="$GPP_PATH/g++.exe"

    BASH_PATH=`which bash | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    BASH_PATH=`cd "$BASH_PATH"; pwd -W`
    BASH_PATH="$BASH_PATH/bash.exe"
    
    sed -i "s|\"terminal.integrated.shell.windows\":.*\".*:.*\"|\"terminal.integrated.shell.windows\": \"$BASH_PATH\"|g" ./.vscode/settings.json
else
    GPP_PATH=`which clang++`

    if [ -z "$VAR" ]; then
        GPP_PATH=`which g++`
    fi
fi

sed -i "s|\"compilerPath\":.*\".*:.*\"|\"compilerPath\": \"$GPP_PATH\"|g" ./.vscode/c_cpp_properties.json