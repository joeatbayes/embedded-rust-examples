

## Rust First App After Toolchain installed
- [Writing your own Rust ESP32 app](https://docs.esp-rs.org/book/writing-your-own-application/index.html)
- 
```file:///home/jwork/rainampmp/firmware/rust/examples/ex-basic-uart/readme.md

# Source this file in every terminal:
export IDF_PATH=~/esp/esp-idf
source $HOME/export-esp.sh
source $HOME/esp/esp-idf/export.sh

# Replace espwrk with whereever you plan to 
# save your work.
md ~/espwrk
cd ~/esp
cargo generate esp-rs/esp-template
# OR
cargo generate esp-rs/esp-idf-template cargo
  # project name: app1
  # CPU chosen: esp32s2
  # Advanced Config: True
  # Enable STD support: True
  # Enable WiFi via esp-wifi crate: false
  # Enable Allocations via esp-alloc cates: true
  # Configure to use dev containers: false
  # Configure project to support wokwi simulation: false
  # Add CI files for Githup Action: false
  # Setup logging using log crate: true
cd app1
cargo build


cargo run
  # Ensure the device selected matches your 
    # device in my case /dev/ttyACM0 matches because  I know 
    # it was created when I plugged in the device.
  # Remember serial port device for future: y
    # Do not do this when using multiple devices simutaneously
    # since that can cause device numbering changes.


# On my system when trying to flash the Wemos ESP32S2 mini
# cargo run fails to connect to the chip so lets try to 
# flash with epstool.py instead.


# Generate .bin file that can be flashed to the
# the chip.
cargo espflash save-image --chip esp32s2 ap.bin

esptool.py --port /dev/ttyACM0 erase_flash

esptool.py --chip esp32 --port /dev/ttyACM0 write_flash -z 0x1000 ap.bin

Seems to be an issue using OTG USB via USB from ESP32-S2. Have not figured out how to enable CDC (Debug stream over built int USB) yet. https://github.com/esp-rs/esp-println/issues/14
rustup component add rustfmt


# https://github.com/esp-rs/espflash/blob/main/cargo-espflash/README.md

https://github.com/espressif/rust-esp32-example

podman run --device /dev/ttyUSB0 --device /dev/ttyUSB1 -it espressif/idf-rust-examples
# sudo apt-get -y install podman
https://docs.podman.io/en/latest/Introduction.html


# Run these two source commands every time
# a new command shell is started.

export IDF_PATH=~/esp/esp-idf/
source ~/export.sh 
source ~/export-esp.sh
idf.py set-target esp32s2
python -m esptool --chip esp32s2 -b 460800 --before default_reset --after hard_reset write_flash "@flash_args"

```

