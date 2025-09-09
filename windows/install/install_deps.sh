#!/bin/bash

case $MSYSTEM in
MINGW64)
    echo Installing the necessary $MSYSTEM pacman packages for SplashKit...
    echo
    echo You are about to install the dependencies using the following command:
    echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
    echo
    pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
    ;;
CLANGARM64)
    echo Installing the necessary $MSYSTEM pacman packages for SplashKit...
    echo
    echo You are about to install the dependencies using the following command:
    echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
    echo
    pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
    ;;
*)
    echo Unable to install the necessary pacman packages
    if [[ $(uname) == *ARM64 ]]; then
        echo Run the following command in the "CLANGARM64" terminal to install the required pacman packages for SplashKit:
        echo
        echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
    else
        echo Run the following command in the "MINGW64" terminal to install the required pacman packages for SplashKit:
        echo
        echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
    fi
    echo
    ;;
esac

echo "SplashKit Dependencies Installed"
