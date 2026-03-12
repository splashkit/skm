#!/bin/bash

APP_PATH=$(echo "$0" | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(
  cd "$APP_PATH"
  pwd
)

SKM_PATH=$(
  cd "$APP_PATH/../.."
  pwd
)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "macos" ] || (echo "${*}" | grep '\-\-no-os-detect'); then
  : # All good - no op and continue
elif [ "$SK_OS" = "linux" ] || [ "$SK_OS" = "win64" ]; then
  echo "macOS install only available on macOS"
  exit 1
else
  echo "Unable to detect operating system..."
  echo "If you are running macOS, you can bypass os detection with the '--no-os-detect' flag."
  exit 1
fi

"${APP_PATH}/install_deps.sh"
if [ $? -ne 0 ]; then
  exit $?
fi

export CC=$(which clang)
export CXX=$(which clang++)

echo "Adding llama.cpp"
cd "${SKM_PATH}/source"
mkdir -p "${SKM_PATH}/source/llama.cpp"
cd "${SKM_PATH}/source/llama.cpp"
if [ ! -d ".git" ]; then
  git init
  git remote add origin https://github.com/ggml-org/llama.cpp
fi

git fetch origin 8ea958d4d9d5bd43f9f244950bf057b5227ce8d9 && git reset --hard FETCH_HEAD

if [ $? -ne 0 ]; then
  echo "Failed to clone llama"
  exit $?
fi

echo "Configuring SplashKit"
cd "${SKM_PATH}/source"
pwd
cmake .
if [ $? -ne 0 ]; then
  echo "Configuration failed"
  exit $?
fi

echo "Compiling SplashKit..."
make
if [ $? -ne 0 ]; then
  echo "Compilation failed"
  exit $?
fi

echo "Installing compiled SplashKit library..."
make install
if [ $? -ne 0 ]; then
  echo "Install failed"
  exit $?
fi

echo "SplashKit Installed"

"${SKM_PATH}/global/install/skm_global_install.sh"
