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
