use crate::{memory::physical_address::PhysicalAddress, ylib::utilities::bit_manipulator::BitManipulation};

const PRESENT: u8 = 0;
const WRITABLE: u8 = 1;
const USER_ACCESSIBLE: u8 = 2;
const WRITE_TRHOUGH_CACHING: u8 = 3;
const DISABLE_CACHE: u8 = 4;
const CPU_ACCESSED: u8 = 5;
const CPU_DIRTY: u8 = 6;
const HUGE_PAGE: u8 = 7;
const GLOBAL: u8 = 8;
const NO_EXECUTE: u8 = 63;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn invalid() -> Self {
        PageTableEntry(0)
    }

    pub fn set_address(&mut self, address: PhysicalAddress) {
        assert_eq!(address.value() & 0xfff, 0); // Address must be page aligned
    }

    pub fn set_writable(&mut self) {
        self.0.set_bit(WRITABLE, 1);
        self.0.set_bit(NO_EXECUTE, 1);
    }

    pub fn set_executable(&mut self) {
        self.0.set_bit(WRITABLE, 0);
        self.0.set_bit(NO_EXECUTE, 0);
    }

    pub fn set_present(&mut self) {
        self.0.set_bit(PRESENT, 1);
    }
}