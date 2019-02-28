#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    dotnet     Run the dotnet command line tool for SplashKit"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm dotnet command line tool"
    echo
    echo "USAGE: skm dotnet [options]"
    echo 
    echo "Runs the dotnet command line tool with the provided options."
    echo
    echo "Example usage:"
    echo "    Create a new dotnet project"
    echo "    ${bold}skm dotnet new console${normal}"
    echo
    echo "    Restore a project to get changed or updated settings."
    echo "    ${bold}skm dotnet restore${normal}"
    echo 
    echo "    Build a project, creating the executable file."
    echo "    ${bold}skm dotnet build${normal}"
    echo 
    echo "    Build and run a project."
    echo "    ${bold}skm dotnet run${normal}"
    echo 
    echo "    Get additional help about the dotnet command line tool"
    echo "    ${bold}skm dotnet --help${normal}"
fi

