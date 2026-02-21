#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod led;
mod startup_stm32f401;

use core::panic::PanicInfo;

use crate::led::*;

pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = GPIO_PIN_0 << 1;
pub const GPIO_PIN_13: u32 = 0x2000;

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x0400;
pub const GPIOC_BASE: u32 = GPIOA_BASE + 0x0800;
pub const GPIOD_BASE: u32 = GPIOA_BASE + 0x0C00;
pub const GPIOE_BASE: u32 = GPIOA_BASE + 0x1000;
pub const GPIOH_BASE: u32 = GPIOA_BASE + 0x1C00;

pub const LED_PIN: u32 = GPIO_PIN_13;
pub const LED_PORT: u32 = GPIOC_BASE;

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
        blink_n(3);
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
