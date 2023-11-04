#!/bin/bash

echo "Updating system..."

sudo apt update
sudo apt -y full-upgrade

echo "Installing X"

sudo apt -y install git xorg xserver-xorg-video-fbturbo lightdm ratpoison

sudo raspi-config nonint do_boot_behaviour B4

# Move thermal-camera binary
sudo mkdir /opt/thermal-camera
sudo cp ./files/thermal-camera /opt/thermal-camera/thermal-camera
sudo chmod a+x /opt/thermal-camera/thermal-camera

# Configure systemctl service
sudo cp ./files/thermal-camera.service /etc/systemd/system/thermal-camera.service
sudo systemctl enable thermal-camera
