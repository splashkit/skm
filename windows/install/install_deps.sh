#!/bin/bash

export DISTRO_ID="UNKNOWN"
export DISTRO_VERSION="UNKNOWN"

to_upper () {
  echo $@ | tr [a-z] [A-Z]
}

detect_msys2_shell () {
	if command -v pacman &> /dev/null; then
		# if [ "$MSYSTEM" = "MINGW64" ] || [ "$MSYSTEM" = "CLANG64" ] || [ "$MSYSTEM" = "CLANGARM64" ]; then
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

install_deps () {
  case $1 in
	MINGW64 )
	  echo Installing depencies with $1 method
	  echo You are about to install the dependencies using the following command:
	  echo   pacman -S --needed --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
	  pacman -S --needed --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
	  ;;
	# CLANGARM64 )
	#   echo Installing depencies with $1 method
	#   echo You are about to install the dependencies using the following command:
	#   echo   pacman -S --needed mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-gcc mingw-w64-clang-aarch64-gdb mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb
	#   pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb
	#   ;;
	*)
    echo Unable to install dependency;
	  echo Going ahead with SplashKit install.
	  echo If unsuccessful, ensure you have the following dependencies installed:
	  echo   CMake, libpng, SDL2, SDL2_mixer, SDL2_gfx, SDL2_image, SDL_net, SDL2_ttf,
	  echo   libmikmod, ncurses, bzip2, flac, vorbis, webp, freetype, clang

	  ;;
  esac
}

detect_msys2_shell
install_deps $DISTRO_ID
