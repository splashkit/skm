#!/bin/bash

if [[ `uname` == Darwin ]]; then
  echo "installing SplashKit for MacOS" 
  bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/mac-install.sh)
  echo "finished installing SplashKit for MacOS"
  exit
fi

if [[ `uname` == MINGW* ]] || [[ `uname` == MSYS* ]]; then
  echo "installing SplashKit for Windows MingW"
  bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/windows-install.sh)
  echo "finished installing SplashKit for Windows (MINGW)"
  exit
fi

if [[ `uname` == Linux ]]; then
  echo "installing SplashKit for Linux"
  bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/linux-install.sh)
  echo "finished installing SplashKit for Linux"
  exit
fi

echo "SplashKit is not supported on this Operating System. If using Windows, ensure you install SKM in a MinGW Terminal."