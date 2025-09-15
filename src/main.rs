#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod startup_stm32f401;

use core::panic::PanicInfo;

static mut SCORES_GLOBAL: [i32; 5] = [1, 2, 3, 4, 5];
const _NUMBERS: [i32; 5] = [9, 8, 7, 6, 5];
static mut BUFFER: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut _total_score = 0;

    unsafe {
        for score in SCORES_GLOBAL {
            _total_score += score;
        }
    }

    unsafe {
        BUFFER[0] = 42;
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
