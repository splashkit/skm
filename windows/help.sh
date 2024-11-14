#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    windows      Run windows distribution specific commands - namely 'skm windows install'"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm windows commands"
    echo
    echo "USAGE: skm windows install [--no-os-detect]"
    echo 
    echo "Perform necessary installation steps to build the SplashKit library locally. This will attempt to install the necessary components, or will provide instructions to do this manually."
    echo
    echo "Options:"
    echo "  --no-os-detect  Flag to bypass OS detection, in case OS not detected correctly"
    echo
    echo "Example usage:"
    echo "    Run the install scripts for SphasKit on windows."
    echo "    ${bold}skm windows install${normal}"
fi

