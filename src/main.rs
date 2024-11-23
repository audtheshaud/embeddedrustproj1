/*

Adrian Nelson November 21 2024: Blinking the blue LED on GPIO pin 15 of the PICO-ICE using the Peripheral Access Crate

To format this code: open a terminal and run cargo fmt

GPIO Pins used: GPIO pin 15 of the RP2040, which was found on the schematic included in this project

The blue led turns on when the logic level is set to low on GPIO pin 15 (Annode)

*/

#![no_std] // Removes the std library
#![no_main] // Removes main() as the program entry point

use cortex_m::asm::nop;
use cortex_m_rt::entry; // Set the entry point
use panic_halt as _; // Minimum panic_halt // nop(); asm command

use core::ptr::write_volatile;

const SIO_BASE_ADDR: usize = 0xD0000000;
const GPIO_SET: *mut u32 = (SIO_BASE_ADDR + 0x14) as *mut u32;
const GPIO_CLR: *mut u32 = (SIO_BASE_ADDR + 0x18) as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        let gpio_use_sio = (0x40014000 + 0x7C) as *mut u32; // Get the memory address using the IO_BANKO 0x40014000 and offset 0x7C for the GPIO15_CTRL
        *gpio_use_sio = 0x5; // FUNCSEL = 5 for SIO
    }
    loop {
        
    }
}
