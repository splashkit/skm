#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

mkdir include
ln -f -s "${APP_PATH}/include" ./include/splashkit
cp -r -n "${APP_PATH}/files/" .

if [ `uname -o 2>>/dev/null` = "Msys" ]; then
    skm fix c++
fi

