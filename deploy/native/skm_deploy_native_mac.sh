#!/bin/sh

APP_PATH=`echo $0 | awk '{split($0,patharr,"/"); idx=1; while(patharr[idx+1] != "") { if (patharr[idx] != "/") {printf("%s/", patharr[idx]); idx++ }} }'`
APP_PATH=`cd "$APP_PATH"; pwd`

SKM_PATH=`cd "$APP_PATH/../.."; pwd`

source "${SKM_PATH}/tools/set_sk_env_vars.sh"

CURRENT_PATH=`pwd`
GAME_NAME=${CURRENT_PATH##*/}

GAMEAPP_PATH="./deploy/${GAME_NAME}.app"
if [ -d "${GAMEAPP_PATH}" ]; then
    rm -rf "${GAMEAPP_PATH}"
fi

if [ ! -d "./deploy" ]; then
    mkdir ./deploy
fi

echo "Creating Mac Application Bundle"

mkdir "${GAMEAPP_PATH}"
mkdir "${GAMEAPP_PATH}/Contents"
mkdir "${GAMEAPP_PATH}/Contents/MacOS"
mkdir "${GAMEAPP_PATH}/Contents/Frameworks"

echo "Copying Resources"
if [ -d "./Resources" ]; then
    cp -r -n "./Resources" "${GAMEAPP_PATH}/Contents/"
fi

echo "Copying executable"
cp "$2" "${GAMEAPP_PATH}/Contents/MacOS/"
cp "$SKM_PATH/lib/macos/libSplashKit.dylib" "${GAMEAPP_PATH}/Contents/MacOS/"

echo "Adding meta-data"
echo "<?xml version='1.0' encoding='UTF-8'?>\
<!DOCTYPE plist PUBLIC \"-//Apple Computer//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\
<plist version=\"1.0\">\
<dict>\
        <key>CFBundleDevelopmentRegion</key>\
        <string>English</string>\
        <key>CFBundleExecutable</key>\
        <string>$2</string>\
        <key>CFBundleIconFile</key>\
        <string>${ICON}</string>\
        <key>CFBundleIdentifier</key>\
        <string>io.splashkit.${GAME_NAME}</string>\
        <key>CFBundleInfoDictionaryVersion</key>\
        <string>6.0</string>\
        <key>CFBundleName</key>\
        <string>${GAME_NAME}</string>\
        <key>CFBundlePackageType</key>\
        <string>APPL</string>\
        <key>CFBundleSignature</key>\
        <string>SPLA</string>\
        <key>CFBundleVersion</key>\
        <string>1.0</string>\
        <key>CSResourcesFileMapped</key>\
        <true/>\
</dict>\
</plist>" >> "${GAMEAPP_PATH}/Contents/Info.plist"

echo "APPLSPLA" >> "${GAMEAPP_PATH}/Contents/PkgInfo"
