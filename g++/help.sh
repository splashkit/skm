#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    g++        Run the g++ compiler"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm g++ compiler call"
    echo
    echo "USAGE: skm g++ [options] input"
    echo
    echo "ENVIRONMENT VARS:"
    echo "    LIBS: A string which can be used to specify other librarys which depend on SplashKit"
    echo
    echo "Runs the g++ (or g++) compiler with any requested options and input files."
    echo
    echo "Example usage:"
    echo "    Compile a 'program.cpp' file into an executable program called 'HelloWorld'."
    echo "    ${bold}skm g++ program.cpp -o HelloWorld${normal}"
    echo
    echo "    Compile a all of the cpp files into an executable program called 'MyProgram'."
    echo "    ${bold}skm g++ *.cpp -o MyProgram${normal}"
    echo
    echo "    Output options for the g++ compiler"
    echo "    ${bold}skm g++ --help${normal}"
    echo
    echo "    Using the LIBS envvar to compile with other libraries which depend on SplashKit."
    echo "    ${bold}LIBS='-lfirestorm' skm g++ program.cpp${normal}"
fi
