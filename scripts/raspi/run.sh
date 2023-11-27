#!/bin/bash

# Run all scripts inside scripts/ in order

YELLOW='\033[1;33m'
NC='\033[0m'

# Check internet
if ! ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet connection required for configuration."
    exit 1
fi

# Make all scripts executable
chmod -R +x ./scripts

echo -e "${YELLOW}Creating user...${NC}"
./scripts/create_user.sh

echo -e "${YELLOW}Configuring X server and thermal-camera binary...${NC}"
./scripts/config.sh

echo -e "${YELLOW}Configuring USB automount...${NC}"
./scripts/automount.sh

echo -e "${YELLOW}Installing display drivers...${NC}"
./scripts/display.sh

echo -e "${YELLOW}Configuring I2C...${NC}"
./scripts/enable_i2c.sh

echo -e "${YELLOW}System will be rebooted now${NC}"
sudo reboot
