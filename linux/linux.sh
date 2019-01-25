#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

case $1 in
    install)
    "${APP_PATH}/install/install.sh" $*
    ;;
    *)
    echo "Unknown option for skm linux. "    
esac
