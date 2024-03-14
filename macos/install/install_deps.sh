#!/bin/bash

brew install pkgconfig sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng cmake
if [ $? -ne 0 ]; then
  echo "Failed to install dependencies - please ensure brew is installed and functional"
  exit $?
fi
