//! 
//! Measure change in capacitance by measuring time to discharge a 
//! a capacitor from! VCC (3.3V) to logic 0 at approximately 0.7V
//! We Oversample and average this across a large number of passes 
//! to help filter out noise.
//! See Also: ../ex-gpio-measure-time-to-change-high-to-low)
//! - It is similar but uses 2 pins to avoid the need to change pin direction.
//! 
//! WARN: See readme.md for sdkconfig.default settings
//!   This example will not work without those settings
//!   due to erratic timing delays from logging. 
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
//use env_logger::{Builder, Target};
//use esp_idf_sys;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    // Disable informational log messages just in case
    // they are what is affecting our timing.
    //Builder::new()
    //.filter(None, log::LevelFilter::Off)
    //.target(Target::Stdout)
    //.init();

    let peripherals = Peripherals::take()?;

    let led_pin = peripherals.pins.gpio11;
    let sense_pin = peripherals.pins.gpio1;    
    let delay: Delay = Default::default();
    let mut led = PinDriver::output(led_pin)?;
    let mut csense_in = PinDriver::input(sense_pin)?;
    // must be mutable because we are assigning to it
    // latter to avoid moved rule violations.
    
    let num_pass = 250;
      // Oversample number of passes more will
      // reduce variability but takes longer 
      
    let full_discharge_time_ms = 2; 
      // milliseconds to allow full discharge
      // any remaining charge in capacitor will add a 
      // uknown element to how much charging is accomplished
      // during charge_time_us which means it could leave the
      // pin higher for longer modifying the readings.

    let charge_time_us = 500; //250;//150;//75; //100; //315; // micro seconds to allow pin to discharge
     // Charge time is to allow capacitor a level close to
     // VCC once pin drive is set high.  It Should
     // be fast since no resistor between capacitor and
     // sensor.  May consider adding 3.3K resistor between pin
     // and electrode to avoid excessive drain on the pin.  Keep 
     // Increasing until further increases do not cause a material 
     // increase to count at the higher capacitance levels
     // When set to 500 it is sensitive enough to detect a 1" movement of 
     // hand from 10" away from empty bottle.   When set to 25 it doesn't
     // sense the hand until much closer.  Another way to set this value
     // is to use a ocilloscope when the circuit is at it's highest capacitance
     // and set it just long enough so voltage spikes to 90% of VCC.
     // in reality I could not use the scope because it added more capacitance
     // and distorted the readings. 


    loop {                
        led.set_high()?;        
        let mut cout = csense_in.into_output()?; // Change our Pin into output mode
        cout.set_low()?;  // Start discharge cycle
        delay.delay_ms(full_discharge_time_ms); // Wait for long enough to ensure fully discharged          
        csense_in = cout.into_input()?; // have to Move back to keep move/borrow checker happy
        let mut sum = 0_i64;         
        let start = Instant::now();                 
        for _pass in 1..num_pass {
            // Be careful with num_pass and charge_time
            // because a large #pass combined with high 
            // cound can case sum to overflow.
            cout = csense_in.into_output()?; // Change our Pin into output mode            
            cout.set_high()?; // Start charging the capacitor 
            delay.delay_us(charge_time_us);           
            csense_in = cout.into_input()?; 
              // places drive pin into high impedance mode.
              // stops charging capacitor leaving pin in 
              // input mode so we can sense how long it takes
              // to drain.
            let mut cnt = 0;            
            // Wait for capacitor to drain to ground
            // Through bleed resistor.
            while csense_in.is_high() && cnt < 1000000 {
                cnt += 1;
            }
            sum += cnt;
        }
        // Average out our oversamples
        let elap_us = Instant::now().duration_since(start).as_nanos() as f64 / 1000_f64;
        let elap_net_us = elap_us - (num_pass as f64 * charge_time_us as f64);
        // We are mostly interested in the time spent decaying not the 
        // charge time. Our delays are not always 100% consistent due to 
        // RTOS but we average those out in our num_pass.
        let elap_avg_us = elap_net_us / num_pass as f64;
        let elap_net_ms = elap_net_us / 1000_f64;
        let tout = sum / num_pass as i64;                
        println!("elap during pin state change cnt={:?} avg={:.3}us tot={:4}ms", 
           tout, elap_avg_us, elap_net_ms);
        led.set_low()?;                
        
        //unsafe { vTaskDelay(18) };
        delay.delay_ms(18); // Allow WDT to auto clear
          // TODO: TO RESET WATCHDOG IN TIGHT LOOP LIKE THIS
          // without depending on sleep time for auto reset

        delay.delay_ms(1000); // Slow things down so I can read them
    }
}
