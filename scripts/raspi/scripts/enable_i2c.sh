#!/bin/bash

sudo raspi-config nonint do_i2c 0

sudo sed -i "s/dtparam=i2c_arm=on/dtparam=i2c_arm=on,i2c_arm_baudrate=400000/g" /boot/config.txt
