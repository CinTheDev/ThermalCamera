#!/bin/bash

# Runs thermal-camera via ssh; window is displayed on host system

./scripts/upload_bin.sh

ssh -Y thermal-camera@thermal-camera '/opt/thermal-camera/bin/thermal-camera -w'
