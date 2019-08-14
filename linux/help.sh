#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    linux      Run linux distribution specific commands - namely 'skm linux install'"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm linux commands"
    echo
    echo "USAGE: skm linux install [--no-os-detect]"
    echo 
    echo "Perform necessary installation steps to build the SplashKit library locally. This will attempt to install the necessary components, or will provide instructions to do this manually."
    echo
    echo "Options:"
    echo "  --no-os-detect  Flag to bypass OS detection, in case OS not detected correctly"
    echo
    echo "Example usage:"
    echo "    Run the install scripts for SphasKit on Linux."
    echo "    ${bold}skm linux install${normal}"
fi

