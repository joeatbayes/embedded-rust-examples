//! WARN: Broken until we can figure out how to 
//! turn of GPIO Direction Change Logging.
//! Use .... ../ex-gpio-measure-time-to-change-high-to-low)
// - It is similar but uses 2 pins to avoid the need to change pin direction.
/* SAMPLE OF LOG MESSAGES I CAN NOT DISABLE
I (346) sleep: Configure to isolate all GPIO pins in sleep state
I (353) sleep: Enable automatic switching of GPIO sleep configuration
I (361) main_task: Started on CPU0
I (371) main_task: Calling app_main()
I (371) gpio: GPIO[11]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (381) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (391) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (411) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (411) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (411) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown: 0| Intr:0 
I (421) gpio: GPIO[1]| InputEn: 0| OutputEn: 0| OpenDrain: 0| Pullup: 0| Pulldown
*/
//! 
//! 
//! 
//! Measure change in capacitance by measuring time to discharge a 
//! a capacitor from! VCC (3.3V) to logic 0 at approximately 0.7V
//! We Oversample and average this across a large number of passes 
//! to help filter out noise.
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
    
    let num_pass = 1000;
      // Oversample number of passes more will
      // reduce variability but takes longer 
      
    let full_discharge_time_ms = 2; 
      // milliseconds to allow full discharge
      // any remaining charge in capacitor will add a 
      // uknown element to how much charging is accomplished
      // during charge_time_us which means it could leave the
      // pin higher for longer modifying the readings.

    let charge_time_us = 25; // micro seconds to allow pin to discharge
     // Charge time is to allow capacitor a level close to
     // VCC once pin drive is set high.  It Should
     // be fast since no resistor between capacitor and
     // sensor.  May consider adding 3.3K resistor between pin
     // and electrode to avoid excessive drain on the pin. 

    loop {                
        led.set_high()?;        
        let mut cout = csense_in.into_output()?; // Change our Pin into output mode
        cout.set_low()?;  // Start discharge cycle
        delay.delay_ms(full_discharge_time_ms); // Wait for long enough to ensure fully discharged          
        csense_in = cout.into_input()?; // have to Move back to keep move/borrow checker happy
        let mut sum = 0;          
        let start = Instant::now();                 
        for _pass in 1..1000 {
            cout = csense_in.into_output()?; // Change our Pin into output mode            
            cout.set_high()?; // Start charging the capacitor 
            delay.delay_us(charge_time_us);           
            csense_in = cout.into_input()?; 
              // places drive pin into high impedance mode.
              // stops charging capacitor leaving pin in 
              // input mode so we can sense hole long it takes
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
        let elap1 = Instant::now().duration_since(start).as_nanos();  
        let tout = sum / num_pass;                
        println!("elap during pin state change cnt={:?} {:?}ms", tout, ((elap1 as f64 / num_pass as f64 ) / 1000000_f64));
        led.set_low()?;                
        
        delay.delay_ms(18); // Allow WDT to auto clear
          // TODO: TO RESET WATCHDOG IN TIGHT LOOP LIKE THIS
          // without depending on sleep time for auto reset

        delay.delay_ms(1000); // Slow things down so I can read them
    }
}
