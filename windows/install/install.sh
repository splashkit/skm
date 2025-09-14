#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

# # Suppress the splashkit not found error (we're installing splashkit)
# mkdir -p "$SKM_PATH/lib/linux"

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "win64" ]; then
  : # All good - no op and continue
elif [ "$SK_OS" = "macos" ] || [ "$SK_OS" = "linux" ] || ( echo "${*}" | grep '\-\-no-os-detect' ); then
  echo "Windows install only available on Windows"
  exit 1
else
    echo "Unable to detect operating system..."
    exit 1
fi

"${APP_PATH}/install_deps.sh"

# enable the integrated MINGW64 terminal in VS Code to update the .bash_history file
grep -Fqx "PROMPT_COMMAND='history -a'" ~/.bashrc || echo "PROMPT_COMMAND='history -a'" >>~/.bashrc
source ~/.bashrc

# Libary building to come later

# echo "Configuring SplashKit"
# cd "${SKM_PATH}/source"
# pwd
# cmake -G "Unix Makefiles" .
# if [ $? -ne 0 ]; then
#   echo "Configuration failed"
#   exit $?
# fi

# echo "Compiling SplashKit..."
# make
# if [ $? -ne 0 ]; then
#   echo "Compilation failed"
#   exit $?
# fi

# echo "Installing compiled SplashKit library..."
# make install
# if [ $? -ne 0 ]; then
#   echo "Install failed"
#   exit $?
# fi

# echo "SplashKit Installed"

echo "SplashKit Dependencies Installed"

"${SKM_PATH}/global/install/skm_global_install.sh"
