#!/bin/bash

# Runs thermal-camera via ssh; window is displayed on host system

./scripts/upload_bin.sh

ssh -Y pi@thermal-camera 'sudo systemctl stop thermal-camera && sudo su thermal-camera -c "opt/thermal-camera/bin/thermal-camera -w"'
