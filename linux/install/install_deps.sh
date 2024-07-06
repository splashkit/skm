#!/bin/bash

export DISTRO_ID="UNKNOWN"
export DISTRO_VERSION="UNKNOWN"

to_upper () {
  echo $@ | tr [a-z] [A-Z]
}

detect_distro () {
  if [ -e /etc/os-release ]; then
    source /etc/os-release
    export DISTRO_ID=$(to_upper ${ID})
  elif (lsb_release); then
    export DISTRO_ID=$( to_upper $(lsb_release -is) )
  else
	if (apt-get 2>&1 | grep -v "command not found"); then
	  echo "We think you're using a Debian like distro"
	  export DISTRO_ID="DEBIAN"
	elif (pacman 2>&1 | grep -v "command not found"); then
	  echo "We think you're using a Arch Linux like distro"
	  export DISTRO_ID="ARCH"
	elif (dnf 2>&1 | grep -v "command not found"); then
	  echo "We think you're using a Fedora like distro"
	  export DISTRO_ID="FEDORA"
    else
      echo Unable to detect Linux distrobution
    fi
	export DISTRO_ID="UNKNOWN"
  fi

}

install_deps () {
  case $1 in
	ARCH )
	  echo Installing depencies with Arch Linux method
	  echo You are about to be prompt for your sudo password to install the dependencies using the following command:
	  echo   pacman -S --needed base-devel cmake libpng sdl2 sdl2_mixer sdl2_gfx sdl2_image sdl2_net sdl2_ttf libmikmod clang
	  sudo pacman -S --needed base-devel cmake libpng sdl2 sdl2_mixer sdl2_gfx sdl2_image sdl2_net sdl2_ttf libmikmod clang
	  ;;
    DEBIAN | UBUNTU | KALI | RASPBIAN | LINUXMINT )
	  echo Installing depencies with $1 method
	  echo You are about to be prompt for your sudo password to install the dependencies using the following command:
	  echo   apt-get install cmake libpng-dev libcurl4-openssl-dev libsdl2-dev libsdl2-mixer-dev libsdl2-gfx-dev libsdl2-image-dev libsdl2-net-dev libsdl2-ttf-dev libmikmod-dev libncurses5-dev libbz2-dev libflac-dev libvorbis-dev libwebp-dev libfreetype6-dev build-essential clang
	  sudo apt-get install cmake libpng-dev libcurl4-openssl-dev libsdl2-dev libsdl2-mixer-dev libsdl2-gfx-dev libsdl2-image-dev libsdl2-net-dev libsdl2-ttf-dev libmikmod-dev libncurses5-dev libbz2-dev libflac-dev libvorbis-dev libwebp-dev libfreetype6-dev build-essential clang
	  ;;
	FEDORA )
	  echo Installing depencies with Fedora method
	  echo You are about to be prompt for your sudo password to install the dependencies using the following command:
	  echo   dnf install cmake libpng-devel libcurl-devel SDL2-devel SDL2_mixer-devel SDL2_gfx-devel SDL2_image-devel SDL2_net-devel SDL2_ttf-devel libmikmod-devel ncurses-devel bzip2-devel flac-devel libvorbis-devel libwebp-devel freetype-devel clang
	  sudo dnf install cmake libpng-devel libcurl-devel SDL2-devel SDL2_mixer-devel SDL2_gfx-devel SDL2_image-devel SDL2_net-devel SDL2_ttf-devel libmikmod-devel ncurses-devel bzip2-devel flac-devel libvorbis-devel libwebp-devel freetype-devel clang
	  ;;
	*)
      echo Unable to install dependency;
	  echo Going ahead with SplashKit install.
	  echo If unsuccessful, ensure you have the following dependencies installed:
	  echo   CMake, libpng, SDL2, SDL2_mixer, SDL2_gfx, SDL2_image, SDL_net, SDL2_ttf,
	  echo   libmikmod, ncurses, bzip2, flac, vorbis, webp, freetype, clang

	  ;;
  esac
}

detect_distro
install_deps $DISTRO_ID
