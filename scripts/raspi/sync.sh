#!/bin/bash

# Copy this directory onto raspi and connect via ssh

cd thermal-camera
cross build --target armv7-unknown-linux-gnueabihf
cd ..

rsync -P -r --delete ./scripts/raspi/ pi@thermal-camera:~/thermal-camera-config/
rsync -P ./thermal-camera/target/armv7-unknown-linux-gnueabihf/debug/thermal-camera pi@thermal-camera:~/thermal-camera-config/files/
ssh -t pi@thermal-camera 'cd thermal-camera-config; /bin/bash'
