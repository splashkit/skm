#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

# Set Rust library path
RUST_LIB="${HOME}/.splashkit/cargo"

# Replace SPLASHKIT_PATH in Cargo.toml with actual path
if [ -f "Cargo.toml" ]; then
    if [ "$SK_OS" = "macos" ]; then
        sed -i '' "s|SPLASHKIT_PATH|${RUST_LIB}|g" Cargo.toml
    else
        sed -i "s|SPLASHKIT_PATH|${RUST_LIB}|g" Cargo.toml
    fi
fi
