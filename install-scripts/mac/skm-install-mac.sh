#!/bin/bash

set -eu
GIT_SKM_REPO=https://github.com/splashkit/skm.git
MARK_CHECK="\xE2\x9C\x94"
MARK_CROSS="\xE2\x9C\x95"

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

function has_git() {
    git --help 2>&1 > /dev/null
    return $?
}

function install_developer_tools() {
    echo "macOS developer tools are currently not installed which are required for installation skm."
    read -p "Would you like to install them now? " -n 1 -r < /dev/tty
    echo ""
    if [[ $REPLY =~ [Yy]$ ]]
    then
        xcode-select --install
    else
        echo "macOS developer tools not installed, please run: \"xcode-select --install\" in the terminal and then reinstall."
        exit 1
    fi
}

echo "Checking for macOS developer tools..."
if has_git; then
    echo -e "\t$MARK_CHECK macOS developer tools appears to be installed"
else
    install_developer_tools
    has_git
    if [ $? -ne 0 ]; then
        echo -e "\t$MARK_CROSS Failed to install macOS developer tools. Please ensure you have \"git\" installed and try again."
        exit 1
    fi
fi

echo "Checking for an existing version of SplashKit..."
if [ -d "${INSTALL_PATH}" ]; then
    echo -e "\t$MARK_CROSS SplashKit is already installed!"
    read -p "Would you like to completely re-install SplashKit? " -n 1 -r < /dev/tty
    echo ""
    if [[ $REPLY =~ [Yy]$ ]]; then
        D=`date +%y%m%d-%H%M%S`
        OLD_PATH="$INSTALL_PATH-$D"
        echo "Removing existing SplashKit installation..."
        mv $INSTALL_PATH $OLD_PATH
        if [ -d "${INSTALL_PATH}" ]; then
            echo -e "\t$MARK_CROSS failed to remove existing SplashKit installation."
            exit 1
        else
            echo -e "\t$MARK_CHECK removed existing SplashKit installation."
        fi
    else
        echo -e "\t$MARK_CHECK SplashKit is already installed."
        exit 1
    fi
fi

echo "Installing SplashKit..."
git clone --depth 1 --branch master $GIT_SKM_REPO "${INSTALL_PATH}"
if [ -d "${INSTALL_PATH}" ]; then
    echo -e "\t$MARK_CHECK SplashKit was installed."
else
    echo -e "\t$MARK_CROSS failed to install SplashKit."
    exit 1
fi

# Add to .bashrc if using bash and path line is missing for SplashKit inclusion.
if [ ${SHELL} = "/bin/bash" ]; then
    echo "Adding \"$INSTALL_PATH\" to your bash profile..."
    grep -Fqx "export PATH=\"$INSTALL_PATH:\$PATH\"" ~/.bash_profile || echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bash_profile
    grep -Fqx "export PATH=\"$INSTALL_PATH:\$PATH\"" ~/.bashrc || echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.bashrc
fi

# Add to .zshrc if using zsh and path line is missing for SplashKit inclusion.
if [ ${SHELL} = "/bin/zsh" ]; then
    echo "Adding \"$INSTALL_PATH\" to your ZSH profile..."
    grep -Fqx "export PATH=\"$INSTALL_PATH:\$PATH\"" ~/.zshrc || echo "export PATH=\"$INSTALL_PATH:\$PATH\"" >> ~/.zshrc
fi

echo -e "\t$MARK_CHECK Done"
export PATH="$INSTALL_PATH:$PATH"

# Verify installation and access
SKM_PATH="${INSTALL_PATH}/skm"
if [ -f $SKM_PATH ]; then
    echo -e "$MARK_CHECK SplashKit installed successfully!"
else
    echo -e "$MARK_CROSS SplashKit installation failed!"
fi

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

if which skm > /dev/null 2>&1; then
    echo -e "$MARK_CHECK Verified that \"skm\" can be accessed."
else
    echo -e "$MARK_CROSS Failed to validate \"skm\" command."
    exit 1
fi

echo -e "$MARK_CHECK SplashKit installation process has been completed."
echo "Type \"skm help\" to get started."
