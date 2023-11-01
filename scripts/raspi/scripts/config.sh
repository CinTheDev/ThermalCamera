#!/bin/bash

# THIS SCRIPT SHOULD ONLY BE EXECUTED LOGGED IN AS "user"

# Delete other user (optionally)
# TODO

echo "Configuration yeah"
source ~/.bashrc

echo "Updating system..."

sudo apt update
sudo apt full-upgrade

echo "Installing X"

sudo apt install xauth xorg eog
