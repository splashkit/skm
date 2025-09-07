#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)
SKM_PATH=$(cd "$APP_PATH/../.." && pwd)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

# Terminal text highlighting
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

VSCODE_PATH=""
# Set path to 'code' command
if ! command -v code &>/dev/null; then
    if [ "$SK_OS" = "macos" ]; then
        VSCODE_PATH="/Applications/Visual Studio Code.app/Contents/Resources/app/bin"
        # Back up in case they haven't moved Visual Studio Code to Applications
        if [ ! -f "$VSCODE_PATH/code" ]; then
            VSCODE_PATH="/usr/local/bin"
        fi
    elif [ "$SK_OS" = "linux" ]; then
        # Check if using WSL
        if [ -d "/mnt/c/Users" ]; then
            WIN_USER=$(powershell.exe -Command "[System.Environment]::UserName" | sed 's/\r$//')
            VSCODE_PATH=$(cd /mnt/c/Users/$WIN_USER/AppData/Local/Programs/Microsoft\ VS\ Code/bin && pwd)
        else
            VSCODE_PATH="/usr/bin"
            # Snap path (ubuntu)
            if [ ! -f "$VSCODE_PATH/code" ]; then
                VSCODE_PATH="/snap/bin"
            fi
        fi
    elif [ "$SK_OS" = "win64" ]; then
        VSCODE_PATH=$(cd $LOCALAPPDATA/Programs/Microsoft\ VS\ Code/bin && pwd)
    fi

    # Check if path to "code" command exists
    if [ ! -f "$VSCODE_PATH/code" ]; then
        echo -e "${RED}Unable to find path to \"code\" executable file.${NC}"
        echo -e "${RED}Make sure you have installed \"Visual Studio Code\", and then try again.${NC}"
        exit 1
    fi
else
    VSCODE_PATH="$(which code)"
    VSCODE_PATH=${VSCODE_PATH///code}
fi

# ------------------------------
# Install VS Code extensions
# ------------------------------

# Install "WSL" extension if not already installed
# Check if using WSL
if [ "$SK_OS" = "linux" ] && [ -d "/mnt/c/Users" ]; then
    if ! "$VSCODE_PATH/code" --list-extensions | grep -q "ms-vscode-remote.remote-wsl"; then
        echo "${BLUE}Installing \"WSL\" VS Code extension...${NC}"
        "$VSCODE_PATH/code" --install-extension ms-vscode-remote.remote-wsl
    fi
fi

# Install "C/C++ Extension Pack" extension if not already installed
if ! "$VSCODE_PATH/code" --list-extensions | grep -q "ms-vscode.cpptools-extension-pack"; then
    echo -e "${BLUE}Installing \"C/C++ Extension Pack\" VS Code extension...${NC}"
    "$VSCODE_PATH/code" --install-extension ms-vscode.cpptools-extension-pack
fi

# Install "C#"" extension if not already installed
if ! "$VSCODE_PATH/code" --list-extensions | grep -q "ms-dotnettools.csharp"; then
    echo -e "${BLUE}Installing \"C#\" VS Code extension...${NC}"
    "$VSCODE_PATH/code" --install-extension ms-dotnettools.csharp
fi

# Install "C# Dev Kit" extension if not already installed
if ! "$VSCODE_PATH/code" --list-extensions | grep -q "ms-dotnettools.csdevkit"; then
    echo -e "${BLUE}Installing \"C# Dev Kit\" VS Code extension...${NC}"
    "$VSCODE_PATH/code" --install-extension ms-dotnettools.csdevkit
fi

# Install "Python" extension if not already installed
if ! "$VSCODE_PATH/code" --list-extensions | grep -q "ms-python.python"; then
    echo -e "${BLUE}Installing \"Python\" VS Code extension...${NC}"
    "$VSCODE_PATH/code" --install-extension ms-python.python
fi

# No longer needed due to setting found to disable this extension
# # Uninstall "Intellicode for C# Dev Kit" extension if installed (disabling is only very temporary)
# if "$VSCODE_PATH/code" --list-extensions | grep -q "ms-dotnettools.vscodeintellicode-csharp"; then
#     echo -e "${BLUE}Uninstalling \"Intellicode for C# Dev Kit\" VS Code extension...${NC}"
#     "$VSCODE_PATH/code" --uninstall-extension ms-dotnettools.vscodeintellicode-csharp
# fi
