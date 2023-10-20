#!/bin/bash

./scripts/upload_bin.sh

ssh thermal-camera@raspberrypi 'sudo systemctl stop thermal-camera && cd thermal-camera && export DISPLAY=:0 && export RUST_BACKTRACE=1 && ./thermal-camera -w'
