#!/bin/sh

GPP_PATH=`which g++ | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
GPP_PATH=`cd "$GPP_PATH"; pwd -W`

if [ `uname -o` = "Msys" ]; then
    sed -i "s|\"compilerPath\":.*\".*:.*\"|\"compilerPath\": \"$GPP_PATH/g++.exe\"|g" ./.vscode/c_cpp_properties.json
fi
