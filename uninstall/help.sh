#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "$1" = "-s" ] ; then
    echo "    uninstall  Uninstall skm"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: uninstall skm"
    echo
    echo "USAGE: skm uninstall"
    echo
    echo "Uninstall skm."
    echo
    echo "Example usage:"
    echo "    Uninstall skm using."
    echo "    ${bold}skm uninstall${normal}"
fi