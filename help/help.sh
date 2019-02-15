#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/.."; pwd`

if [ "$#" -ge 1 ] ; then
    case $1 in
        -s)
        echo "    help       Output skm usage details"
        ;;

        *)
        if [ -f "$SKM_PATH/$1/help.sh" ]; then
            COMMAND=$1
            shift
            "$SKM_PATH/$COMMAND/help.sh" $*
        else
            echo "There is no command named '$1'. Use 'skm help' for a list of commands."
        fi
        ;;
    esac
else
    echo "OVERVIEW: SplashKit Manager - skm"
    echo
    echo "USAGE: skm [command] [arguments]"
    echo
    echo "COMMANDS: "
    for i in `find "$SKM_PATH" -maxdepth 1 -type d | sort`
    do
        if [ -f "$i/help.sh" ]; then
            "$i/help.sh" -s
        fi
    done
    echo
    echo "Example usage:"
    echo "    Output this help message"
    echo "    ${bold}skm help${normal}"
    echo
    echo "    Get usage details for clang++ compiler"
    echo "    ${bold}skm help clang++${normal}"
    echo
    echo "    Get usage details for python3"
    echo "    ${bold}skm help python3${normal}"

fi
