#!/bin/bash

( cd thermal-camera && cargo build )

scp thermal-camera/target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera

ssh thermal-camera@raspberrypi "cd thermal-camera && ./thermal-camera"

mkdir -p test-output

rm -f test-output/test.pgm

scp thermal-camera@raspberrypi:~/thermal-camera/test.pgm test-output

echo "Output image has been copied to test-output/"
