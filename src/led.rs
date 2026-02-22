use crate::gpio;
use crate::gpio::GPIOC_BASE;

pub const LED_PIN: u32 = 13;

pub fn led_init() {
    gpio::gpio_enable_clock(GPIOC_BASE);
    gpio::gpio_init_mode(
        GPIOC_BASE,
        LED_PIN,
        gpio::Mode::Output(gpio::OutputType::PushPull),
    );
}

pub fn led_on() {
    gpio::gpio_set_pin(GPIOC_BASE, LED_PIN, gpio::PinState::Low)
}

pub fn led_off() {
    gpio::gpio_set_pin(GPIOC_BASE, LED_PIN, gpio::PinState::High)
}
