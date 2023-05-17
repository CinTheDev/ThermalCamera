#!/bin/bash

ORANGE='\033[0;33m'
NOCOL='\033[0m'

(
    cd thermal-camera
    cargo build
    scp target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
)

echo -e "${ORANGE}You will now be ssh-redirected onto the Thermal Camera."
echo -e "Remember to exit when finished.${NOCOL}"

ssh -Y -t thermal-camera@raspberrypi 'cd thermal-camera; /bin/bash'
