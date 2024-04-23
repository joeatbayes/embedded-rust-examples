* [LICENSE](LICENSE) - Please give us links from your articles
  and sites if you find our work useful.

* **[research](research.md)** Rust Notes on ESP with Examples. 
  I haven't found existing examples to be particularly useful, accurate, or compilable. Consequently, I've begun creating my own that are guaranteed to compile for a ESP32 S3 mini on our custom RainAmp.com sensor board..

* **[linux rust install](linux-install.md)**
* [first app after install](first-rust-app-after-toolchain-installed.md)

* **[windows-wsl-install](windodows-wsl-install.md)** Notes collected when trying to get rust toolchain working on WSL under windows.  Got to build but never sucessful flash from WSL.


# Rust ESP32 Examples 
Examples intended to show each piece of functionality needed for the RainAmp System in Isolation where they can be tested in isolation.    

Written for the RainAmp Soil Moisture Board Ver 1.1 from March-26-2024 running ESP32S3 mini 1U module 
with 4MB flash and 2MB PS RAM.


- GPIO Turn on/off LED
  - [turn led or motor on/off and toggle led state](ex-led-on-off/readme.md)

- GPIO Turn on/off Motor at different speed using PWM

- GPIO Read GPIO Status 

- GPIO Change Pin from Input to Output and back
  - **[ex-gpio-change-pin-direction-var-cap-read-1-pin](ex-gpio-change-pin-direction-var-cap-read-1-pin)** Similar to our [two pin variable cap read](ex-gpio-measure-time-to-change-high-to-low) but
  rather than measuring time to rise from 0 to 0.7V 
  this version measures time to drop from VCC (3.3V) to 0.7V
  while using only 1 pin and 1 resistor. *esp-idf-hal*
    - Change GPIO Pin Direction
    - Read GPIO Status
    - Mesure variable capacitance
    - Using Polling to detect GPIO status change
    - Measure elapsed time in Microseconds(uS)
    - Measure elapsed time in Nanosecods
    - Delay for Micro seconds and milli seconds    
    - Disable debug logging
    - Isolate pin Assignments for each change between boards
    - Change default watchdog timer
    - Change minimum freetos task sleep down to 1ms
    


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

    This project aims to measure changes in capacitance using a foil strip sensor taped to a bottle. The idea is to track the time it takes for a circuit to charge from 0V to a logic high transition voltage around 0.7V.The theory is that a higher capacitance, caused by increased water level, should take longer to charge. Conversely, a lower capacitance due to decreased water level should charge faster.
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

# About Us
If you find value in these examples please add a link to our main 
product site https://RainAmp.com to one or more of your sites or 
articles. Please let us know if you add links so we can reference 
them.    If you feel like donating funds to help accelerate these
examples then please contact us info@RainAmp.com    

At our small shop, we combine a deep understanding of physics and fundamental electronics to achieve results others might deem overly complex or expensive. We handle everything in-house, from designing circuits and PCBs using discrete components to writing firmware and integrating with cloud components, 
machine learning and GUIs as needed.

#### Need help accelerating your impossible project?
If others have told you your project is impossible or will take years, we can help! We excel at taking on challenges others shy away from.

#### Tired of cookie-cutter solutions?
We understand your frustration with engineers who simply slap open-source firmware on existing boards and call it a product. We offer a more tailored approach, creating solutions specifically for your needs.

#### Let's talk!
If you're looking for a team that can take your project from concept to reality, we'd love to hear from you.
