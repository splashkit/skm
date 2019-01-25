#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$1" = "-s" ] ; then
    echo "    fix        Correct path issues in SplashKit project files"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: Correct path issues in SplashKit project files"
    echo
    echo "USAGE: skm new [language]"
    echo
    echo "Updates paths in SplashKit project files based on tool paths identified in the current environment."
    echo
    echo "LANGUAGES:"
    echo
    echo "You can fix projects for the following languages:"
    echo

    for i in `find "$APP_PATH" -maxdepth -type d1 | sort`
    do
        if [ -f "$i/lang_details.sh" ]; then
            "$i/lang_details.sh"
        fi
    done

    echo
    echo "Example usage:"
    echo "    Fix a c++ project."
    echo "    ${bold}skm fix c++${normal}"
fi

