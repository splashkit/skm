#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "macos" ]; then
    cp -r -n "${APP_PATH}/files/" .
else
    cp -r -n "${APP_PATH}/files/" -T .
fi

"$SKM_PATH/fix/python3/skm_fix_python3.sh"
