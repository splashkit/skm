#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$1" = "-s" ] ; then
    echo "    global     Setup SplashKit to work via global install"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: install globally"
    echo
    echo "USAGE: skm global [options]"
    echo
    echo "Perform necessary installation steps to make SplashKit available globally within your computer. This may require root/administrator access."
    echo
    echo "Options:"
    echo

    for i in `find "$APP_PATH" -maxdepth 1 -type d | sort`
    do
        if [ -f "$i/command_details.sh" ]; then
            "$i/command_details.sh"
        fi
    done

    echo
    echo "Example usage:"
    echo "    Install globally"
    echo "    ${bold}skm global install${normal}"
fi

