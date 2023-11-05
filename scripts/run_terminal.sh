#!/bin/bash

# Runs thermal-camera via ssh; console messages are synchronized, the window is displayed on the raspi

./scripts/upload_bin.sh

ssh thermal-camera@raspberrypi 'sudo systemctl stop thermal-camera && export DISPLAY=:0 && export RUST_BACKTRACE=1 && opt/thermal-camera/bin/thermal-camera -w'
