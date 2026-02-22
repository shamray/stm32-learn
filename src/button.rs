use crate::gpio;
use crate::gpio::GPIOA_BASE;

const BUTTON_PIN: u32 = 0;

pub enum State {
    Released,
    Pressed,
}

pub enum Trigger {
    FallingEdge,
    RisingEdge,
}

pub enum Mode {
    Input,
    Interrupt(Trigger),
}

pub fn init(mode: Mode) {
    gpio::gpio_enable_clock(GPIOA_BASE);

    match mode {
        Mode::Input => {
            gpio::gpio_init_mode(GPIOA_BASE, BUTTON_PIN, gpio::Mode::Input);
        }
        Mode::Interrupt(_) => {}
    }
}

pub fn read_state() -> State {
    if !gpio::gpio_get_pin(GPIOA_BASE, BUTTON_PIN) {
        State::Pressed
    } else {
        State::Released
    }
}
