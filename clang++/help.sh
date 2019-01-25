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
    echo "Example usage:"
    echo "    Compile a 'program.cpp' file into an executable program called 'HelloWorld'."
    echo "    ${bold}skm clang++ program.cpp -o HelloWorld${normal}"
    echo
    echo "    Compile all of the cpp files into an executable program called 'MyProgram'."
    echo "    ${bold}skm clang++ *.cpp -o MyProgram${normal}"
    echo 
    echo "    Output options for the clang++ compiler"
    echo "    ${bold}skm clang++ --help${normal}"
fi

