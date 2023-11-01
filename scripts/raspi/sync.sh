#!/bin/bash

# Copy this directory onto raspi and connect via ssh

rsync -r --delete ./scripts/raspi/ pi@thermal-camera:~/thermal-camera-config/
ssh -t pi@thermal-camera 'cd thermal-camera-config; /bin/bash'
