# Thermal Camera

A project for the Raspberry Pi, where a thermal camera is connected to the Raspi, which save or display pictures on a screen.

## Build guide

### Requirements

Before starting the build, let's first talk about what is required for building a ThermalCamera.

- A 3D-Printer (Optional, 3D-prints can be ordered online, but it's rather expensive, not recommended)
- About 150€ [TODO: Verify price] for electrical components
- Some jumper cables, both female-male and female-female [TODO: Tell precise amount of cables]
- A soldering kit

### Preparation

#### Ordering components

The ThermalCamera system consits of three components: The IR sensor, a processing unit, and a touchscreen display. With the 150€, order these components:

- [MLX90640](https://www.berrybase.de/adafruit-mlx90640-ir-waermebildkamera-breakout)
- [Raspberry Pi 3 B](https://www.berrybase.de/detail/index/sArticle/3924?src=raspberrypi)
- [3.5" Berrybase Display](https://www.berrybase.de/3-5-display-fuer-raspberry-pi-mit-resistivem-touchscreen)

While waiting for the components to arrive, start to print all the parts for the case

#### 3D-Printing the case

I recommend to use a more strong filament type, like PETG or ABS/ASA. The case used by this guide is made of orange PETG.

Under the folder CAD_CaseV2/Export you'll find a bunch of .stl and .3mf files. The .3mf are project files from Prusa Slicer, and can be ignored if you don't have the exact same printer and exact same settings as the file dictates.

The .stl files are the files you'll need, except for the Tripod_Connector.stl, which is optional. Start with printing the support first, as you'll need them right away when continuing with the components.

If the supports are done and your components have arrived, you can go on with wiring while the rest of the case is still printing.

If the case is done, make sure to test if the pen holder on the right bottom side is good. The Berrybase display comes with a stylus, which can be put into the pen holder. If you need to use a lot of force, it means that 3D-printing artifacts like strings and blobs have clogged the hole a little, and need to be removed by scraping along the inner walls a bit. Once the stylus fits easily into the pen holder, the case is good to go!

Another feature of the case is the interface at the bottom. Normally you'd screw the handgrip on for a nice way of holding the case, but anything can be screwed onto it. An example is the tripod connector: This is an adapter which makes it able to put the ThermalCamera onto a tripod, so you can statically record a specific location for a longer period of time.

### Supports and wiring

If all the electrical components have arrived and all three supports have been printed, you can go on with this section.

Before you can do anything else, the MLX module needs to be prepared for wiring. If you haven't noticed it yet, it comes in two parts: the main module, and a pin header. If you are using jumper wires, you need to solder the pin header onto the MLX module. Make sure to put the pins on the bottom side, on the opposite side of the sensor cylinder.

Now put all components on the dedicated supports. The Raspberry Pi must be screwed onto the support, and the MLX module can be screwed on, but it's not mandatory. The display has no way of being fixed to the support.

Important note: when pointing the MLX module sideways, **the edge with the pin header marks the bottom side**. Later when putting the support into the case, make sure to put it in the right way.

With the components on their supports, go on with the wiring. Start with the MLX module

#### MLX wiring

The MLX module is connected to Raspberry Pi GPIO in the following fashion:

- [Camera Pin] > [Raspi Pin]
- VIN > 5V (Pin 2)
- GND > Ground (Any GND Pin equal or above Pin 30)
- SCL > Pin 3 (I2C SCL)
- SDA > Pin 2 (I2C SDA)

#### Display wiring

Since some Pins are used by the MLX, we cannot put the Display directly on the Raspberry Pi, we have to manually connect the pins with jumper wires.

Also, the display, by documentation, occupies every power pin on the raspi. This is a problem since the MLX needs one power pin for itself. I have discovered that keeping **Pin 2 disconnected** won't affect the display. That way we have a free power pin for the MLX.

Use [this guide](https://www.waveshare.com/wiki/3.5inch_RPi_LCD_(A)#Interface_Definition) to connect all other pins using jumper wires.

## Development setup

On our working machine, we have to make sure we can cross compile the program and upload it to the raspi.

We first have to add the correct Rust toolchain:

`rustup target add armv7-unknown-linux-gnueabihf`

For our program to link correctly, we have to manually download the gnu toolchain and add it to our PATH. I got my toolchain from here: <https://developer.arm.com/downloads/-/gnu-a>

**IMPORTANT: The glibc version of the toolchain must NOT be higher than 2.31!! Otherwise the program will crash on the raspi.**

Choose **gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf.tar.xz** and extract it to /opt.

Finally, run `cargo clean` inside thermal-camera/ (if there's some build files already), and run a build task from VSCode to verify the configuration (e.g. "Run manual test"). If VSCode somehow doesn't work, just run `./scripts/manual_test.sh` from the project's root directory.
