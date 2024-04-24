## Enviornmental Sensing

- **[Sensing Variable capacitance changes like Water level changes with Single Pin and resistor](https://github.com/joeatbayes/embedded-rust-examples/tree/main/ex-gpio-change-pin-direction-var-cap-read-1-pin)
  Charges a variable capacitor towards VCC and measures time to for voltage to drop to logic low transition level while using only 1 pin
  and 1 resistor. A good  way to add touch or poximity sensing to any CPU that has a spare I/O pin. It Demonstrates the following: *esp-idf-hal*
    - Change GPIO Pin Direction
    - Read GPIO Status
    - Mesure variable capacitance
    - Using Polling to detect GPIO status change
    - Measure elapsed time in Microseconds(uS)
    - Measure elapsed time in Nanosecods
    - Delay for Micro seconds and milli seconds    
    - Disable debug logging
    - Isolate pin Assignments for each change between boards
    - Change default watchdog timer in default.config
    - Change minimum freetos task sleep down to 1ms

- **[Sensing Variable Capacitance with 2 pins one of which must be High Drive Open drain ](https://github.com/joeatbayes/embedded-rust-examples/tree/main/ex-gpio-measure-time-to-change-high-to-low)**
  This project aims to measure changes in capacitance using a foil strip sensor taped to a bottle. The idea is to track the time it
  takes for a circuit to charge from 0V to a logic high transition voltage around 0.7V.The theory is that a higher capacitance, caused
  by increased water level, should take longer to charge. Conversely, a lower capacitance due to decreased water level should charge faster.
  An advantage of this circuit is the high drive pin can drain the capacitor very fast which means the circuit can cycle fast.*uses esp-idf-hal* 
    - Measure elapsed time    
    - Detect change in GPIO state
    - Demonstrate Open Drain high-impedance pins 
    - Delay for a specific duration
    - Change OpenDrive GPIO pin mode between Drain and high impedance
    - Oversampling to improve reading stability 

  **[RS485 Read/Write with 1/2 Duplex transceiver](https://github.com/joeatbayes/embedded-rust-examples/tree/main/ex-uart-echo-rs485)**: Reads uart 1 byte
  at a time and writing data back to sender 1 byte at a time with write enable
  pin control to allow use of 1/2 duplex transceiver.  RS485 is heavily used in enviornmental
  sensing especially when we need to deliver data several hundred feet through metal rich soil *esp-hal*

- **[UART - Process inbound uart data while also doing other work](https://github.com/joeatbayes/embedded-rust-examples/tree/main/ex-uart-non-block-rs485)**:
  Include code that shows how to control Write Enable pin for 1/2 duplex RS485
  transceiver.   The Real value in this example is showing how to use non-blocking
  fast polling to accomplish what is normally done with async threads.  In some instances the
  fast polling can deliver more predictable results since you have more control over which
  code is receiving time slices than you do with threading.  *esp-idf-hal*

