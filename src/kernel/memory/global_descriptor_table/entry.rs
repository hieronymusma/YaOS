use crate::ylib::utilities::bit_manipulator::BitManipulation;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Entry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: Access,
    limit_and_flags: u8,
    base_high: u8,
}

impl Entry {
    pub fn empty() -> Self {
        Entry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: Access::new(),
            limit_and_flags: 0,
            base_high: 0,
        }
    }

    pub fn set_limit(&mut self, limit: u32) {}

    pub fn set_base(&mut self, base: u32) {}

    pub fn set_access(&mut self, access: u8) {}
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Access(u8);

impl Access {
    pub fn new() -> Access {
        Access(0)
    }

    pub fn set_present(&mut self, is_present: bool) {
        self.0.set_bit(0, true);
    }
}
