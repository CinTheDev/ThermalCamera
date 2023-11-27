# Thermal Camera Raspberry Pi configuration

This directory contains scripts to fully configure a Raspberry Pi to run thermal-camera on startup

## Setup

Do the following before running any scripts inside here:

- Prepare a new Raspbian OS lite x32 sd card; if possible, do the following steps inside the raspberry pi imager
- (It's recommended to set the hostname to something other than raspberrypi)
- Make sure to create a user 'pi' with sudo rights; this will serve as the administrative user
- Connect to the internet either over wifi (recommended) or over ethernet
- (Turn on ssh)

## Installation

- Login as pi either over ssh or directly
- Copy this directory to /home/pi
- Set executable flag for run.sh
- Execute run.sh
