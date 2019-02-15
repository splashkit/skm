#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$IS_WINDOWS" = true ]; then
    PYTHON_PATH=`cd $SKM_PATH/python3; pwd -W`

    PYTHON_BIN=`which python3 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    PYTHON_BIN=`cd "$PYTHON_BIN"; pwd -W`
    PYTHON_BIN="$PYTHON_BIN/python3.exe"
else
    PYTHON_PATH=`cd $SKM_PATH/python3; pwd`
    PYTHON_BIN=`which python3`
fi

if [ $SK_OS = "macos" ]; then
    sed -i '' "s|PYTHONPATH=.*|PYTHONPATH=$PYTHON_PATH|g" .env
    sed -i '' "s|DYLD_LIBRARY_PATH=.*|DYLD_LIBRARY_PATH=$DYLIB_PATH|g" .env
    sed -i '' "s|\"python.pythonPath\":.*\".*\"|\"python.pythonPath\": \"$PYTHON_BIN\"|g" ./.vscode/settings.json
else
    sed -i "s|PYTHONPATH=.*|PYTHONPATH=$PYTHON_PATH|g" .env
    sed -i "s|DYLD_LIBRARY_PATH=.*|DYLD_LIBRARY_PATH=$DYLIB_PATH|g" .env
    sed -i "s|\"python.pythonPath\":.*\".*\"|\"python.pythonPath\": \"$PYTHON_BIN\"|g" ./.vscode/settings.json
fi