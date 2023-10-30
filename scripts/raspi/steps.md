# Script steps

The following things will be done by the script:

- Check for internet connection; if none, abort
- Create user "thermal-camera" if not already there
- Install necassary X libs for windowing to work
- Enable I2C
- Configure I2C to be on fast mode
- Do Display setup
- Configure display
- Add systemctl service
- Restart

## Requirements

The directory of the script should contain the following:

- Compiled binary "thermal-camera"
- The system should be minimal linux / Raspbian lite
