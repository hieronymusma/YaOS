pub trait BitManipulation {
    fn set_bit(&mut self, bit_position: T, target_value: bool);
}

impl BitManipulation for u8 {
    fn set_bit(&mut self, bit_position: u8, target_value: bool) {
        // const bit_size: usize = core::mem::size_of::<u8>() * 8;
        // if usize::from(bit_position) > bit_size {
        //     panic!("bit_positon is {}; But type has only {} bits", bit_position, bit_size);
        // }
        if target_value {
            *self |= 1 << bit_position;
        } else {
            *self &= !(1 << bit_position);
        }
    }
}

macro_rules! bit_manipulation_impl {
    ($t:ty) => {
        impl<$t> BitManipulation<$t> for $t {
            fn set_bit(&mut self, bit_position: $t, target_value: bool) {
                const bit_size: usize = core::mem::size_of::<$t>() * 8;
                if usize::from(bit_position) > bit_size {
                    panic!(
                        "bit_positon is {}; But type has only {} bits",
                        bit_position, bit_size
                    );
                }
                if target_value {
                    *self |= 1 << bit_position;
                } else {
                    *self &= !(1 << bit_position);
                }
            }
        }
    };
}

// bit_manipulation_impl!(u8);
// bit_manipulation_impl!(u16);
// bit_manipulation_impl!(u32);
// bit_manipulation_impl!(u64);
