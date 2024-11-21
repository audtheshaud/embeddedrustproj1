# Embedded Rust Project 1: Using the Peripheral Access Crate
### Utilizing my RP2040 ICE40UP5K, I am going to use "unsafe" Rust through the form of the PAC (Peripheral Access Crate) to blink an LED
### How does this work?
1. By examining the data sheet of the RP2040 ICE40UP5K I need to find the pins that control the annode and cathode of the LED and manipulate their values to be either logic level 1 or logic level 0
2. From there I need to set up the Rust project to inlcude certain crates as well as set the proper target for the Rust compiler
3. In my Rust project I need to dereference the pointers of the memory addresses where the pin values live in order to set them accordingly
4. I then use an unsafe block to properly set the values using volatile_write(), making sure the compiler does not optimize this code out'
5. Finally I loop using nop() to toggle the LED state on and off
