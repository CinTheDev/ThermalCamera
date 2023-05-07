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

### Peripheral setup

Later, when enabling the I2C peripheral:

`sudo raspi-config`

-> Interfacing Options

-> I2C

-> "Yes" to enable

-> Finish

Restart so I2C can be activated.

## Development setup

On our working machine, we have to make sure we can cross compile the program and upload it to the raspi.

We first have to add the correct Rust toolchain:

`rustup target add armv7-unknown-linux-musleabihf`

For our program to link correctly, we have to manually download the gnu toolchain and add it to our PATH. I got my toolchain from here: <https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads>

I chose **gcc-arm-11.2-2022.02-x86_64-arm-none-linux-gnueabihf.tar.xz**, extracted it somewhere in my home directory, and prepended the binary directory to PATH via ~/.bashrc

Verify the installation by running `cargo clean` (if there's some build files already) and `cargo build`. If it compiles successfully, that's a good sign. Finally, upload the binary to the raspi using the **upload_bin.sh** script, and do a test run on the raspi with ssh.
