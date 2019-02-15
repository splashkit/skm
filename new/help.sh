#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$1" = "-s" ] ; then
    echo "    new        Create a new SplashKit project in the current folder"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: create new SplashKit project"
    echo
    echo "USAGE: skm new [language]"
    echo
    echo "Perform necessary installation steps to build the SplashKit library locally. This will attempt to install the necessary components, or will provide instructions to do this manually."
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
    echo "    Create a new c++ project."
    echo "    ${bold}skm new c++${normal}"
fi

