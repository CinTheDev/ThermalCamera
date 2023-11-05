#!/bin/bash

# Dumps the terminal log messages from the thermal-camera service

ssh pi@thermal-camera 'journalctl -u thermal-camera -b'
