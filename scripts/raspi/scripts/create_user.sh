#!/bin/bash

# Check internet

if ! ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet connection required for configuration."
    exit 1
fi

# Create user

if ! id "user" >/dev/null 2>&1; then
    echo "Creating user \"user\""
    sudo useradd user
    sudo passwd user
    sudo usermod -aG sudo user
    sudo mkdir /home/user
else
    echo "User \"user\" already exists"
fi

# Sync files with new user
sudo cp -r . /home/user/thermal-camera
sudo cp ~/.bashrc /home/user/.bashrc
sudo chmod -R a+rw /home/user/thermal-camera/
sudo su -l user -c "cd thermal-camera && ./config.sh"
