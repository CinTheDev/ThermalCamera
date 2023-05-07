# Thermal Camera

A project for the Raspberry Pi, where a thermal camera is connected to the Raspi, which save or display pictures on a screen.

## Raspberry Pi image setup

Following steps have been taken for the Raspberry Pi setup:

- Install Raspberry OS x32 on the SD card
- Setup install
- Country & Language: German
- Username: thermal-camera
- Password: thermal-camera
- Connect to WIFI
- Finish setup & Restart
- Enable ssh via systemctl

Now the Raspi can be accessed via `ssh thermal-camera@raspberrypi`

At this point, there is no need for an external keyboard and monitor anymore, the Raspi just needs to be powered and in reach of the configured WIFI.

## Development setup

On our working machine, we have to make sure we can compile a binary and upload it to the raspi.

Following rust commands have been run:

`sudo apt install gcc-arm-linux-gnueabihf`

`rustup target add armv7-unknown-linux-gnueabihf`
