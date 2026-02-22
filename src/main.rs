#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod button;
mod exti;
mod gpio;
mod led;
mod proc;
mod reg;
mod startup_stm32f401;

use core::panic::PanicInfo;

pub fn delay(duration: u32) {
    for _ in 0..duration {}
}

static mut BLINKS: u8 = 1;

pub fn blink_n(n: u8) {
    const INTERVAL: u32 = 24000;

    led::led_off();
    delay(INTERVAL * 4);

    for i in 0..8 {
        if i < n {
            led::led_on();
        }
        delay(INTERVAL);
        led::led_off();
        delay(INTERVAL);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    led::init();
    button::init(button::Mode::Interrupt(button::Trigger::FallingEdge));

    loop {
        let blinks = unsafe { BLINKS };
        blink_n(blinks);
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn EXTI0_Handler() {
    unsafe {
        BLINKS = BLINKS + 1;
        if BLINKS > 4 {
            BLINKS = 1;
        }
    }
    button::on_clicked();
}
