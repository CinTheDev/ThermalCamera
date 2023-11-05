#!/bin/bash

# Restarts thermal-camera on raspi; it can run independently from host system

./scripts/upload_bin.sh

ssh thermal-camera@raspberrypi 'sudo systemctl restart thermal-camera'
