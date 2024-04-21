//! Simple RS485 Uart Echo with write control
//! to enable 1/2 duplex transceiver chip
//! See readme.md for wiring and setup
// 
// Tested with ESP32-S3 mini  2024-04-18
//        cargo 1.77.2 (e52e36006 2024-03-26)

#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(dead_code)]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay,
    delay::Delay,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    uart::{config::Config, TxRxPins, Uart},
    timer,
};
use esp_println::println;
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio43.into_push_pull_output(),
        io.pins.gpio44.into_floating_input(),
    );
    let mut led = io.pins.gpio11.into_push_pull_output();
    let mut rs48_enable = io.pins.gpio21.into_push_pull_output();


    let mut serial1 = Uart::new_with_config(
        peripherals.UART1,
        Config::default(),
        Some(pins),
        &clocks,
        None );

    println!("Start");
    //let rx,tx = serial1.split();
    loop {                  
          let read = block!(serial1.read_byte());          
          let _ = led.toggle();
          match read {
              Ok(read) => { 
                 println!("Read 0x{:02x}", read);

                 let _ = rs48_enable.set_high(); // Enabled write mode on our
                                         // half duplex RS485 transceiver

                 let _ = serial1.write_byte(read);  // Write our byte back to caller
                 
                 delay.delay_millis(1u32); // give time to send the byte

                 let _ = rs48_enable.set_low(); // Disable write mode so we can 
                                        // receive characters.
                 },
              Err(err) => println!("Error {:?}", err),
          }
        //}

        delay.delay_millis(250u32);
    }
}