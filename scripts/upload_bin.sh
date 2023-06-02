#!/bin/bash

cd ../thermal-camera
cargo build
rsync -P target/armv7-unknown-linux-gnueabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
