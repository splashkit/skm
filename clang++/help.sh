#!/bin/bash

if [ "$1" = "-s" ] ; then
    echo "    clang++    Run the clang++ (or g++) compiler"
else
    bold=$(tput bold)
    normal=$(tput sgr0)

    echo "OVERVIEW: skm clang++ compiler call"
    echo
    echo "USAGE: skm clang++ [options] input"
    echo
    echo "Runs the clang++ (or g++) compiler with any requested options and input files."
    echo
    echo "ENVIRONMENT VARS:"
    echo "    LIBS: A string which can be used to specify other librarys which depend on SplashKit"
    echo
    echo "Example usage:"
    echo "    Compile a 'program.cpp' file into an executable program called 'HelloWorld'."
    echo "    ${bold}skm clang++ program.cpp -o HelloWorld${normal}"
    echo
    echo "    Compile all of the cpp files into an executable program called 'MyProgram'."
    echo "    ${bold}skm clang++ *.cpp -o MyProgram${normal}"
    echo
    echo "    Output options for the clang++ compiler"
    echo "    ${bold}skm clang++ --help${normal}"
    echo
    echo "    Using the LIBS envvar to compile with other libraries which depend on SplashKit."
    echo "    ${bold}LIBS='-lfirestorm' skm clang++ program.cpp${normal}"
fi
