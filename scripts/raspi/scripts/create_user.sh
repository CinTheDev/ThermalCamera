#!/bin/bash

USER_NAME="thermal-camera"

# Create user
if ! id "$USER_NAME" >/dev/null 2>&1; then
    echo "Creating user \"$USER_NAME\""
    sudo useradd $USER_NAME
    sudo passwd $USER_NAME
    sudo mkdir "/home/$USER_NAME"
    sudo cp ~/.Xauthority "/home/$USER_NAME/.Xauthority"
else
    echo "User \"$USER_NAME\" already exists"
fi
