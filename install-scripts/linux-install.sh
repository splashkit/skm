#!/bin/bash
GIT_LINUX_REPO=https://github.com/splashkit/splashkit-linux

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

declare -a DEPENDENCIES=("libsdl2-dev" "libsdl2-gfx-dev" "libsdl2-mixer-dev" "libsdl2-ttf-dev" "libsdl2-net-dev" "libsdl2-image-dev" "libncurses5-dev" "libpng12-dev" "libcurl4-openssl-dev")

command -v git >/dev/null 2>&1 || { echo "Please install git using your package manager For example: sudo apt install git" >&2; exit;}

# Add SKM app to path without needing sudo
echo "SKM and SplashKit depends on the following libraries:
    * sdl2 development library
    * sdl2 gfx development library
    * sdl2 mixer development library
    * sdl2 ttf development library
    * sdl2 net development library
    * sdl2 image development library
    * ncurses development library
    * png development library
    * curl4 openssl development library
    * bz2 development library
    * flac, libvorbis libmikmod development libraries
    * webp development library
    * freetype development library
    * CMAKE

    Please ensure these dependencies are installed using your package manager.
    For example:

    sudo apt install libsdl2-dev libsdl2-gfx-dev libsdl2-mixer-dev libsdl2-ttf-dev libsdl2-net-dev libsdl2-image-dev libncurses-dev libpng-dev libcurl4-openssl-dev libbz2-dev libflac-dev libvorbis-dev libmikmod-dev libwebp-dev libfreetype6-dev cmake

"

while true
do
  read -p "Have you installed these dependencies? [y/n]: " answer
  case $answer in
    [yY]* ) echo "Okay, installing SKM and SplashKit."
            break;;

    [nN]* ) echo "Please install these dependencies and run this script again."
            exit;;

    * )     echo "Enter Y or N, please.";
  esac
done

git clone --depth 1 --branch master $GIT_LINUX_REPO "${INSTALL_PATH}"

echo "
SKM and SplashKit has been installed. please ensure SKM is added to the PATH.
    SKM is located at: $INSTALL_PATH/splashkit-linux/skm-linux-x64

    For example: echo export PATH="$INSTALL_PATH/skm-linux-x64:$PATH" >> ~/.bash_profile"