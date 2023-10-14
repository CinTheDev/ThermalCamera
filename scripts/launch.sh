#!/bin/bash

./scripts/upload_bin.sh

ssh -Y thermal-camera@raspberrypi 'sudo systemctl stop thermal-camera && cd thermal-camera && ./thermal-camera -w'
