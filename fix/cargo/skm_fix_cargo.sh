#!/bin/bash

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

# Create src directory if it doesn't exist
if [ ! -d src ]; then
    mkdir src
fi

# Replace SPLASHKIT_PATH in Cargo.toml with actual path
SPLASHKIT_RUST_PATH="${HOME}/.splashkit/cargo"
sed -i "s|SPLASHKIT_PATH|${SPLASHKIT_RUST_PATH}|" Cargo.toml
