#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

if [ "$1" = "-s" ] ; then
    echo "    resources  Create the resources folders used by splashkit for resources"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: initialise folder structure for resource management"
    echo
    echo "USAGE: skm resources"
    echo 
    echo "Create the resource folder structures for a SplashKit project."
    echo
    echo "Example usage:"
    echo "    Create the resources folder in the current folder."
    echo "    ${bold}skm resources${normal}"
fi

