#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"

cd $DRIVERS_PATH

sudo chmod +x ./LCD35-show
sudo ./LCD35-show

# TODO: Somehow make the drivers not restart
#       and configure display further
