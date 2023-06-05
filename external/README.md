# External dependencies not handled by cargo

Here is additional software which has to be manually set up for the project to compile with cargo.

I have manually created these directories and compiled the libraries inside, and cannot verify that this is the correct way to do it.
Please tell me if something is done wrong or doesn't work.

## Toolchain "arm-none-linux-gnueabihf" setup

- Follow the steps *Development setup* inside the project root's README.md to obtain the correct toolchain inside /opt
- Copy the contents of arm-none-linux-gnueabihf from here into the toolchain folder, like this:
- `cp -r [projcet root]/external/arm-none-linux-gnueabihf /opt`
