#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ -f "${APP_PATH}/$1/skm_new_$1.sh" ]; then
    "${APP_PATH}/$1/skm_new_$1.sh"
else
    "$APP_PATH/help.sh"
    exit 1
fi
