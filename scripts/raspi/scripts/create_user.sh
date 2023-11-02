#!/bin/bash

USER_NAME="thermal-camera"

# Create user
if ! id "$USER_NAME" >/dev/null 2>&1; then
    echo "Creating user \"$USER_NAME\""
    sudo useradd $USER_NAME
    sudo passwd $USER_NAME
    #sudo usermod -aG sudo user
    #sudo mkdir /home/user
else
    echo "User \"$USER_NAME\" already exists"
fi
