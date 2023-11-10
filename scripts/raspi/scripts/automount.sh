#!/bin/bash

# Since installing usbmount from apt doesn't work for some reason, we'll have to compile it from source

git clone https://github.com/rbrito/usbmount.git ~/usbmount/source
cd ~/usbmount/source

sudo apt update && sudo apt -y install debhelper build-essential
sudo dpkg-buildpackage -us -uc -b
cd ..
sudo dpkg -i usbmount_0.0.24_all.deb
