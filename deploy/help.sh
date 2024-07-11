#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$1" = "-s" ] ; then
    echo "    deploy     Package your SplashKit program for deployment to others."
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: package your program so it can be deployed elsewhere"
    echo
    echo "USAGE: skm deploy [language]"
    echo
    echo "Perform necessary steps to create a build of your SplashKit program that can be distributed to others."
    echo
    echo "LANGUAGES:"
    echo
    echo "You can create projects for the following languages:"
    echo

    for i in `find "$APP_PATH" -maxdepth 1 -type d | sort`
    do
        if [ -f "$i/lang_details.sh" ]; then
            "$i/lang_details.sh"
        fi
    done

    echo
    echo "Example usage:"
    echo "    Deploy MyGame executable."
    echo "    ${bold}skm deploy native MyGame${normal}"
fi

