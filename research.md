# RUST

## Generic Rust
- [Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
 )

- [Rust Reference](https://doc.rust-lang.org/reference)
   -  [Functions](https://doc.rust-lang.org/reference/items/functions.html)
   -  [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html)
   -  [Unions](https://doc.rust-lang.org/reference/items/unions.html)
   -  [Match expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html)
   -  [structs](https://doc.rust-lang.org/reference/items/structs.html)
   -  [Macros](https://doc.rust-lang.org/reference/macros.html)
   -  [traits](https://doc.rust-lang.org/reference/items/traits.html)
   -  [Implementations](https://doc.rust-lang.org/reference/items/implementations.html)
   - [Generics](https://doc.rust-lang.org/reference/items/generics.html)

- [Write Better match statements in rust](https://towardsdatascience.com/write-better-match-statements-in-rust-7458402afacd)


## Embedded Rust
- [The Rust Embedded Book](https://docs.rust-embedded.org/book/intro/hardware.html) includes lots of good resource links.

- [The Embedded Rust Ecosystem](https://dev.to/apollolabsbin/)embedded-rust-the-esp-development-ecosystem-5478 explains the ESP rust ecosystem 


- [Rust Tips for embedded C developers](https://doc.rust-lang.org/beta/embedded-book/c-tips/index.html)

- [The Rust Embedded Book](https://docs.rust-embedded.org/book/intro/hardware.html) includes lots of good resource links.


## ESP32 Rust
-  [Understanding esp-template on rust](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-template.html)

- [Embedded Rust Training for Espressif ](https://github.com/esp-rs/std-training)

- [Rust on ESP Book](https://docs.esp-rs.org/book/) by exspresiff 

- [Embedded Rust Training for Espressif ](https://github.com/esp-rs/std-training)



 
## Rust Examples 
   - [Basic example of hello world, connect to wifi,  set RGB pixel,  asic thread](https://github.com/esp-rs/std-training/blob/main/intro/hardware-check/src/main.rs)
   
  -  [Basic example of hello world, connect to wifi,  set RGB pixel,  asic thread](https://github.com/esp-rs/std-training/blob/main/intro/hardware-check/src/main.rs)

  ### I2C Samples   
  - [ESP32 Standard library Embedded Rust: I2C communication](https://dev.to/apollolabsbin/esp32-standard-library-embedded-rust-i2c-communication-53c3) shows interacting with DS1307 clock.
  - [BMP 180 temperature senso with mqttr](https://github.com/bjoernQ/esp32c3-rust-std-temperature-logger)  Also shows accessing WiFi Access point, Threading for mqtt listner, 

  
  * [ESP32 Embedded Rust at the HAL: I2C Scanner](https://dev.to/apollolabsbin/esp32-embedded-rust-at-the-hal-i2c-scanner-54mg)

  ### 1 Wire & Other Bus
  - DHT22 Read
  
  - DS1820B Read
    - [rust library on stmf103 to read DS18B20](https://github.com/kellerkindt/onewire)
    - [esp rust 1 wire read ds18b20](https://github.com/fuchsnj/one-wire-bus/blob/master/src/lib.rs)

  
  ### Time Measurement
    [Embedded-time crates.io](https://crates.io/crates/embedded-time/0.4.0)

    [ESP32 Embedded Rust at the HAL: Button-Controlled Blinking by Timer Polling](https://dev.to/apollolabsbin/esp32-embedded-rust-at-the-hal-button-controlled-blinking-by-timer-polling-27ep)


  ### GPIO Change Pin from Input to OUput and back
   [ stack overflow discussion about how to do it](https://stackoverflow.com/questions/65755746/rust-embedded-change-gpio-pin-from-output-to-input)
   

  ### GPIO READ 
  -  [Change GPIO Pin Direction](https://stackoverflow.com/questions/65755746/)rust-embedded-change-gpio-pin-from-output-to-input]
  -  [Change GPIO Direction and measure elapsed time using Rust and Embassy](https://dev.to/apollolabsbin/embedded-rust-and-embassy-timer-ultrasonic-distance-measurement-31mj)
  -  [ESP32 Embedded Rust at the HAL: GPIO Interrupts](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-gpio-interrupts)  esp-c3 requires unsafe code.



  ### POC Code to read internal temperature sensor 
  - [from esp_idf_issue_333](https://github.com/esp-rs/esp-idf-hal/issues/333)
      use std::{thread::sleep, time::Duration};

    use esp_idf_sys::{
        soc_periph_temperature_sensor_clk_src_t_TEMPERATURE_SENSOR_CLK_SRC_DEFAULT,
        temperature_sensor_config_t, temperature_sensor_enable, temperature_sensor_get_celsius,
        temperature_sensor_handle_t, temperature_sensor_install,
    };

    fn main() {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_svc::sys::link_patches();

        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();
        let mut temp_sensor: temperature_sensor_handle_t = std::ptr::null_mut();
        let temp_sensor_config = temperature_sensor_config_t {
            range_min: 10,
            range_max: 50,
            clk_src: soc_periph_temperature_sensor_clk_src_t_TEMPERATURE_SENSOR_CLK_SRC_DEFAULT,
        };
        unsafe {
            temperature_sensor_install(&temp_sensor_config, &mut temp_sensor);
            temperature_sensor_enable(temp_sensor);
        }
        let mut tsens_value: f32 = 0.0;
        loop {
            unsafe {
                temperature_sensor_get_celsius(temp_sensor, &mut tsens_value);
            }
            log::info!("{}", tsens_value);
            sleep(Duration::from_millis(1000));
        }
    }

  ### Touch Sensor Read Patch in esp_idf_hal
  - https://github.com/esp-rs/esp-idf-hal/pull/346
  - https://github.com/esp-rs/esp-idf-hal/pull/346/files/cba54d1aa70e956ebcebca4937db79e69211516e
  - https://github.com/esp-rs/esp-idf-hal/pull/346/files/cba54d1aa70e956ebcebca4937db79e69211516e#diff-2707a304c8fd32c1fe26c63c5f5491f5cc6a670ae492e779b3dfd272246cf3da

  ### GPIO Interupt on change
  - [GPIO Interupts Rust at the Hal ESP32](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-gpio-interrupts)

  ### GPIO Write 
  - [turn led or motor on/off and toggle led state](examples/ex-led-on-off/readme.md)


  ### GPIO PWM Write

  ### Accessing HAL

  ### ADC READ 

  ### ADC Read continous interupt

  ### ADC Change Reference voltage

  ### Touch Read  / Capacitive Read
    * [Code to read Capacitive sensor ](https://github.com/milewski/sensors-esp/blob/main/esp32/capacitive-switch/src/capacitive_sensor.rs)

  ### UART READ Simple
  - [ex-uart-echo-rs485](examples/ex-uart-echo-rs485/readme.md)  esp-hal

  - [ex-uart-non-block-rs485](examples/ex-uart-non-block-rs485/readme.md) esp-idf-hal

  * [ Simple Uart loopback Echo](https://github.com/esp-rs/esp-idf-hal/blob/master/examples/uart_loopback.rs)

  * [ Uart Echo Async](https://github.com/esp-rs/esp-idf-hal/blob/master/examples/uart_loopback_async.rs)

  * [ UART echo with Rust on ESP32-C3 youtube](https://youtu.be/-xEivxWe29M?si=ogXT1cRuUneOooyQ)
  
  * [Rust at the HAL: UART Serial Communication ](https://dev.to/apollolabsbin/esp32-embedded-rust-at-the-hal-uart-serial-communication-1ig4)

  - [P2 Threads with Rust ESP32-C3 on YouTube](https://youtu.be/ht5t39dEa4E?si=hR_mfYA-aH4eGTui)  May need this as alternative for reading Uart with events or interupts

  ### UART Write Simple
  - [ex-uart-echo-rs485](examples/ex-uart-echo-rs485/readme.md)  esp-hal
  - [ex-uart-non-block-rs485](examples/ex-uart-non-block-rs485/readme.md) esp-idf-hal

  ### UART READ Non-Block
   - [ex-uart-echo-rs485](examples/ex-uart-echo-rs485/readme.md)  esp-hal
   - [ex-uart-non-block-rs485](examples/ex-uart-non-block-rs485/readme.md) esp-idf-hal


  ### Threading 
  - [P2 Threads with Rust ESP32-C3 on YouTube](https://youtu.be/ht5t39dEa4E?si=hR_mfYA-aH4eGTui)  May need this as alternative for reading Uart

  ### BLE
  * [Embedded Rust Bluetooth on ESP: BLE Scanner](https://dev.to/apollolabsbin/embedded-rust-bluetooth-on-esp-ble-scanner-1gb7) Also includes many other ESP Rust BLE features BLE Scanner,  BLE Advertiser,   Secure BLE Client, Secure BLE SErver, 

  ### BLE MESH

  ### HTTP Server
  * [Rust on ESP32 STD demo app. A demo STD binary crate for the ESP32[XX] and ESP-IDF, which connects to WiFi, Ethernet, drives a small HTTP server and draws on a LED Screen](https://github.com/ivmarkov/rust-esp32-std-demo)


  ### HTTP Client

  ### MQTT Client
 - [std training](https://github.com/esp-rs/std-training/tree/main/intro)  hardware-check, http-client, http-server, mqtt, tls, tls_async, wifi, wifi_async, wifi_async_send, wifi_static_ip, wps, wps_async

  ### Read Write Flash as File System
  - [OFMon energy monitor](https://github.com/arashsm79/OFMon?tab=readme-ov-file#Using-LittleFS-in-Rust)  Shows how to create flash memory partitions and access using littlefs,   Also includes a web server, measuring current with an ADC, Flutter app to display results,  Scan nearby access points, OTA Update via Mobile device, 
  ### Read / Write Micro SD Card

  ### Multiple hardware boards from single Codebase 
   - https://www.youtube.com/watch?v=8ydYtuUKOsI

  ### Uncategorized

  - [esp-rs awsome-esp-rust](https://github.com/esp-rs/awesome-esp-rust?tab=readme-ov-file#projects)
    - std - anemometer with gps calibration and ota,  bluedroid bluetooth stack, dark sky monitor,  tookio asynk for IO, wi-fi, efuse, gpio, i2c, http, ble advertising, etc,  esp-clock,  esp32-nmea example,  tokio + axum web server on S3,  mqtt temperature logger on C3 
    - no standard - temperature logger,  maze,  ps2keyboard, ota experiment, slimevr,  graphic exmples esp32-s3 box,  neopixel via wifi,  opensource plant watering using esp-now,  mqtt-demo esp-c3,  lilygo t5 epaper with WiFi,  bare metal rust examples,  drawing robot,  esp32-camp  telegram bot taking pictures,  fast reverse proxy,   screensaver in mch2022 badge,  cadputer interactive 3d graphics, smart energy monitor,  littlefs, demo WiFi, ethternet, httpser draws on led screen,  self balancing robot, 


  - [Wide variety of sample code for Runs on ESP32](https://github.com/ivmarkov/rust-esp32-std-demo) 
    - [SVC Examples](https://github.com/esp-rs/esp-idf-svc/tree/master/examples)  eth, eth_spi, event_loop, http_client, http_server, http_ws_client,  http_ws_server, mqtt_client, mqtt_client_async, gest_set_c, get_set_raw_storage, sntp, tcp, 
      
    - [HAL Examples](https://github.com/esp-rs/esp-idf-hal/tree/master/examples) adc, adc_oneshot, blinky, blinky_async, button, button_async, i2c_master_slave, i2c_ssd1306, ledc_simple, ledc_simple, ledc_thredad, pcnt_rotary_encoder, rmt_morse_code, rmt_musical_buzzer, spi_loopback, spi_loopback_async, spi_st7789, timer_async, timer_notify, uart_loopback, uart_loopback_async

    - [IDF Examples](https://github.com/esp-rs/esp-idf-sys/tree/master/examples) std_basics.rs,  unsafe_call.rs

    - Also include information on alternative flashing and alternative monitoring approach.



* [Rust on ESP32 - Building an RC car in ESP-RS: Using ESP-NOW](https://youtu.be/6gFAKYULHDM?si=9GdLgf51NEjVwTqq)

* [Personal walk through learning embedded systems with ESP32 in Rust Sensors](https://github.com/milewski/sensors-esp) Rotary Encode, Line tracking sensor,  Passiv Buzzer,  Micro SD Card,  Capacitive Switch Sensor, Accelerometer,  MotorDriver,  8X* Matric, LCD 1602A,  DETM Decoding, ADC/DAC,  Mecanum Wheel




## Rust Research Links
  - [tool to create new app template](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html)

  - [espflash explained](https://docs.esp-rs.org/book/tooling/espflash.html)
  

  -  [Rust Safe wrappers for much of the ESP ecosystem](https://crates.io/crates/esp-idf-hal)

  - [ESP-IDF configuration for RUST on ESP32]( https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md)

  - [Targeting rust for RISC-V and Xtensa CPU](https://doc.rust-lang.org/nightly/rustc/platform-support/esp-idf.html)


  - [ESP32 Rust repositories](https://github.com/orgs/esp-rs/repositories)

  - [Installing ESP32 rust toolchain](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)


  
