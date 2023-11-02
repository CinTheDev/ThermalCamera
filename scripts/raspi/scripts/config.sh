#!/bin/bash

echo "Updating system..."

sudo apt update
sudo apt full-upgrade

echo "Installing X"

sudo apt install git xorg xserver-xorg-video-fbturbo lightdm eog

sudo raspi-config nonint do_boot_behaviour B4

# Configure fbturbo driver to use fb1
sudo cp ./files/99-fbturbo.conf /usr/share/X11/xorg.conf.d/99-fbturbo.conf
