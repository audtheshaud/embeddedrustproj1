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
const GPIO_OUT_SET: usize = 0x14;
const GPIO_OUT_CLR: usize = 0x18;
const SIO_BASE_ADDR: usize = 0xD0000000;

const FUNC_GPIO_SET: *mut u32 = (SIO_BASE_ADDR + GPIO_OUT_SET) as *mut u32;
const FUNC_GPIO_CLR: *mut u32 = (SIO_BASE_ADDR + GPIO_OUT_CLR) as *mut u32;


const IO_BANK0: usize = 0x40014000;
const GPIO15_CTRL: usize = 0x7C;

#[entry]
fn main() -> ! {
    unsafe {
        
        let gpio_use_sio = (IO_BANK0 + GPIO15_CTRL) as *mut u32; // Get the memory address using the IO_BANKO 0x40014000 and GPIO15_CTRL 0x7C
        *gpio_use_sio = 0x5; // FUNCSEL = 5 for SIO
    }
    loop {
        // Turn GPIO15 (LED) ON
        unsafe { GPIO_OUT_CLR.write_volatile(1 << 15); }

        // Delay
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }

        // Turn GPIO15 (LED) OFF
        unsafe { GPIO_OUT_SET.write_volatile(1 << 15); }
    }
}
