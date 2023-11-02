#!/bin/bash

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
