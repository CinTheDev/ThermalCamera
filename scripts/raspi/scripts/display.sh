#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"
(
    cd $DRIVERS_PATH

    # To prevent the display drivers from restarting the whole system
    sudo chmod 0 /sbin/reboot

    sudo chmod +x ./LCD35-show
    # For some reason, directly installing the lite variant leads to
    # touch drivers not being installed, so this has to be called twice.
    sudo ./LCD35-show
    sudo ./LCD35-show lite

    sudo chmod 0755 /sbin/reboot
)

# Calibration
sudo cp ./files/40-libinput.conf /usr/share/X11/xorg.conf.d/40-libinput.conf

# Increase framerate
sudo sed -i "s/dtoverlay=waveshare35a/dtoverlay=waveshare35a:speed=20000000,fps=30/g" /boot/config.txt
