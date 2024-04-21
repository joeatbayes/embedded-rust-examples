//! Measure change in capacitance by measuring time to charge a capacitor from
//! 0 to transition voltage from logic 0 to logic 1 at approximately 0.7V
//! We average this across a large number of passes to help filter out noise.
//! 
//! See readme.md for pin mapping and explanation
#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use std::time::{Duration, Instant};
//use esp_idf_sys;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    let peripherals = Peripherals::take()?;

    let led_pin = peripherals.pins.gpio11;
    let sense_pin = peripherals.pins.gpio1;
    let sense_drv_pin = peripherals.pins.gpio2;
    let delay: Delay = Default::default();
    let mut led = PinDriver::output(led_pin)?;
    let csense_in = PinDriver::input(sense_pin)?;
    let mut csense_drv = PinDriver::input_output_od(sense_drv_pin)?;
    csense_drv.set_low()?;
    let discharge_time = 25; // micro seconds to allow pin to discharge
     // Time to discharge before starting sensing 
     // read.  There is no resistor between this csense_drv
     // and var capacitor so discharge should be very fast.

    loop {                
        led.set_high()?;
        
        // Measure time for signal to rise from 0V to logic 1 which should 
        // be at about 0.7V. Average our output across num_pass to help filter out noise.
        let mut sum = 0;  
        let num_pass = 1000;
        let start = Instant::now();                 
        for _pass in 1..1000 {
            csense_drv.set_low()?; // will drain capacitor pin
            delay.delay_us(discharge_time);  // Charge the cap pin       
            csense_drv.set_high()?; // places drive pin into high impedance mode.                
            let mut cnt = 0;            
            while csense_in.is_low() {
                cnt += 1;
            }
            sum += cnt;
        }
        let elap1 = Instant::now().duration_since(start).as_nanos();  
        let tout = sum / num_pass;
                
        println!("elap during pin state change cnt={:?} {:?}ms", tout, ((elap1 as f64 / num_pass as f64 ) / 1000000_f64));
        led.set_low()?;        
              
        delay.delay_ms(18); // TODO: TO RESET WATCHDOG IN TIGHT LOOP LIKE THIS USING esp_idf_hal

        delay.delay_ms(1000); // Slow things down so I can read them
    }
}
