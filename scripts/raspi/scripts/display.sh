#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

sudo git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"

cd $DRIVERS_PATH

# To prevent the display drivers from restarting the whole system
sudo systemd-inhibit --why="The installation script is still running" sleep 1000 &

sudo chmod +x ./LCD35-show
./LCD35-show lite

kill $!

# TODO: Somehow make the drivers not restart
#       and configure display further
