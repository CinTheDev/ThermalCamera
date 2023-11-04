#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"
(
    cd $DRIVERS_PATH

    # To prevent the display drivers from restarting the whole system
    sudo chmod 0 /sbin/reboot

    sudo chmod +x ./LCD35-show
    ./LCD35-show lite

    sudo chmod 0755 /sbin/reboot
)

# Calibration
sudo cp ./files/99-calibration.conf /usr/share/X11/xorg.conf.d/99-calibration.conf

# Increase framerate
sudo sed -i "s/dtoverlay=waveshare35a/dtoverlay=waveshare35a:speed=20000000,fps=30/g" /boot/config.txt
