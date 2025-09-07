#!/bin/bash

# Install homebrew if not installed
if ! command -v brew &>/dev/null; then
    echo "\"brew\" command not found."
    echo "Installing homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

    # Add homebrew to shell profile
    echo "Adding \"eval \"\$(/opt/homebrew/bin/brew shellenv)\" to your shell profile"

    # Add to .bashrc if using bash
    if [[ ${SHELL} = "/bin/bash" ]] || [[ ${SHELL} = "/usr/bin/bash" ]]; then
        grep -Fqx "eval \"\$(/opt/homebrew/bin/brew shellenv)\"" ~/.bashrc || echo "eval \"\$(/opt/homebrew/bin/brew shellenv)\"" >>~/.bashrc
        source ~/.bashrc
    fi

    # Add to .zshrc if using zsh
    if [[ ${SHELL} = "/bin/zsh" ]] || [[ ${SHELL} = "/usr/bin/zsh" ]]; then
        grep -Fqx "eval \"\$(/opt/homebrew/bin/brew shellenv)\"" ~/.zshrc || echo "eval \"\$(/opt/homebrew/bin/brew shellenv)\"" >>~/.zshrc
        source ~/.zshrc
    fi
fi

# Double check brew command is available
if command -v brew &>/dev/null; then
    echo Installing the necessary brew packages for SplashKit...
    echo
    echo You are about to install the dependencies using the following command:
    echo brew install pkgconfig sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng cmake jq sponge
    brew install pkgconfig sdl2 sdl2_ttf sdl2_image sdl2_net sdl2_mixer sdl2_gfx libpng cmake jq sponge
    if [ $? -ne 0 ]; then
        echo "Failed to install dependencies - please ensure brew is installed and functional"
        exit $?
    fi
else
    echo Unable to install the necessary brew packages.
    echo Please install homebrew and then run the following command:
    echo
    echo skm macos install
fi
