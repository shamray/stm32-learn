#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod button;
mod exti;
mod gpio;
mod led;
mod reg;
mod startup_stm32f401;

use core::panic::PanicInfo;

pub fn delay(duration: u32) {
    for _ in 0..duration {}
}

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
    button::init(button::Mode::Input);

    loop {
        let state = button::read_state();
        match state {
            button::State::Pressed => led::led_on(),
            button::State::Released => blink_n(2),
        }
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
