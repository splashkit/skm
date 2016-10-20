#!/usr/bin/env bash
if [[ `uname` == Darwin; ]] then
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/develop/mac-build/mac-install.sh)
fi

if [[ `uname` == MINGW* ]]; then
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/develop/windows-build/windows-install.sh)
fi

if [[ `uname` == Linux ]]; then
  bash <(curl -s https://raw.githubusercontent.com/jakerenzella/skm/develop/linux-build/linux-install.sh)
fi