# Rust example Turn LED or Motor On/Off and toggle LED state
### Tested on esp32-s3-mini-1 on custom board 2024-04-18

- [Rust source main.rs](src/main.rs)
- [Cargo.toml](Cargo.toml)

# Expected Wiring
  LED Pin on gpio-11
  GPIO11 ->  LED ->  4.7K resistor -> Ground

  
  If LED does not blink try flipping it around since 
  current can only flow 1 way thorugh a LED.


  I used GPIO 11 on a my custom board.  You will need to find the 
  LED pin for your board and use it. 


NOTE: This version uses esp_hal instead of esp_idf_hal because I planned to use the esp_hal in other parts of our project and it seemed more general purpose. 