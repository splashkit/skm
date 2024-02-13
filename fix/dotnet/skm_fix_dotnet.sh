#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

DIR_NAME="${PWD##*/}"
CORRECTED_DIR_NAME="${DIR_NAME//[^A-Za-z0-9]/_}"
if [[ "${CORRECTED_DIR_NAME::1}" =~ [0-9] ]]; then
    CORRECTED_DIR_NAME="_$CORRECTED_DIR_NAME"
fi
INIT_PRJ_NAME=`echo *.csproj`
PRJ_NAME="$CORRECTED_DIR_NAME.csproj"

SKM_PATH=`cd "$APP_PATH/../.."; pwd`
PROGRAM_CS_NAME='Program.cs'
MAC_RUN_CONFIG="  <PropertyGroup Condition=\" '\$(RunConfiguration)' == 'Default' \">\n\
    <StartAction>Project</StartAction>\n\
    <StartWorkingDirectory>.</StartWorkingDirectory>\n\
    <ExternalConsole>true</ExternalConsole>\n\
    <EnvironmentVariables>\n\
    <Variable name=\"DYLD_LIBRARY_PATH\" value=\"$DYLIB_PATH\" />\n\
    </EnvironmentVariables>\n  </PropertyGroup>\n"

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$IS_WINDOWS" = true ]; then
    sed -i "s|\"PATH\":.*\".*:.*\"|\"PATH\": \"\${env:PATH};$DYLIB_PATH\"|g" ./.vscode/launch.json
elif [ $SK_OS = "macos" ]; then
    sed -i '' "s|\"DYLD_LIBRARY_PATH\":.*\".*\"|\"DYLD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
else
    sed -i "s|\"LD_LIBRARY_PATH\":.*\".*\"|\"LD_LIBRARY_PATH\": \"$DYLIB_PATH\"|g" ./.vscode/launch.json
fi

if [ "$INIT_PRJ_NAME" != "$PRJ_NAME" ]; then
    mv -- "$INIT_PRJ_NAME" "$PRJ_NAME"
fi

if [ $SK_OS = "macos" ]; then
#     sed -i '' "s|<TargetFramework>.*</TargetFramework>|<TargetFramework>net8.0</TargetFramework>|g" "$PRJ_NAME"
    if ! grep -q RunConfiguration "$PRJ_NAME"; then
        sed -i '' "s|</Project>|$MAC_RUN_CONFIG\n</Project>|g" "$PRJ_NAME"
    fi
# else
#     sed -i "s|<TargetFramework>.*</TargetFramework>|<TargetFramework>net8.0</TargetFramework>|g" "$PRJ_NAME"
fi

if [ $SK_OS = "macos" ]; then
    sed -i '' "s|namespace .*|namespace $CORRECTED_DIR_NAME|g" "$PROGRAM_CS_NAME"
else
    sed -i "s|namespace .*|namespace $CORRECTED_DIR_NAME|g" "$PROGRAM_CS_NAME"
fi

dotnet restore
