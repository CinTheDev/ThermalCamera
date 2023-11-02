#!/bin/bash

echo "Updating system..."

sudo apt update
sudo -y apt full-upgrade

echo "Installing X"

sudo -y apt install git xorg xserver-xorg-video-fbturbo lightdm ratpoison

sudo raspi-config nonint do_boot_behaviour B4
