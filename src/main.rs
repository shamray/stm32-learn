#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod gpio;
mod led;
mod startup_stm32f401;

use core::panic::PanicInfo;

use crate::led::*;

pub fn delay(duration: u32) {
    for _ in 0..duration {}
}

pub fn blink_n(n: u8) {
    const INTERVAL: u32 = 24000;

    for i in 0..8 {
        if i < n {
            led_on();
        }
        delay(INTERVAL);
        led_off();
        delay(INTERVAL);
    }

    delay(INTERVAL * 4);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    led_init();

    loop {
        blink_n(2);
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
