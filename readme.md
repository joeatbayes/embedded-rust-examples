* [LICENSE](LICENSE) - Please give us links from your articles
  and sites if you find our work useful.

* [research](research.md) Rust Notes on ESP as well as examples 
  from others.  I can not claim that I found these examples 
  great which is the main reason I started producing my own 
  which I know will compile on the ESP32 S3 mini on our 
  custom RainAmp.com sensor board.

* [linux rust install](linux-install.md)

* [humid port](chat-gpt-port-humid.md) example output of chatgpt attempt to port my humid code.

* [debugging-reset](debugging-reset.md) research into why the reset button is not working as expected.

* [windows-wsl-install](windodows-wsl-install.md) Notes collected when trying to get rust toolchain working on WSL under windows.  Got to build but never sucessful flash from WSL.


# Rust ESP32 Examples 
Examples intended to show each piece of functionality needed for the RainAmp System in Isolation where they can be tested in isolation.    

Written for the RainAmp Soil Moisture Board Ver 1.1 from March-26-2024 running ESP32S3 mini 1U module 
with 4MB flash and 2MB PS RAM.


- GPIO Turn on/off LED
  - [turn led or motor on/off and toggle led state](ex-led-on-off/readme.md)

- GPIO Turn on/off Motor at different speed using PWM

- GPIO Read GPIO Status 

- GPIO Change Pin from Input to Output and back

- Record time for GPIO to change Status
  - **[ex-gpio-measure-time-to-change-high-to-low](ex-gpio-measure-time-to-change-high-to-low)** - 
    - Measure elapsed time
    - Measure variable capacitance
    - Detect change in GPIO state
    - Demonstrate Open Drain high-impedance pins 
    - Drive I/O pin high 
    - Delay for a specific duration
    - Read GPIO pin status
    - Change OpenDrive GPIO pin mode between Drain and high impedance
    - Oversampling to improve reading stability 

    This project aims to measure changes in capacitance using a foil strip sensor taped to a bottle as the water level fluctuates. The idea is to track the time it takes for a circuit to discharge (not charge) from a high voltage (around 3.3V) to a specific threshold voltage for a logic low state (around 0.7V). The theory is that a higher capacitance, caused by increased water level, should take longer to discharge. Conversely, a lower capacitance due to decreased water level should discharge faster.
    *uses esp-idf-hal* 


- UART - Read / Write UART ECHO
- **[ex-uart-echo-rs485](ex-uart-echo-rs485/readme.md)**: Reads uart 1 byte
  at a time and writing data back to sender 1 byte at a time with write enable
  pin control to allow use of 1/2 duplex transceiver.  Note esp-hal is missing
  a feature to either query the uart RX buffer or read non-blocking so 
  could not show that feature.  *esp-hal*

- UART - Process inbound uart data while also doing other work.
- **[ex-uart-non-block-rs485](ex-uart-non-block-rs485/readme.md)**:
  Uart reading in a non blocking fashion and echoing back the 
  data with write enable control to allow use of 1/2 duplex transceiver
  intended to demonstrate fast polling approach to servicing inbound
  uart traffic inbetween other sensor duties.
  *esp-idf-hal*

- TODO: UART - Basic UART protocol
   - TODO: ex-uart-non-block-readline-with-callback-485

- TODO: Read Touch Sensor Raw Reading

- TODO: 1Wrire - Read DS1820B Sensor

- TODO: Read internal temperature sensor 

- TODO: Read DHT22 Sensor

- TODO: Split a line into Tokens

- TODO: Parse Key = Value pairs file 

- TODO: Parse & Serialize simnple JSON structure

- TODO: Partition Flash into Program and Storage

- TODO: Save Calibration Settings

- TODO: Read Calibration Sessings

- TODO: Enter and resume from Light Sleep

- TODO: Enter and resume from deep sleep

- TODO: Create a BLE service

- TODO: Show results from BLE service on chrome BLE

- TODO: Demonstrate BLE Mesh with at least 1 relay

- TODO: One Rust project built for different board with different CPU 
  and differnt Pin assignments.
  - Show how using Rust config and macros
