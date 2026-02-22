use crate::exti;
use crate::gpio;
use crate::gpio::GPIOA_BASE;
use crate::proc;

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
    gpio::gpio_init_mode(GPIOA_BASE, BUTTON_PIN, gpio::Mode::Input);
    exti::init_syscfg(GPIOA_BASE, BUTTON_PIN);

    match mode {
        Mode::Input => {}
        Mode::Interrupt(trigger) => {
            match trigger {
                Trigger::FallingEdge => {
                    exti::set_edge(BUTTON_PIN, exti::EdgeTrigger::Falling);
                }
                Trigger::RisingEdge => {
                    exti::set_edge(BUTTON_PIN, exti::EdgeTrigger::Rising);
                }
            }
            exti::enable_interrupt(BUTTON_PIN);
            proc::enable_irq(exti::EXTI0_IRQ);
        }
    }
}

pub fn read_state() -> State {
    if !gpio::gpio_get_pin(GPIOA_BASE, BUTTON_PIN) {
        State::Pressed
    } else {
        State::Released
    }
}

pub fn on_clicked() {
    exti::clear_pending_interrupt(BUTTON_PIN);
}
