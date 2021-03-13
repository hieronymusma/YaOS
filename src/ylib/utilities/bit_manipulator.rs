pub trait BitManipulation<T> {
    fn set_bit(&mut self, position: u8, target_value: u8);
    fn get_bit(&self, position: u8) -> T;
    fn is_bit_set(&self, bit: T) -> bool;
}

macro_rules! impl_bitmanipulator {
    ($t:ty) => {
        impl BitManipulation<$t> for $t {
            fn get_bit(&self, position: u8) -> $t {
                (self >> position) & 0x1
            }
        
            fn set_bit(&mut self, position: u8, target_value: u8) {
                const BIT_SIZE: usize = core::mem::size_of::<$t>() * 8;
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

            fn is_bit_set(&self, bit: $t) -> bool {
                if (*self & bit) > 0 {
                    true
                } else {
                    false
                }
            }
        }
    };
}

impl_bitmanipulator!(u8);
impl_bitmanipulator!(u16);
impl_bitmanipulator!(u32);
impl_bitmanipulator!(u64);