#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "macos" ]; then
    cp -r --update=none "${APP_PATH}/files/" .
else
    cp -r --update=none "${APP_PATH}/files/" -T .
fi

"$SKM_PATH/fix/cargo/skm_fix_cargo.sh"
