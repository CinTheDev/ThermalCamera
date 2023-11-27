#!/bin/bash

USER_NAME="thermal-camera"

# Create user
if ! id "$USER_NAME" >/dev/null 2>&1; then
    echo "Creating user \"$USER_NAME\""
    sudo useradd $USER_NAME
    sudo passwd $USER_NAME
    sudo mkdir "/home/$USER_NAME"
    sudo chmod a+rw "/home/$USER_NAME"

    sudo usermod -a -G i2c "$USER_NAME"
else
    echo "User \"$USER_NAME\" already exists"
fi
