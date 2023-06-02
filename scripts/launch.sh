#!/bin/bash

(
    cd scripts
    ./upload_bin.sh
)

ssh -Y thermal-camera@raspberrypi << EOF
    cd thermal-camera
    ./thermal-camera -w
EOF
