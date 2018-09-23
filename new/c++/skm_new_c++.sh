#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

INCLUDE_PATH=`cd "$APP_PATH/../../clang++/include"; pwd`

if [ ! -d include ]; then
    mkdir include
else
    if [ -e include/splashkit ]; then
        rm -rf include/splashkit
    fi
fi

ln -f -s "${INCLUDE_PATH}" ./include/splashkit

if [ "`uname -o 2>>/dev/null`" = "Msys" ]; then
    cp -r -n "${APP_PATH}/files/" -T .
else
    cp -r -n "${APP_PATH}/files/" .
fi

../../skm fix c++