# Thermal Camera

A project for the Raspberry Pi, where a thermal camera is connected to the Raspi, which save or display pictures on a screen.

## Raspberry Pi image setup

### Initial setup

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

Optional:

- Add personal public ssh key for easy access via ssh
- Install eog for viewing image files via ssh

### Peripheral setup

Later, when enabling the I2C peripheral:

- `sudo raspi-config`
- -> Interfacing Options
- -> I2C
- -> "Yes" to enable
- -> Finish

Restart so I2C can be activated.

Lastly, increase the speed of the I2C peripheral for images to be read much quicker:

- `sudo nano boot/config.txt`
- Look for `dtparam=i2c_arm=on`
- Append `,i2c_arm_baudrate=400000` or whatever speed should be used
- Save

And restart again.

## Development setup

On our working machine, we have to make sure we can cross compile the program and upload it to the raspi.

We first have to add the correct Rust toolchain:

`rustup target add armv7-unknown-linux-gnueabihf`

For our program to link correctly, we have to manually download the gnu toolchain and add it to our PATH. I got my toolchain from here: <https://developer.arm.com/downloads/-/gnu-a>

**IMPORTANT: The glibc version of the toolchain must NOT be higher than 2.31!! Otherwise the program will crash on the raspi.**

Choose **gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf.tar.xz** and extract it to /opt.

Additionally, follow the steps inside external/README.md

Finally, run `cargo clean` inside thermal-camera/ (if there's some build files already), and run a build task from VSCode to verify the configuration (e.g. "Run manual test"). If VSCode somehow doesn't work, just run `./scripts/manual_test.sh` from the project's root directory.

## Hardware setup & Used pins

The Thermal Camera is connected in the following fashion:

- [Camera Pin] > [Raspi Pin]
- VIN > 3.3V
- GND > Ground
- SCL > GPIO 3 (I2C SCL)
- SDA > GPIO 2 (I2C SDA)

**Make sure not to switch up 3.3V and 5V!! Doing so could damage the camera!**

## TODOs

- Add Bad Pixel correction
- Improve eframe UI (aspect ratio correction, nicer buttons, ...)
