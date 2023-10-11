#!/bin/bash

./scripts/upload_bin.sh

ssh thermal-camera@raspberrypi 'sudo systemctl restart thermal-camera'
