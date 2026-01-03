#!/bin/bash

# Other device examples:
# - "iPhone 16"
# - "iPhone 15 Pro Max"
# - "iPhone SE (3rd generation)"
# - "iPad Pro (12.9-inch) (6th generation)"
# - "iPad Air (5th generation)"
DEVICE_NAME="iPhone 16 Pro"

# Auto-detect Mac's IP and update tauri.ios.conf.json (localhost doesn't work on mobile)
# - SERVER_IP: tells iOS app where to connect
# - LEPTOS_SITE_ADDR=0.0.0.0: allows connections from iOS simulator
SERVER_IP=$(ipconfig getifaddr en0)
sed -i '' "s|http://[0-9.]*:3000|http://${SERVER_IP}:3000|g" src-tauri/tauri.ios.conf.json
echo "Updated SERVER_URL to http://${SERVER_IP}:3000"

xcrun simctl boot "$DEVICE_NAME"
open -a Simulator
LEPTOS_SITE_ADDR="0.0.0.0:3000" cargo tauri ios dev "$DEVICE_NAME"
