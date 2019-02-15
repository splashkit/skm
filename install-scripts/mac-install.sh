#!/bin/bash
GIT_SKM_REPO=https://github.com/splashkit/skm
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

command -v git >/dev/null 2>&1 || { echo "Developer tools not installed, please run: \"xcode-select --install\" in the terminal and then rerun this script." >&2; exit 1;}

git clone --depth 1 --branch master $GIT_SKM_REPO "${INSTALL_PATH}"

# Add SKM app to path without needing sudo
echo "export PATH=\"$INSTALL_PATH:$PATH\"" >> ~/.bash_profile

git clone --depth 1 --branch master $GIT_MACOS_REPO "${INSTALL_PATH}/lib/macos"

if [ -f ~/.zshrc ]; then
    echo "export PATH=\"$INSTALL_PATH:$PATH\"" >> ~/.zshrc
fi

export PATH="$INSTALL_PATH:$PATH"
echo "SplashKit Successfully installed! Please restart your terminal..."
