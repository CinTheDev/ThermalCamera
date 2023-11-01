#!/bin/bash

# Install and configure display drivers

DRIVERS_PATH='/opt/display-drivers'

git clone https://github.com/waveshare/LCD-show.git "$DRIVERS_PATH"

chmod +x "$DRIVERS_PATH/LCD35-show"
$DRIVERS_PATH/LCD35-show lite
