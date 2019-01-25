#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    fpc        Run the Free Pascal compiler"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm free pascal compiler call"
    echo
    echo "USAGE: skm fpc [options] input"
    echo 
    echo "Runs fpc with any requested options and input files."
    echo
    echo "Example usage:"
    echo "    Compile the 'Program.pas' file"
    echo "    ${bold}skm fpc Program.pas${normal}"
fi

