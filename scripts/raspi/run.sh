#!/bin/bash

# Run all scripts inside scripts/ in order

# Check internet
if ! ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet connection required for configuration."
    exit 1
fi

# TODO: Implement all scripts

./scripts/create_user.sh
./scripts/config.sh
./scripts/display.sh
# [Copy important files to where they belong]
# [Enable and config I2C]
# [Systemctl]
