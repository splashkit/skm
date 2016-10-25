#!/bin/bash

if [[ `uname` == Darwin ]]; then
  echo "installing SplashKit for MacOS" 
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/master/install-scripts/mac-install.sh)
  echo "finished installing SplashKit for MacOS"
fi

if [[ `uname` == MINGW* ]]; then
  echo "installing SplashKit for Windows MingW"
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/master/install-scripts/windows-install.sh)
fi

if [[ `uname` == Linux ]]; then
  echo "installing SplashKit for Windows Linux"
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/master/install-scripts/linux-install.sh)
fi