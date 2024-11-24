/*

Adrian Nelson November 21 2024: Blinking the blue LED on GPIO pin 15 of the PICO-ICE using the Peripheral Access Crate

To format this code: open a terminal and run cargo fmt

GPIO Pins used: GPIO pin 15 of the RP2040, which was found on the schematic included in this project

The blue led turns on when the logic level is set to low on GPIO pin 15 (Annode)

*/

#![no_std] // Removes the std library
#![no_main] // Removes main() as the program entry point

use cortex_m::asm::nop; // nop(); asm command
use cortex_m_rt::entry;
use panic_halt as _; // Minimum panic_halt

use core::ptr::write_volatile;
const GPIO_OE_SET: usize = 0x24;
const GPIO_OUT_SET: usize = 0x14;
const GPIO_OUT_CLR: usize = 0x18;
const SIO_BASE_ADDR: usize = 0xD0000000;

const FUNC_GPIO_SET: *mut u32 = (SIO_BASE_ADDR + GPIO_OUT_SET) as *mut u32;
const FUNC_GPIO_CLR: *mut u32 = (SIO_BASE_ADDR + GPIO_OUT_CLR) as *mut u32;


const IO_BANK0: usize = 0x40014000;
const GPIO15_CTRL: usize = 0x7C;
const GPIO_15: usize = 15;

#[entry]
fn main() -> ! {
    unsafe {
        write_volatile((IO_BANK0 + GPIO15_CTRL) as *mut u32, 0x5); // Set SIO function of the GPIO pin 15 function MUX by setting value to 5
        write_volatile((SIO_BASE_ADDR + GPIO_OE_SET) as *mut u32, 1 << GPIO_15); // Set GPIO pin 15 output
    }
    loop {
        // Turn GPIO15 (LED) ON
        unsafe { write_volatile(FUNC_GPIO_SET, 1 << GPIO_15); } // Writes a 1 to clear the output of GPIO15

        // Delay
        for _ in 0..100_000 {
            nop();
        }

        // Turn GPIO15 (LED) OFF
        unsafe {write_volatile(FUNC_GPIO_CLR, 1 << GPIO_15); } // Writes a 1 to set the output of GPIO15

        // Delay
        for _ in 0..100_000 {
            nop();
        }
    }
}
