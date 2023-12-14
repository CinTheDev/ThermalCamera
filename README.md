# Thermal Camera

A project for the Raspberry Pi, where a thermal camera is connected to the Raspi, which save or display pictures on a screen.

## Development setup

On our working machine, we have to make sure we can cross compile the program and upload it to the raspi.

We first have to add the correct Rust toolchain:

`rustup target add armv7-unknown-linux-gnueabihf`

For our program to link correctly, we have to manually download the gnu toolchain and add it to our PATH. I got my toolchain from here: <https://developer.arm.com/downloads/-/gnu-a>

**IMPORTANT: The glibc version of the toolchain must NOT be higher than 2.31!! Otherwise the program will crash on the raspi.**

Choose **gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf.tar.xz** and extract it to /opt.

Finally, run `cargo clean` inside thermal-camera/ (if there's some build files already), and run a build task from VSCode to verify the configuration (e.g. "Run manual test"). If VSCode somehow doesn't work, just run `./scripts/manual_test.sh` from the project's root directory.

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
