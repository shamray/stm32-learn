use core::ptr;

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x0400;
pub const GPIOC_BASE: u32 = GPIOA_BASE + 0x0800;
pub const GPIOD_BASE: u32 = GPIOA_BASE + 0x0C00;
pub const GPIOE_BASE: u32 = GPIOA_BASE + 0x1000;
pub const GPIOH_BASE: u32 = GPIOA_BASE + 0x1C00;

pub const RCC_BASE: u32 = 0x4002_3800;

pub enum PinState {
    High,
    Low,
}

pub fn gpio_init(port: u32, pin: u32) {
    let mode_offset = 0;
    let gpio_reg_addr = (port + mode_offset) as *mut u32;

    let bit_position = pin * 2;
    let mode_mask = 0x3 << bit_position;
    let mode_value = 0x1 << bit_position;

    let mut gpio_mode_reg_value = unsafe { ptr::read_volatile(gpio_reg_addr) };

    gpio_mode_reg_value &= !mode_mask;
    gpio_mode_reg_value |= mode_value;

    unsafe {
        ptr::write_volatile(gpio_reg_addr, gpio_mode_reg_value);
    }

    let output_type_offset = 4;
    let gpio_output_type_reg_addr = (port + output_type_offset) as *mut u32;

    let output_type_mask = 0x1 << pin;
    let output_type_value = 0x0;

    let mut gpio_output_type_reg_value = unsafe { ptr::read_volatile(gpio_output_type_reg_addr) };

    gpio_output_type_reg_value &= !output_type_mask;
    gpio_output_type_reg_value |= output_type_value;

    unsafe {
        ptr::write_volatile(gpio_output_type_reg_addr, gpio_output_type_reg_value);
    }
}

fn reg_set_bit(reg_addr: *mut u32, bit_position: u32, bit_val: bool) {
    let reg_value = unsafe { ptr::read_volatile(reg_addr) };

    let updated_value = if bit_val {
        reg_value | (1 << bit_position)
    } else {
        reg_value & !(1 << bit_position)
    };

    unsafe { ptr::write_volatile(reg_addr, updated_value) }
}

pub fn gpio_enable_clock(port: u32) {
    let rcc_ahb1enr_offset = 0x30;
    let rcc_ahb1enr_addr = (RCC_BASE + rcc_ahb1enr_offset) as *mut u32;

    match port {
        GPIOA_BASE => reg_set_bit(rcc_ahb1enr_addr, 0, true),
        GPIOB_BASE => reg_set_bit(rcc_ahb1enr_addr, 1, true),
        GPIOC_BASE => reg_set_bit(rcc_ahb1enr_addr, 2, true),
        GPIOD_BASE => reg_set_bit(rcc_ahb1enr_addr, 2, true),
        GPIOE_BASE => reg_set_bit(rcc_ahb1enr_addr, 2, true),
        GPIOH_BASE => reg_set_bit(rcc_ahb1enr_addr, 2, true),
        _ => {}
    }
    unsafe { _ = ptr::read_volatile(rcc_ahb1enr_addr) }
}

pub fn gpio_set_pin(port: u32, pin: u32, state: PinState) {
    let gpio_bsrr_offset = 0x18;
    let gpio_bsrr_addr = (port + gpio_bsrr_offset) as *mut u32;

    match state {
        PinState::High => unsafe { ptr::write_volatile(gpio_bsrr_addr, 1 << pin) },
        PinState::Low => unsafe { ptr::write_volatile(gpio_bsrr_addr, 1 << (pin + 16)) },
    }
}
