#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    DYLD_PATH=`cd $APP_PATH/../../lib; pwd -W`
    PYTHON_PATH=`cd $APP_PATH/../../python3; pwd -W`
else
    DYLD_PATH=`cd $APP_PATH/../../lib; pwd`
    PYTHON_PATH=`cd $APP_PATH/../../python3; pwd`
fi

PYTHON_BIN=`which python3`

sed -i '' "s|PYTHONPATH=.*|PYTHONPATH=$PYTHON_PATH|g" ./.env
sed -i '' "s|DYLD_LIBRARY_PATH=.*|DYLD_LIBRARY_PATH=$DYLD_PATH|g" ./.env
sed -i '' "s|\"python.pythonPath\":.*\".*\"|\"python.pythonPath\": \"$PYTHON_BIN\"|g" ./.vscode/settings.json
