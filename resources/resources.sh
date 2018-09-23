#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    cp -r -n "${APP_PATH}/Resources" -T .
else
    cp -r -n "${APP_PATH}/Resources" .
fi