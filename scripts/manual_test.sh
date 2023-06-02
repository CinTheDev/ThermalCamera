#!/bin/bash

ORANGE='\033[0;33m'
NOCOL='\033[0m'

(
    cd scripts
    ./upload_bin.sh
)

echo -e "${ORANGE}You will now be ssh-redirected onto the Thermal Camera."
echo -e "Remember to exit when finished.${NOCOL}"

ssh -Y -t thermal-camera@raspberrypi 'cd thermal-camera; /bin/bash'
