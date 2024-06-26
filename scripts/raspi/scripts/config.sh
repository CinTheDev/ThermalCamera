#!/bin/bash

sudo apt update
sudo apt -y full-upgrade

sudo apt -y install git xorg xserver-xorg-video-fbturbo xserver-xorg-input-evdev x11-xserver-utils libxkbcommon-x11-dev lightdm ratpoison

# Login automatically as thermal-camera user
sudo raspi-config nonint do_boot_behaviour B4

sudo sed -i "s/autologin-user=pi/autologin-user=thermal-camera/g" /etc/lightdm/lightdm.conf
sudo sed -i "s/#xserver-command=X/xserver-command=X -s 0 dpms -nocursor/g" /etc/lightdm/lightdm.conf

# Disable ratpoison splash message
sudo touch /etc/ratpoisonrc
sudo bash -c 'echo "startup_message off" > /etc/ratpoisonrc'

# Move thermal-camera binary and run script
sudo mkdir /opt/thermal-camera
sudo mkdir /opt/thermal-camera/bin
sudo mv ./files/thermal-camera /opt/thermal-camera/bin/thermal-camera
sudo cp ./files/run_thermal_camera.sh /opt/thermal-camera/run_thermal_camera.sh

sudo chmod a+x /opt/thermal-camera/bin/thermal-camera
sudo chmod a+x /opt/thermal-camera/run_thermal_camera.sh

# Configure systemctl service
sudo cp ./files/thermal-camera.service /etc/systemd/system/thermal-camera.service
sudo systemctl enable thermal-camera
