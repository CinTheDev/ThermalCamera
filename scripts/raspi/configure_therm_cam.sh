#!/bin/bash

# Check internet

if ! ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet connection required for configuration."
    exit 1
fi

# Create user

if ! "user" >/dev/null 2>&1; then
    echo "Creating user \"user\""
    sudo useradd user
    sudo passwd user
    sudo mkdir /home/user
else
    echo "User \"user\" already exists"
fi

(
    sudo su -l user

    whoami
    sudo whoami
)
