#!/bin/bash

# Restarts thermal-camera on raspi; it can run independently from host system

./scripts/upload_bin.sh

ssh pi@thermal-camera 'sudo systemctl restart thermal-camera'
