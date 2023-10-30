#!/bin/bash

# Copy script

rsync -r ./scripts/raspi/configure_therm_cam.sh temp@thermal-camera:~/thermal-camera/
ssh temp@thermal-camera 'cd thermal-camera && ./configure_therm_cam.sh'
