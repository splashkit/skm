#!/bin/bash

APP_PATH=$(echo "$0" | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(cd "$APP_PATH" && pwd)

SKM_PATH=$(cd "$APP_PATH/../.." && pwd)

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

if [ "$SK_OS" = "win64" ]; then
    : # All good - no op and continue
elif [ "$SK_OS" = "macos" ] || [ "$SK_OS" = "linux" ] || (echo "${*}" | grep '\-\-no-os-detect'); then
    echo "Windows install only available on Windows"
    exit 1
else
    echo "Unable to detect operating system..."
    exit 1
fi

if [[ $(uname) != *ARM64 ]]; then
    "${APP_PATH}/install_deps.sh"
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
cmake -G "Unix Makefiles" .
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
