#![no_std]
#![no_main]

// Demonstrate turning a LED PIN on and Off and toggeling it's state
// Tested with ESP32-S3 mini  2024-04-18
//            cargo 1.77.2 (e52e36006 2024-03-26)
//
// See Also: https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/gpio_interrupt.rs


use esp_backtrace as _;
use esp_hal::{IO, clock::ClockControl, peripherals::Peripherals,  prelude::*, Delay};
use esp_println::println;

#[entry]
fn main() -> ! {
    
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // This pin is for my custom board.  Use the gpio#
    // approapriate for you'r board.  Note while this says 
    // led it could be any device you want to control with 
    // a GPIO pin. 
    let mut led = io.pins.gpio11.into_push_pull_output();

    //let peripherals = Peripherals::take().unwrap();
    //let mut led = PinDriver::output(peripherals.pins.gpio4).unwrap();


    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Led Blink Test!");
    loop {
        println!("Leed Loop...");
        let _ = led.set_high();
        delay.delay_ms(500u32);
        let _ = led.set_low();
        delay.delay_ms(500u32);
        let _ = led.toggle();
        delay.delay_ms(200u32);
        
    }
}
