#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    DYLD_PATH=`cd $APP_PATH/../../lib; pwd -W`
    PYTHON_PATH=`cd $APP_PATH/../../python3; pwd -W`

    PYTHON_BIN=`which python3 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
    PYTHON_BIN=`cd "$PYTHON_BIN"; pwd -W`
    PYTHON_BIN="$PYTHON_BIN/python3.exe"
else
    DYLD_PATH=`cd $APP_PATH/../../lib; pwd`
    PYTHON_PATH=`cd $APP_PATH/../../python3; pwd`
    PYTHON_BIN=`which python3`
fi

sed -i'' "s|PYTHONPATH=.*|PYTHONPATH=$PYTHON_PATH|g" .env
sed -i'' "s|DYLD_LIBRARY_PATH=.*|DYLD_LIBRARY_PATH=$DYLD_PATH|g" .env
sed -i'' "s|\"python.pythonPath\":.*\".*\"|\"python.pythonPath\": \"$PYTHON_BIN\"|g" ./.vscode/settings.json
