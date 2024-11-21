/*

Adrian Nelson November 21 2024: Blinking the blue LED on GPIO pin 15 of the PICO-ICE using the Peripheral Access Crate

To format this code: open a terminal and run cargo fmt

GPIO Pins used: GPIO 15 of the RP2040, which was found on the schemati included in this project

*/

#![no_std] // Removes the std library
#![no_main] // Removes main() as the program entry point

use ::cortex_m_rt::entry; // Set the entry point
use ::panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}
