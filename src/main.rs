#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod led;
mod startup_stm32f401;

use core::{panic::PanicInfo, ptr};

use crate::led::*;

pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = GPIO_PIN_0 << 1;
pub const GPIO_PIN_13: u32 = 0x2000;

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x400;
pub const GPIOC_BASE: u32 = 0x4002_0800;

pub const LED_PIN: u32 = GPIO_PIN_13;
pub const LED_PORT: u32 = GPIOC_BASE;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // unsafe {
    //     ptr::write_volatile(0x4002_3830 as *mut u32, 0x00000004);
    // }
    // unsafe { _ = ptr::read_volatile(0x4002_3830 as *mut u32) }

    // unsafe {
    //     ptr::write_volatile(0x4002_0800 as *mut u32, 0x04000000);
    // }

    enable_gpio_clock(GPIOC_BASE);
    led_init(GPIOC_BASE, 13);

    const INTERVAL: u32 = 64000;

    loop {
        led_on(GPIOC_BASE, 13);
        for _ in 0..INTERVAL {}
        led_off(GPIOC_BASE, 13);
        for _ in 0..INTERVAL {}
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
