#!/bin/bash

echo "Updating system..."

sudo apt update
sudo apt full-upgrade

echo "Installing X"

sudo apt install git xauth xorg eog
