#!/bin/bash

# Run all scripts inside scripts/ in order

# Check internet
if ! ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet connection required for configuration."
    exit 1
fi

# Make all scripts executable
chmod -R +x ./scripts

./scripts/create_user.sh
./scripts/config.sh
./scripts/display.sh
./scripts/enable_i2c.sh

sudo reboot
