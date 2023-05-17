#!/bin/bash

cd thermal-camera
cargo build

scp target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
