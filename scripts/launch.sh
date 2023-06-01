#!/bin/bash

(
    cd thermal-camera
    cargo build
    scp target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
)

ssh -Y thermal-camera@raspberrypi << EOF
    cd thermal-camera
    ./thermal-camera -w
EOF
