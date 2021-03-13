use core::{num, ops::BitAnd};

pub trait BitManipulation {
    fn set_bit(&mut self, position: u8, target_value: u8);
    fn get_bit(&self, position: u8) -> u8;
}

impl BitManipulation for u8 {
    fn get_bit(&self, position: u8) -> u8 {
        (self >> position) & 0x1
    }

    fn set_bit(&mut self, position: u8, target_value: u8) {
        const BIT_SIZE: usize = core::mem::size_of::<u8>() * 8;
        if usize::from(position) > BIT_SIZE {
            panic!(
                "bit_positon is {}; But type has only {} bits",
                position, BIT_SIZE
            );
        }
        if target_value > 1 {
            panic!("target_value can only be a bit value.");
        }
        if target_value == 1 {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }
}

pub fn is_bit_set(value: u64, bit: u64) -> bool {
    if (value & bit) > 0 {
        true
    } else {
        false
    }
}