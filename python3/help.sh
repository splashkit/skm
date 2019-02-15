#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    python3    Run python3 with SplashKit settings"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: run python3 with SplashKit settings"
    echo
    echo "USAGE: skm python3 [options] [input]"
    echo 
    echo "Runs python3 with the required SplashKit settings and any additional options and input files."
    echo
    echo "Example usage:"
    echo "    Run a 'program.py' file using"
    echo "    ${bold}skm python3 program.py${normal}"
    echo
    echo "    Run an interactive python3 REPL"
    echo "    ${bold}skm python3${normal}"
    echo 
    echo "    Output options for python3"
    echo "    ${bold}skm python3 --help${normal}"
fi

