#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

INCLUDE_PATH=`cd "$SKM_PATH/clang++/include"; pwd`

if [ ! -d include ]; then
    mkdir include
else
    if [ -e include/splashkit ]; then
        rm -rf include/splashkit
    fi
fi

ln -f -s "${INCLUDE_PATH}" ./include/splashkit

if [ "$IS_WINDOWS" = true ]; then
    cp -r -n "${APP_PATH}/files/" -T .
else
    cp -r -n "${APP_PATH}/files/" .
fi

"$SKM_PATH/fix/c++/skm_fix_c++.sh"