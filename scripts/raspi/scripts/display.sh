#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/goodtft/LCD-show.git "$DRIVERS_PATH"
sudo chmod -R 755 "$DRIVERS_PATH"
(
    cd $DRIVERS_PATH

    # To prevent the display drivers from restarting the whole system
    sudo chmod 0 /sbin/reboot

    sudo ./LCD35-show

    sudo chmod 0755 /sbin/reboot
)

# Calibration
sudo cp ./files/40-libinput.conf /usr/share/X11/xorg.conf.d/40-libinput.conf
sudo cp ./files/99-calibration.conf /usr/share/X11/xorg.conf.d/99-calibration.conf

# Increase framerate
sudo sed -i "s/dtoverlay=waveshare35a/dtoverlay=waveshare35a:speed=41000000,fps=60/g" /boot/config.txt
