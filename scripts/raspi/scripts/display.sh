#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"

cd $DRIVERS_PATH

# To prevent the display drivers from restarting the whole system
sudo chmod 0 /sbin/reboot

sudo chmod +x ./LCD35-show
./LCD35-show lite

sudo chmod 0755 /sbin/reboot
