use core::ptr;
pub fn led_init(port: u32, pin: u32) {
    let mode_offset = 0;
    let gpio_reg_addr = (port + mode_offset) as *mut u32;

    let bit_position = pin * 2;
    let mode_mask = 0x3 << bit_position;
    let mode_value = 0x1 << bit_position;

    // let mut gpio_mode_reg_value = unsafe { ptr::read_volatile(gpio_reg_addr) };

    // gpio_mode_reg_value &= !(mode_mask);
    // gpio_mode_reg_value |= mode_value;

    unsafe {
        ptr::write_volatile(gpio_reg_addr, mode_value);
    }

    let output_type_offset = 4;
    let gpio_output_type_reg_addr = (port + output_type_offset) as *mut u32;

    let output_type_mask = 0x3 << bit_position;
    let output_type_value = 0x0;

    let mut gpio_output_type_reg_value = unsafe { ptr::read_volatile(gpio_output_type_reg_addr) };

    gpio_output_type_reg_value &= !(output_type_mask);
    gpio_output_type_reg_value |= output_type_value;
    unsafe {
        ptr::write_volatile(gpio_output_type_reg_addr, output_type_value);
    }
}

enum PinState {
    GpioPinHi,
    GpioPinLo,
}

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x400;
pub const GPIOC_BASE: u32 = 0x4002_0800;
pub const GPIOD_BASE: u32 = GPIOC_BASE + 0x400;

pub const RCC_BASE: u32 = 0x4002_3800;

pub unsafe fn read_register(addr: *mut u32) -> u32 {
    ptr::read_volatile(addr)
}

pub unsafe fn write_register(addr: *mut u32, value: u32) {
    ptr::write_volatile(addr, value)
}

pub fn reg_set_bit(reg_addr: *mut u32, bit_position: u32, bit_val: bool) {
    unsafe {
        // Read the current value of the register
        let reg_value = read_register(reg_addr);

        // Set or clear the specific bit based on `bit_val`
        let updated_value = if bit_val {
            reg_value | (1 << bit_position)
        } else {
            reg_value & !(1 << bit_position)
        };

        // Write the modified value back to the register
        write_register(reg_addr, updated_value);
    }
}

pub fn enable_gpio_clock(port: u32) {
    let rcc_abhenr_offset = 0x30;
    let rcc_ahbenr_addr = (RCC_BASE + rcc_abhenr_offset) as *mut u32;

    unsafe {
        ptr::write_volatile(rcc_ahbenr_addr, 0x00000004);
    }
    unsafe { _ = ptr::read_volatile(rcc_ahbenr_addr) }

    // match port {
    //     GPIOA_BASE => {
    //         //enable the 17th bit  of rcc_ahbenr_addr
    //         reg_set_bit(rcc_ahbenr_addr, 0, true);
    //     }

    //     GPIOB_BASE => {
    //         //enable the 18th bit of rcc_ahbenr_addr
    //         reg_set_bit(rcc_ahbenr_addr, 1, true);
    //     }

    //     GPIOC_BASE => {
    //         //enable the 18th bit of rcc_ahbenr_addr
    //         reg_set_bit(rcc_ahbenr_addr, 2, true);
    //     }

    //     _ => {} //catch all pattern, do nothing for values other than GPIOA_BASE
    // }
    // unsafe {
    //     _ = read_register(rcc_ahbenr_addr);
    // }
}

fn gpio_set_pin(port: u32, pin: u32, state: PinState) {
    let gpio_bit_set_reg_offset = 0x18;
    let gpio_output_type_reg_addr = (port + gpio_bit_set_reg_offset) as *mut u32;

    match state {
        PinState::GpioPinHi => unsafe { ptr::write_volatile(gpio_output_type_reg_addr, 1 << pin) },
        PinState::GpioPinLo => unsafe {
            ptr::write_volatile(gpio_output_type_reg_addr, 1 << (pin + 16))
        },
    }
}

pub fn led_on(port: u32, pin: u32) {
    gpio_set_pin(port, pin, PinState::GpioPinLo)
}

pub fn led_off(port: u32, pin: u32) {
    gpio_set_pin(port, pin, PinState::GpioPinHi)
}
