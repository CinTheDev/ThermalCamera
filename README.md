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

- `sudo nano /boot/config.txt`
- Look for `dtparam=i2c_arm=on`
- Append `,i2c_arm_baudrate=400000` or whatever speed should be used
- Save

And restart again.

### Display setup

In order to use the display, some things still have to be configured.

<https://www.waveshare.com/wiki/3.5inch_RPi_LCD_(A)#Getting_Started>

Follow the first part of the guide with the driver installaion, skip the rest.

If the touchscreen axes are "inverted" in some way after reboot, you must edit `/usr/share/X11/xorg.conf.d/99-callibration.conf` and change "SwapAxes" to "0" (or to "1" if it's already 0)

Now the touchscreen should behave properly.

#### Increase Display framerate

If the default framerate is too slow:

- `sudo nano /boot/config.txt`
- Look for `dtoverlay=waveshare35a` and append `:speed=20000000,fps=30` or other desired values
- Save and restart

## Development setup

On our working machine, we have to make sure we can cross compile the program and upload it to the raspi.

We first have to add the correct Rust toolchain:

`rustup target add armv7-unknown-linux-gnueabihf`

We will use cargo cross for compiling. Install cross with `cargo install cross`

Additionally, we will use `podman` for cross, you can use docker instead, but it's kinda a chore to deal with root priviledges. Install podman with `sudo apt install podman`.

## Hardware setup & Used pins

### MLX Thermal Camera

The MLX module is connected to Raspberry Pi GPIO in the following fashion:

- [Camera Pin] > [Raspi Pin]
- VIN > 5V (Pin 2)
- GND > Ground (Any Pin equal or above Pin 30)
- SCL > Pin 3 (I2C SCL)
- SDA > Pin 2 (I2C SDA)

### Display Pins

Since some Pins are used by the MLX, we cannot put the Display directly on the Raspberry Pi, we have to manually connect the pins with jumper wires.

Also, the display, by documentation, occupies every power pin on the raspi. This is a problem since the MLX needs one power pin for itself. I have discovered that keeping **Pin 2 disconnected** won't affect the display. That way we have a free power pin for the MLX.

Use the display guide mentioned above in Display Setup to connect the relevant pins. The pins are listed on the bottom of the page.
