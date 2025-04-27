#!/bin/bash

export DISTRO_ID="UNKNOWN"
export DISTRO_VERSION="UNKNOWN"

to_upper() {
  echo $@ | tr [a-z] [A-Z]
}

detect_msys2_shell() {
  if command -v pacman &>/dev/null; then
    if [ "$MSYSTEM" = "MINGW64" ]; then
      export DISTRO_ID=$MSYSTEM
    else
      echo "Unable to detect Windows version. Please run in \"MINGW64\" terminal (not MSYS2 or UCRT terminals)"
    fi
  else
    echo "Unable to install dependencies in Git Bash terminal"
    echo "Please run in MINGW64 terminal"
  fi
}

install_deps() {
  case $1 in
  MINGW64)
    echo Installing the necessary $1 pacman packages for SplashKit...
    echo
    echo You are about to install the dependencies using the following command:
    echo pacman -S --needed --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
    echo
    pacman -S --needed --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
    ;;
  *)
    echo Unable to install the necessary pacman packages
    echo Run the following command in the "MINGW64" terminal to install the required pacman packages for SplashKit:
    echo
    echo pacman -S --needed --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
    echo
    ;;
  esac
}

detect_msys2_shell
install_deps $DISTRO_ID
