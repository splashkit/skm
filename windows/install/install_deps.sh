#!/bin/bash

# Get current installed packages
INSTALLED_PACKAGES=$(pacman -Qeq)

# Create string for packages to install
package_install_string=""

# Package list based on MSYS2 terminal environment or arm64 vs x64 architecture
if [[ $MSYSTEM == "MINGW64" ]] || [[ $(uname) != *ARM64 ]]; then
    SHELL_NAME="MINGW64"
    REQUIRED_PACKAGES=(
        "mingw-w64-x86_64-clang"
        "mingw-w64-x86_64-gcc"
        "mingw-w64-x86_64-gdb"
        "mingw-w64-x86_64-cmake"
        "mingw-w64-x86_64-SDL2"
        "mingw-w64-x86_64-SDL2_gfx"
        "mingw-w64-x86_64-SDL2_mixer"
        "mingw-w64-x86_64-SDL2_image"
        "mingw-w64-x86_64-SDL2_ttf"
        "mingw-w64-x86_64-SDL2_net"
        "mingw-w64-x86_64-civetweb"
        "mingw-w64-x86_64-python"
        "mingw-w64-x86_64-python-pip"
        "make"
        "mingw-w64-x86_64-jq"
        "moreutils"
    )
elif [[ $MSYSTEM == "CLANGARM64" ]] || [[ $(uname) == *ARM64 ]]; then
    SHELL_NAME="CLANGARM64"
    REQUIRED_PACKAGES=(
        "mingw-w64-clang-aarch64-clang"
        "mingw-w64-clang-aarch64-llvm"
        "mingw-w64-clang-x86_64-gdb-multiarch"
        "mingw-w64-clang-aarch64-cmake"
        "mingw-w64-clang-aarch64-SDL2"
        "mingw-w64-clang-aarch64-SDL2_gfx"
        "mingw-w64-clang-aarch64-SDL2_mixer"
        "mingw-w64-clang-aarch64-SDL2_image"
        "mingw-w64-clang-aarch64-SDL2_ttf"
        "mingw-w64-clang-aarch64-SDL2_net"
        "mingw-w64-clang-aarch64-civetweb"
        "mingw-w64-clang-x86_64-python"
        "mingw-w64-clang-aarch64-python-pip"
        "make"
        "mingw-w64-clang-aarch64-jq"
        "moreutils"
    )
fi

case $MSYSTEM in
# Previous version (for reference):
# MINGW64)
#     echo Installing the necessary $MSYSTEM pacman packages for SplashKit...
#     echo
#     echo You are about to install the dependencies using the following command:
#     echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
#     echo
#     pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
#     ;;
# CLANGARM64)
#     echo Installing the necessary $MSYSTEM pacman packages for SplashKit...
#     echo
#     echo You are about to install the dependencies using the following command:
#     echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
#     echo
#     pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
#     ;;
# *)
#     echo Unable to install the necessary pacman packages
#     if [[ $(uname) == *ARM64 ]]; then
#         echo Run the following command in the "$SHELL_NAME" terminal to install the required pacman packages for SplashKit:
#         echo
#         echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-clang-aarch64-clang mingw-w64-clang-aarch64-llvm mingw-w64-clang-x86_64-gdb-multiarch mingw-w64-clang-aarch64-cmake mingw-w64-clang-aarch64-SDL2 mingw-w64-clang-aarch64-SDL2_gfx mingw-w64-clang-aarch64-SDL2_mixer mingw-w64-clang-aarch64-SDL2_image mingw-w64-clang-aarch64-SDL2_ttf mingw-w64-clang-aarch64-SDL2_net mingw-w64-clang-aarch64-civetweb mingw-w64-clang-x86_64-python mingw-w64-clang-aarch64-python-pip make mingw-w64-clang-aarch64-jq moreutils
#     else
#         echo Run the following command in the "MINGW64" terminal to install the required pacman packages for SplashKit:
#         echo
#         echo pacman -S --needed --noconfirm --disable-download-timeout mingw-w64-x86_64-clang mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-cmake mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_gfx mingw-w64-x86_64-SDL2_mixer mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_net mingw-w64-x86_64-civetweb mingw-w64-x86_64-python mingw-w64-x86_64-python-pip make mingw-w64-x86_64-jq moreutils
#     fi
#     echo
#     exit 1
#     ;;
MINGW64 | CLANGARM64)
    # Add package string to "package_install_string" if packages not installed
    for package in "${REQUIRED_PACKAGES[@]}"; do
        if ! echo "$INSTALLED_PACKAGES" | grep -xFq "$package"; then
            package_install_string="$package_install_string $package"
        fi
    done

    # Run install command if package string is not empty
    if [ ! -z "$package_install_string" ]; then
        echo Installing the required $MSYSTEM pacman package dependencies using the following command:
        echo pacman -S --needed --noconfirm --disable-download-timeout $package_install_string
        echo
        pacman -S --needed --noconfirm --disable-download-timeout $package_install_string
    fi
    ;;
*)
    echo Unable to install the required pacman packages...
    echo Run the following command in the "$SHELL_NAME" terminal to install the required pacman packages for SplashKit:
    echo
    package_install_string="${REQUIRED_PACKAGES[*]}"
    echo pacman -S --needed --noconfirm --disable-download-timeout $package_install_string
    echo
    exit 1
    ;;
esac

echo "SplashKit Dependencies Installed"
