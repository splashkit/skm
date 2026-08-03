#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    windows    Run windows distribution specific commands - namely 'skm windows install'"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm windows commands"
    echo
    echo "USAGE: skm windows [options]"
    echo
    echo "Runs the windows command with the provided options."
    echo
    echo "Options:"
    if [[ $(uname) != *ARM64 ]]; then
        echo "    install     Installs the necessary pacman packages."
    else
        echo "    install     Installs the necessary pacman packages and builds the SplashKit library locally."
    fi
    echo "    build       Perform necessary installation steps to build the SplashKit library locally."
    echo
    echo "Example usage:"
    echo "    - Install the necessary pacman packages:"
    echo "    ${bold}skm windows install${normal}"
    echo
    echo "    - Build the SplashKit library locally:"
    echo "    ${bold}skm windows build${normal}"
    echo
fi

