
I got the Rust toolchains working along with the IDF.
I was able to flash C IDF examples and was even able to 
get a UART source displaying from chip to connsole.  For 
Rust the default flasher did not work with the USB OTG 
built into the ESP32S2 so I built and flashed manually but
never was able to get a debug stream so not sure where it 
was going.  I think I need a DevKitC or one of their other 
official devkits that inlcude a UART to serial chip other 
OCD.  


## Toolchain setup 

- [Install ESP 32 rust tool chain](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)   
- [Install ESP-IDF and first project setup](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html)  
- [Install Rust on linux using rustup](https://www.rust-lang.org/tools/install) ## Install on ubuntu under windows   
```
######
### Ubuntu Update
######
# Ensure we have a modern glib c will be needed latter for cargo
# The glibc embedded in Ubuntu 20.x is too old and will not work
# Upgrading to Ubuntu 22 bypasses the problem.
cd ~
sudo apt update
sudo apt full-upgrade
sudo apt upgrade
sudo do-release-upgrade
# This process will require answering yes several times.
# Do not execut this process if you have critical data
# or code in WSL which can not operate on a newer version
# of ubuntu.

####
## Enjsure Key pre-requisits are installed
####
# Ensure Python 3.8 or newer is installed
sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0 curl 
# may not be needed 
sudo pip install pyserial

####
## Give current user access to serial ports
####
sudo usermod -aG dialout $USER
#sudo usermod -aG dialout jwork
ln -s /usr/bin/python3 /usr/bin/python

sudoedit /etc/udev/rules.d/50-myusb.rules

# Save this text:
KERNEL=="ttyUSB[0-9]*",MODE="0666"
KERNEL=="ttyACM[0-9]*",MODE="0666"

# Optional add user to sudoers list
sudo usermod -aG sudo $USER



######
# Install the Core IDF for ESP32
######


ln -s /usr/bin/python3 /usr/bin/python


mkdir -p ~/esp
cd ~/esp
git clone -b v5.2.1 --recursive https://github.com/espressif/esp-idf.git
cd ~/esp/esp-idf
sudo ln -s /usr/bin/python3 /usr/bin/python
# RESTART SHELL

sudo idf_tools.py install-python-env
./install.sh esp32,esp32s2,esp32s3,esp32c2
cp export.sh ~
# RESTART SHELL

######
# Install the Basic rust compiler
######
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# RESTART SHELL

######
# Add Rust portions of Build chain for ESP32
######
sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev
sudo apt-get install libudev-dev
rustup update
cargo install espup
espup install
# Source this file in every terminal:
source $HOME/export-esp.sh
source $HOME/esp/esp-idf/export.sh
cargo install ldproxy
cargo install cargo-espflash
cargo install espflash
cargo install cargo-flash
cargo install cargo-generate

```

## Rust First App After Toolchain installed
- [Writing your own Rust ESP32 app](https://docs.esp-rs.org/book/writing-your-own-application/index.html)
- 
```
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



export IDF_PATH=~/esp/esp-idf/
source ~/export.sh 
source ~/export-esp.sh
idf.py set-target esp32s2
python -m esptool --chip esp32s2 -b 460800 --before default_reset --after hard_reset write_flash "@flash_args"

```


