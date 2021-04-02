#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$1" = "-s" ] ; then
    echo "    revert     Downgrade to earlier versions of SplashKit"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: downgrade to an earlier version of splashkit"
    echo
    echo "USAGE: skm revert [version]"
    echo
    echo "Revert back to an earlier version of SplashKit."
    echo
    echo "VERSIONS:"
    echo
    echo "Replave [version] with one of the following:"
    echo " 1: Revert to version prior to integration of C++17 features. This may be needed for some older operating systems."
    echo
    echo "Example usage:"
    echo "    Revert to version prior to C++17 features."
    echo "    ${bold}skm revert 1${normal}"
fi

