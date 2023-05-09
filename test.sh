#!/bin/bash

( cd thermal-camera && cargo build )

scp thermal-camera/target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera

ssh thermal-camera@raspberrypi << EOF
    cd thermal-camera
    ./thermal-camera
EOF

mkdir -p test-output

# Remove the file if it's already there,
# That way it will be more obvious if an error occurs
rm -f test-output/test.pgm

scp thermal-camera@raspberrypi:~/thermal-camera/test.pgm test-output

echo "Output image has been copied to test-output/"
