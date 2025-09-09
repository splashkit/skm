#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)

if [ "$1" = "-s" ]; then
    echo "    vscode      Update Visual Studio Code to add useful 'settings' and install useful 'extensions'"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm vscode commands"
    echo
    echo "USAGE: skm vscode [options]"
    echo
    echo "Runs the vscode command with the provided options."
    echo
    echo "Options:"
    echo "  settings     Update Visual Studio Code settings to add useful settings."
    echo "  extensions   Install Visual Studio Code extensions for C++, C# and Python development."
    echo
    echo "Example usage:"
    echo "    Update VS Code settings"
    echo "    ${bold}skm vscode settings${normal}"
    echo
    echo "    Install VS Code extensions for C++, C# and Python"
    echo "    ${bold}skm vscode extensions${normal}"
fi
