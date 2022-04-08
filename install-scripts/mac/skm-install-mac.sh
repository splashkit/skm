#!/bin/bash
GIT_SKM_REPO=https://github.com/splashkit/skm.git
MARK_CHECK="\xE2\x9C\x94"
MARK_CROSS="\xE2\x9C\x95"

HOME_PATH=~
INSTALL_PATH="${HOME_PATH}/.splashkit"

function has_git() {
	git --help 2>&1 > /dev/null
	return $?
}

function install_git() {
	echo "macOS developer tools are currently not installed which are required for installation skm."
	read -p "Would you like to install them now? " -n 1 -r
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
has_git
if [ $? -ne 0 ]; then
	install_git
	has_git
	if [ $? -ne 0 ]; then
		printf "\t$MARK_CROSS Failed to install macOS developer tools. Please ensure you have \"git\" installed and try again.\n"
		exit 1
	fi
else
	printf "\t$MARK_CHECK macOS developer tools appears to be installed\n"
fi

echo "Checking for an existing version of SplashKit..."
if [ -d "${INSTALL_PATH}" ]; then
	printf "\t$MARK_CROSS SplashKit is already installed!\n"
	read -p "Would you like to completely re-install SplashKit? " -n 1 -r
	echo ""
	if [[ $REPLY =~ [Yy]$ ]]; then
		D=`date +%y%m%d-%H%M%S`
		OLD_PATH="$INSTALL_PATH-$D"
		echo "Removing existing SplashKit installation..."
		mv $INSTALL_PATH $OLD_PATH
		if [ -d "${INSTALL_PATH}" ]; then
			printf "\t$MARK_CROSS failed to remove existing SplashKit installation.\n"
			exit 1
		else
			printf "\t$MARK_CHECK removed existing SplashKit installation.\n"
		fi
	else
		printf "\t$MARK_CHECK SplashKit is already installed.\n"
		exit 1
	fi
fi

echo "Installing SplashKit..."
git clone --depth 1 --branch master $GIT_SKM_REPO "${INSTALL_PATH}"
if [ -d "${INSTALL_PATH}" ]; then
	printf "\t$MARK_CHECK SplashKit was installed.\n"
else
	printf "\t$MARK_CROSS failed to install SplashKit.\n"
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

printf "\t$MARK_CHECK done\n"
export PATH="$INSTALL_PATH:$PATH"

# Verify installation and access
SKM_PATH="${INSTALL_PATH}/skm"
if [ -f $SKM_PATH ]; then
	printf "$MARK_CHECK SplashKit installed successfully!\n"
else
	printf "$MARK_CROSS SplashKit installation failed!\n"
fi

find "${INSTALL_PATH}" -name "*.sh" -exec chmod a+x "{}" \;

which skm 2>&1 > /dev/null
if [ $? -ne 0 ]; then
	printf "$MARK_CROSS Failed to validate \"skm\" command.\n"
	exit 1
else
	printf "$MARK_CHECK Verified that \"skm\" can be accessed.\n"
fi

read -p "Would you like to install the recommended developer tools for SplashKit?: " -n 1 -r
echo ""
if [[ $REPLY =~ [Yy]$ ]]
then
	if [ -d "/Applications/Visual Studio Code.app" ]; then
		printf "\t$MARK_CHECK Microsoft Visual Studio Code is already installed\n"
	else
		echo "Installing Microsoft Visual Studio Code..."
		mkdir -p /tmp/.skm-tmp/vscode
		cd /tmp/.skm-tmp/vscode
		rm *.zip
		curl -LOJ "https://code.visualstudio.com/sha/download?build=stable&os=darwin-universal"
		unzip *.zip
		cp -R ./Visual\ Studio\ Code.app /Application
		if [ -d "/Applications/Visual Studio Code.app" ]; then
			printf "\t$MARK_CHECK Microsoft Visual Studio Code was installed\n"
		else
			printf "\t$MARK_CROSS Failed to install Microsoft Visual Studio Code\n"
		fi
	fi
fi

printf "$MARK_CHECK SplashKit installation process has been completed.\n"
echo "Type \"skm help\" to get started."