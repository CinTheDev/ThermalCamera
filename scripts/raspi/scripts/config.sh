#!/bin/bash

echo "Updating system..."

sudo apt update
sudo apt -y full-upgrade

echo "Installing X"

sudo apt -y install git xorg xserver-xorg-video-fbturbo lightdm ratpoison

sudo raspi-config nonint do_boot_behaviour B4
