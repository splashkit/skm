#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)
SKM_PATH=$(cd "$APP_PATH/../.." && pwd)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

# Terminal text highlighting
LIGHT_BLUE='\033[38;2;157;220;253m'
DARK_BLUE='\033[38;2;86;156;214m'
ORANGE='\033[38;2;206;145;120m'
MAGENTA='\033[38;2;218;112;214m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# ------------------------------
# Check required packages
# ------------------------------

# Check for "jq" and "sponge" commands
if ! command -v jq &>/dev/null || ! command -v sponge &>/dev/null; then
    # Install required packages
    if [ "$SK_OS" = "macos" ]; then
        "${SKM_PATH}/macos/install/install_deps.sh"
    elif [ "$SK_OS" = "linux" ]; then
        "${SKM_PATH}/linux/install/install_deps.sh"
    elif [ "$SK_OS" = "win64" ]; then
        "${SKM_PATH}/windows/install/install_deps.sh"
    fi
fi

# ------------------------------
# Set path to settings.json
# ------------------------------

SETTINGS_JSON_PATH=""
if [ "$SK_OS" = "macos" ]; then
    SETTINGS_JSON_PATH=$(cd "/Users/$(whoami)/Library/Application Support/Code/User" && pwd)
elif [ "$SK_OS" = "linux" ]; then
    # Check if using WSL
    if [ -d "/mnt/c/Users" ]; then
        WIN_USER=$(powershell.exe -Command "[System.Environment]::UserName" | sed 's/\r$//')
        SETTINGS_JSON_PATH="/mnt/c/Users/$WIN_USER/AppData/Roaming/Code/User"
    else
        SETTINGS_JSON_PATH="$HOME/.config/Code/User"
    fi
elif [ "$SK_OS" = "win64" ]; then
    SETTINGS_JSON_PATH="$APPDATA\\Code\\User"
fi

echo "Updating VS Code settings to add useful settings for coding with SplashKit..."
echo

# Will add later
# ------------------------------
# Print list of settings to add
# ------------------------------

# echo "The following settings will be added:"

# # OS-specific settings
# if [ "$SK_OS" = "macos" ]; then
#     : # no specific settings currently
# elif [ "$SK_OS" = "linux" ]; then
#     : # no specific settings currently
# elif [ "$SK_OS" = "win64" ]; then
#     : # still to add
# fi

# # Common settings
# echo -e "  $LIGHT_BLUE\"github.copilot.enable\"$NC:$MAGENTA {\n      $LIGHT_BLUE\"*\"$NC:$DARK_BLUE false\n  $MAGENTA}$NC,"
# echo -e "  $LIGHT_BLUE\"files.autoSave\"$NC:$ORANGE \"afterDelay\"$NC,"
# echo -e "  $LIGHT_BLUE\"editor.formatOnSave\"$NC:$DARK_BLUE true$NC"
# echo

# ------------------------------
# Update temp_settings.json file
# ------------------------------

# Make temporary file for error checking
cp "$SETTINGS_JSON_PATH/settings.json" "$APP_PATH"
if [ ! $? -eq 0 ]; then
    echo -e "${RED}Failed to copy settings.json to $APP_PATH${NC}"
    exit 1
fi

# Make backup settings file for recovery (in case of issues)

echo "Creating \"backup_settings.json\" file to recover your previous settings if something goes wrong."
cp "$APP_PATH/settings.json" "$APP_PATH/backup_settings.json"
if [ ! $? -eq 0 ]; then
    echo -e "${RED}Failed to copy $APP_PATH/settings.json to $APP_PATH/backup_settings.json${NC}"
    exit 1
fi
echo "Backup file location: $APP_PATH/backup_settings.json."
echo

# ------------------------------
# Add OS-specific settings
# ------------------------------

if [ "$SK_OS" = "macos" ]; then
    : # no specific settings currently
elif [ "$SK_OS" = "linux" ]; then
    # Check if using WSL
    if [ -d "/mnt/c/Users" ]; then
        jq '.["security.allowedUNCHosts"] |= ["wsl.localhost"]' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    fi
elif [ "$SK_OS" = "win64" ]; then
    # Terminal profiles:
    # Powershell profile
    jq '.["terminal.integrated.profiles.windows"].PowerShell.source |= "PowerShell"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"].PowerShell.icon |= "terminal-powershell"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    # Command Prompt profile
    jq '.["terminal.integrated.profiles.windows"]."Command Prompt".path |= [
        "${env:windir}/Sysnative/cmd.exe",
        "${env:windir}/System32/cmd.exe"
    ]' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"]."Command Prompt".args |= []' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"]."Command Prompt".icon |= "terminal-cmd"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    # Git Bash profile
    if command -v git &>/dev/null; then
        jq '.["terminal.integrated.profiles.windows"]."Git Bash" |= {"source": "Git Bash"}' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    fi
    # MSYS2 profile
    jq '.["terminal.integrated.profiles.windows"]."MSYS2".path |= "C:/msys64/usr/bin/bash.exe"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"]."MSYS2".args |= ["--login", "-i"]' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"]."MSYS2".env.MSYSTEM |= "MINGW64"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.profiles.windows"]."MSYS2".env.CHERE_INVOKING |= "1"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    # Default profile
    jq '.["terminal.integrated.defaultProfile.windows"] |= "MSYS2"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

    # Terminal environment (for running debugger)
    jq '.["terminal.integrated.env.windows"].MSYSTEM |= "MINGW64"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
    jq '.["terminal.integrated.env.windows"].CHERE_INVOKING |= "1"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

    # Cpp default system include paths
    jq '.["C_Cpp.default.systemIncludePath"] |= [
        "C:/msys64/mingw64/bin",
        "C:/msys64/mingw64/include",
        "C:/msys64/mingw64/include/c++/15.1.0",
        "C:/msys64/mingw64/include/c++/15.1.0/x86_64-w64-mingw32",
        "C:/msys64/mingw64/include/c++/15.1.0/backward",
        "C:/msys64/mingw64/lib/gcc/x86_64-w64-mingw32/15.1.0/include",
        "C:/msys64/mingw64/lib/gcc/x86_64-w64-mingw32/15.1.0/include-fixed",
        "C:/msys64/mingw64/include/c++/15.2.0",
        "C:/msys64/mingw64/include/c++/15.2.0/x86_64-w64-mingw32",
        "C:/msys64/mingw64/include/c++/15.2.0/backward",
        "C:/msys64/mingw64/lib/gcc/x86_64-w64-mingw32/15.2.0/include",
        "C:/msys64/mingw64/lib/gcc/x86_64-w64-mingw32/15.2.0/include-fixed",
        "${default}"
    ]' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

    # Cpp compiler path
    jq '.["C_Cpp.default.compilerPath"] |= "C:/msys64/mingw64/bin/gcc.exe"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

    # Cpp intellisense mode
    jq '.["C_Cpp.default.intelliSenseMode"] |= "gcc-x64"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"
fi

# ------------------------------
# Add common settings (for all OS's)
# ------------------------------

# Disable github copilot
jq '.["github.copilot.enable"]."*" |= false' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

# Close the RHS "copilot" sidebar
jq '.["workbench.secondarySideBar.defaultVisibility"] |= "hidden"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

# Turn on auto save
jq '.["files.autoSave"] |= "afterDelay"' "$APP_PATH/settings.json" | sponge "$APP_PATH/settings.json"

# Format code when manually saving (and fix indentation to 4 spaces)
jq '.["editor.formatOnSave"] |= true' "$APP_PATH/settings.json" --indent 4 | sponge "$APP_PATH/settings.json"

# ------------------------------
# Check temp_settings.json file
# ------------------------------

# Empty file = error
if [ $(wc -c <"$APP_PATH/settings.json") -le 5 ]; then
    echo
    echo -e "${RED}Something went wrong... VS Code Settings not updated.${NC}"
    echo -e "${RED}Check the settings.json file in $SETTINGS_JSON_PATH for errors/warnings and try again.${NC}"
    echo
    exit 1
fi

# ------------------------------
# Copy file back to settings.json file
# ------------------------------

mv "$APP_PATH/settings.json" "$SETTINGS_JSON_PATH/settings.json"
if [ ! $? -eq 0 ]; then
    echo
    echo -e "${RED}Failed to copy $APP_PATH/settings.json to $SETTINGS_JSON_PATH/settings.json${NC}"
    exit 1
fi

echo "VS Code settings updated successfully!"
