#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "$1" = "-s" ] ; then
    echo "    update     Update to the latest version of SKM"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: update to the latest version of skm"
    echo
    echo "USAGE: skm update"
    echo 
    echo "Update skm to the latest version."
    echo
    echo "Example usage:"
    echo "    Update skm using."
    echo "    ${bold}skm update${normal}"
fi

