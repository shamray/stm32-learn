use core::ptr;

pub fn addr(base: u32, offset: u32) -> *mut u32 {
    (base + offset) as *mut u32
}

pub fn set_bit(reg_addr: *mut u32, bit_position: u32, bit_val: bool) {
    let reg_value = unsafe { ptr::read_volatile(reg_addr) };

    let updated_value = if bit_val {
        reg_value | (1 << bit_position)
    } else {
        reg_value & !(1 << bit_position)
    };

    unsafe { ptr::write_volatile(reg_addr, updated_value) }
}

pub fn set_bits(reg_addr: *mut u32, bit_position: u32, bit_val: u32, bit_mask: u32) {
    let mut reg_value = unsafe { ptr::read_volatile(reg_addr) };

    let mode_mask = bit_mask << bit_position;
    let mode_value = bit_val << bit_position;

    reg_value &= !mode_mask;
    reg_value |= mode_value;

    unsafe { ptr::write_volatile(reg_addr, reg_value) }
}

pub fn get_bit(reg_addr: *mut u32, bit: u32) -> bool {
    let reg_value = unsafe { ptr::read_volatile(reg_addr) };
    (reg_value & (1 << bit)) != 0
}
