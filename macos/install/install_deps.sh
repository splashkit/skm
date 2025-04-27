#!/bin/bash

if command -v brew &> /dev/null; then
    echo Installing the necessary brew packages for SplashKit...
    echo
    echo You are about to install the dependencies using the following command:
    echo brew install pkgconfig sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng cmake
    brew install pkgconfig sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng cmake
    if [ $? -ne 0 ]; then
    echo "Failed to install dependencies - please ensure brew is installed and functional"
    exit $?
    fi
else
    echo Unable to install the necessary brew packages.
    echo Please install homebrew and then run the following command:
    echo
    echo skm macos install
fi
