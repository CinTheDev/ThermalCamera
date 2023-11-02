#!/bin/bash

echo "Updating system..."

sudo apt update
sudo apt full-upgrade

echo "Installing X"

sudo apt install git xorg eog

# NOTE: In order to user startx, I had to:
# sudo mv /usr/share/X11/xorg.conf.d/99-fbturbo.conf ~
