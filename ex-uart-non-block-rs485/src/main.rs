//! Non Blocking fast poll Uart RS485 1/2 duplex echo for RS485 style transciever
//! test
//!
//! Folowing pins are used:
//! TX    GPIO43
//! RX    GPIO44
//!//!
//! This example transfers data via UART.
//! Connect TX and RX pins to see the outgoing data is read as incoming data.
//! Echo characters sent from terminal such as linux screen. 
#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use esp_idf_hal::delay::NON_BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_sys;
// TODO:  Figure out how to make WDT work esp_idf_hal https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/watchdog.rs
// Until then disable WDT in menu config

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    let peripherals = Peripherals::take()?;

    let tx = peripherals.pins.gpio43;
    let rx = peripherals.pins.gpio44;
    let led_pin = peripherals.pins.gpio11;
    let rs_48_en_pin = peripherals.pins.gpio21;
    let delay: Delay = Default::default();
    let mut led = PinDriver::output(led_pin)?;
    let mut rs48_enable = PinDriver::output(rs_48_en_pin)?;

    println!("Starting UART loopback test");
    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;
    
    loop {
        let mut buf = [0_u8; 1];        
        uart.read(&mut buf, NON_BLOCK)?;
        // we know read will return a 0 in buf 
        // when nothing is available so we ignore 
        if buf[0] != 0 {            
          println!("Written 0xaa, read {:?} {:?} 0x{:02x}", buf[0] as char, buf[0],buf[0]);
          rs48_enable.set_high()?;
          delay.delay_us(150); // need to allow transceiver time to enter active mode

          // Calling Write directly causes mutliple garbage characters to 
          // be sent.  I think the uart is not properly managing it's buffer.
          //uart.write(&buf)?;
          //uart.wait_tx_done(150);

          uart.write_nb(&mut buf)?;          
          delay.delay_ms(1);
          //buf[0] = 0;                    
          rs48_enable.set_low()?;
          led.toggle()?;
        }
        // Only need this delay because watchdog fires otherwise
        delay.delay_ms(18); // TODO: TO RESET WATCHDOG IN TIGHT LOOP LIKE THIS USING esp_idf_hal

        // TODO: DO SOME OTHER WORK SINCE WE ARE NO LONGER BLOCKING
        // ON DATA IN UART AVAILABILITY
                
    }
}
