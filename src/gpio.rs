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

#[allow(dead_code)]
pub enum OutputType {
    PushPull,
    OpenDrain,
}

pub enum Mode {
    Input,
    Output(OutputType),
}

fn reg_addr(base: u32, offset: u32) -> *mut u32 {
    (base + offset) as *mut u32
}

pub fn gpio_init_mode(port: u32, pin: u32, mode: Mode) {
    let gpio_mode_reg_addr = reg_addr(port, 0x00);
    let mode_value = match mode {
        Mode::Input => 0x0,
        Mode::Output(_) => 0x1,
    };
    reg_set_bits(gpio_mode_reg_addr, pin * 2, mode_value, 0x3);

    match mode {
        Mode::Output(t) => {
            let gpio_output_type_reg_addr = reg_addr(port, 0x04);
            let output_type_value = match t {
                OutputType::PushPull => 0x0,
                OutputType::OpenDrain => 0x1,
            };
            reg_set_bits(gpio_output_type_reg_addr, pin, output_type_value, 0x1);
        }
        Mode::Input => {
            let gpio_pupd_reg_addr = reg_addr(port, 0x0C);
            reg_set_bits(gpio_pupd_reg_addr, pin, 0x1, 0x3);
        }
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

fn reg_set_bits(reg_addr: *mut u32, bit_position: u32, bit_val: u32, bit_mask: u32) {
    let mut reg_value = unsafe { ptr::read_volatile(reg_addr) };

    let mode_mask = bit_mask << bit_position;
    let mode_value = bit_val << bit_position;

    reg_value &= !mode_mask;
    reg_value |= mode_value;

    unsafe { ptr::write_volatile(reg_addr, reg_value) }
}

pub fn reg_get_bit(reg_addr: *mut u32, bit: u32) -> bool {
    unsafe {
        let reg_value = ptr::read_volatile(reg_addr);
        (reg_value & (1 << bit)) != 0
    }
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

pub fn gpio_get_pin(port: u32, pin: u32) -> bool {
    let gpio_idr_addr = reg_addr(port, 0x10);
    reg_get_bit(gpio_idr_addr, pin)
}
