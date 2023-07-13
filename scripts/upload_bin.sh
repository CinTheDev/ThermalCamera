#!/bin/bash

PURPLE="\033[0;35m"
RED="\033[1;31m"
NOCOL="\033[0m"

cd thermal-camera

cargo build

if ! command -v rsync > /dev/null
then
    echo -e "${PURPLE}It is recommended to use rsync for uploading files"
    echo -e "Run ${RED}sudo apt install rsync${PURPLE} for faster upload speeds.${NOCOL}"
    scp target/armv7-unknown-linux-gnueabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
    exit
fi

rsync -P target/armv7-unknown-linux-gnueabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera