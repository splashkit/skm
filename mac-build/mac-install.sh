#!/bin/bash
GIT_SKM_REPO=https://github.com/jakerenzella/skm
GIT_MACOS_REPO=https://github.com/splashkit/splashkit-macos

HOME_PATH=~
FOLDER_PATH="${HOME_PATH}/.splashkit"
INSTALL_PATH="${FOLDER_PATH}/splashkit-macos"
INSTALL_SKM_PATH="${FOLDER_PATH}/skm"

# Check for clang

# command -v clang >/dev/null 2>&1 || { echo "SplashKit will install, but we can not find a clang compiler. Please run Xcode developer command."}
if which clang++ >/dev/null; then
  CLANG_EXISTS=true
else
  "SplashKit will install, but we can not find a clang compiler. Please run Xcode developer command."
fi

# Clone the repos if git is found.
if which git >/dev/null; then
    echo 'Git found'
   echo  git clone --depth 1 $GIT_MACOS_REPO "${INSTALL_PATH}"
    echo git clone -b master --depth 1 --single-branch $GIT_SKM_REPO "${INSTALL_SKM_PATH}"
  else
    echo 'Git not found, can''t install splashkit. Please run xcode developer command.'
fi

# Unzip the SKM app.
unzip "$INSTALL_SKM_PATH/mac-build/skm.zip" -d "$INSTALL_SKM_PATH/mac-build" > "$FOLDER_PATH/install.log"


# Add SKM app to path without needing sudo

echo "export PATH=\""$HOME/.splashkit/skm/mac-build/skm.app/Contents/MacOs:\$PATH\""" >> ~/.bash_profile

if [ -f ~/.zshrc ]; then
    echo "export PATH=\""$HOME/.splashkit/skm/mac-build/skm.app/Contents/MacOs:\$PATH\""" >> ~/.zshrc
fi

export PATH=\""$HOME/.splashkit/skm/mac-build/skm.app/Contents/MacOs:\$PATH\""

echo "SplashKit Successfully installed! Please restart your terminal..."
