#!/bin/bash

# Other device examples:
# - "iPhone 16"
# - "iPhone 15 Pro Max"
# - "iPhone SE (3rd generation)"
# - "iPad Pro (12.9-inch) (6th generation)"
# - "iPad Air (5th generation)"
DEVICE_NAME="iPhone 16 Pro"

xcrun simctl boot "$DEVICE_NAME"
open -a Simulator
cargo tauri ios dev "$DEVICE_NAME"

