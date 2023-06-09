#!/bin/bash

./scripts/upload_bin.sh

ssh -Y thermal-camera@raspberrypi 'cd thermal-camera && ./thermal-camera -w'
