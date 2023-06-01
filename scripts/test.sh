#!/bin/bash

( 
    cd thermal-camera
    cargo build
    scp target/armv7-unknown-linux-musleabihf/debug/thermal-camera thermal-camera@raspberrypi:~/thermal-camera/thermal-camera
)

ssh thermal-camera@raspberrypi << EOF
    cd thermal-camera
    ./thermal-camera test.png
EOF

# Remove the file if it's already there,
# That way it will be more obvious if an error occurs
rm -rf test-output
mkdir test-output

scp thermal-camera@raspberrypi:~/thermal-camera/test.png test-output

echo "Output image has been copied to test-output/"
