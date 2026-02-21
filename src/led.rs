use crate::gpio::{GPIOC_BASE, PinState, gpio_enable_clock, gpio_init, gpio_set_pin};

pub const LED_PIN: u32 = 13;

pub fn led_init() {
    gpio_enable_clock(GPIOC_BASE);
    gpio_init(GPIOC_BASE, LED_PIN);
}

pub fn led_on() {
    gpio_set_pin(GPIOC_BASE, LED_PIN, PinState::Low)
}

pub fn led_off() {
    gpio_set_pin(GPIOC_BASE, LED_PIN, PinState::High)
}
