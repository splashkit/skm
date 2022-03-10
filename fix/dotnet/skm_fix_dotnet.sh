#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

PRJ_NAME="${PWD##*/}.csproj"
SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$IS_WINDOWS" = true ]; then
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$DYLIB_PATH\"|g" ./.vscode/launch.json
elif [ $SK_OS = "macos" ]; then
    sed -i '' "s|\"DYLD_LIBRARY_PATH\":.*\".*\"|\"DYLD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
else
    sed -i "s|\"LD_LIBRARY_PATH\":.*\".*\"|\"LD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
fi

if [ $SK_OS = "macos" ]; then
    sed -i '' "s|<TargetFramework>.*</TargetFramework>|<TargetFramework>netcoreapp5.0</TargetFramework>|g" "$PRJ_NAME"
    sed -i '' "s|<ImplicitUsings>.*</ImplicitUsings>|<ImplicitUsings>disable</ImplicitUsings>|g" "$PRJ_NAME"
else
    sed -i "s|<TargetFramework>.*</TargetFramework>|<TargetFramework>netcoreapp5.0</TargetFramework>|g" "$PRJ_NAME"
    sed -i "s|<ImplicitUsings>.*</ImplicitUsings>|<ImplicitUsings>disable</ImplicitUsings>|g" "$PRJ_NAME"
fi

dotnet restore
