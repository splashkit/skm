#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    macos      Run macos distribution specific commands - namely 'skm macos install'"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm macos commands"
    echo
    echo "USAGE: skm macos install [--no-os-detect]"
    echo
    echo "Perform necessary installation steps to build the SplashKit library locally. This will attempt to install the necessary components, or will provide instructions to do this manually."
    echo
    echo "Options:"
    echo "  --no-os-detect  Flag to bypass OS detection, in case OS not detected correctly"
    echo
    echo "Example usage:"
    echo "    Run the install scripts for SphasKit on macOS."
    echo "    ${bold}skm macos install${normal}"
fi

