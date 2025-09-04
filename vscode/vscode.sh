#!/bin/bash

APP_PATH=$(echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }')
APP_PATH=$(
    cd "$APP_PATH"
    pwd
)

case $1 in
settings)
    "${APP_PATH}/settings/skm_vscode_settings.sh" $*
    ;;
extensions)
    "${APP_PATH}/extensions/skm_vscode_extensions.sh" $*
    ;;
*)
    echo "Unknown option for skm vscode. "
    ;;
esac
