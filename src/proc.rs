use crate::reg;

pub const NVIC_BASE: u32 = 0xE000_E100;

pub const NVIC_ISER: u32 = 0x00;
pub const NVIC_ICER: u32 = 0x80;

pub fn enable_irq(irq_number: u32) {
    let offset = (irq_number / 32) * 4;
    let bit_position = irq_number % 32;

    let iser_addr = reg::addr(NVIC_BASE + NVIC_ISER, offset);
    reg::set_bit(iser_addr, bit_position, true);
}

pub fn disable_irq(irq_number: u32) {
    let offset = (irq_number / 32) * 4;
    let bit_position = irq_number % 32;

    let icer_addr = reg::addr(NVIC_BASE + NVIC_ICER, offset);
    reg::set_bit(icer_addr, bit_position, true);
}
