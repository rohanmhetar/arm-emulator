pub fn sign_extend(value: u32, bits: u8) -> i32 {
    let sign_bit = 1 << (bits - 1);
    ((value as i32) & (sign_bit - 1)) - ((value as i32) & sign_bit)
}

pub fn rotate_right(value: u32, shift: u8) -> u32 {
    (value >> shift) | (value << (32 - shift))
}
