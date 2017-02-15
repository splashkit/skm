#!/bin/bash
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

command -v git >/dev/null 2>&1 || { echo "Developer tools not installed, please run: \"xcode-select --install\" in the terminal and then rerun this script." >&2; exit;}

git clone --depth 1 --branch master $GIT_MACOS_REPO "${INSTALL_PATH}"

# Add SKM app to path without needing sudo
echo "export PATH=\"$INSTALL_PATH/skm-darwin-x64/skm.app/Contents/MacOS:$PATH\"" >> ~/.bash_profile

if [ -f ~/.zshrc ]; then
    echo "export PATH=\"$INSTALL_PATH/skm-darwin-x64/skm.app/Contents/MacOS:$PATH\"" >> ~/.zshrc
fi

export PATH="$INSTALL_PATH/skm-darwin-x64/skm.app/Contents/MacOS:$PATH"
echo "SplashKit Successfully installed! Please restart your terminal..."
