use core::ptr;

use crate::{gpio::GPIOA_BASE, reg};

pub const EXTI_BASE: u32 = 0x4001_3C00;
pub const SYSCFG_BASE: u32 = 0x4001_3800;
pub const RCC_BASE: u32 = 0x4002_3800;
pub const EXTI0_IRQ: u32 = 6;
pub enum EdgeTrigger {
    Rising,
    Falling,
}

fn enable_syscfg_clock() {
    let rcc_apb2enr_addr = reg::addr(RCC_BASE, 0x44);
    // Set bit 14 to enable SYSCFG clock
    reg::set_bit(rcc_apb2enr_addr, 14, true);

    unsafe {
        core::ptr::read_volatile(rcc_apb2enr_addr);
    }
}

pub fn set_edge(pin: u32, trigger: EdgeTrigger) {
    let rtsr_addr = reg::addr(EXTI_BASE, 0x08);
    let ftsr_addr = reg::addr(EXTI_BASE, 0x0C);

    match trigger {
        EdgeTrigger::Falling => reg::set_bit(ftsr_addr, pin, true),
        EdgeTrigger::Rising => reg::set_bit(rtsr_addr, pin, true),
    }
}

fn init_interrupt(line: u32, unmask: bool) {
    let exti_imr_addr = reg::addr(EXTI_BASE, 0x00);
    reg::set_bit(exti_imr_addr, line, unmask);
}

pub fn enable_interrupt(line: u32) {
    init_interrupt(line, true);
}

pub fn init_syscfg(port: u32, pin: u32) {
    enable_syscfg_clock();

    let reg_offset = (pin / 4) * 4;
    let exti_imr_addr = reg::addr(SYSCFG_BASE, 0x08 + reg_offset);
    let bit_position = (pin % 4) * 4;

    let value = match port {
        GPIOA_BASE => Some(0),
        _ => None,
    };

    if let Some(value) = value {
        reg::set_bits(exti_imr_addr, bit_position, value, 0xF);
    }
}

pub fn clear_pending_interrupt(line: u32) {
    let exti_pr_addr = reg::addr(EXTI_BASE, 0x14);

    unsafe {
        ptr::write_volatile(exti_pr_addr, 1 << line);
    }
}
