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
    pub fn set_address(&mut self, address: &PhysicalAddress) {
        assert!(address.value() & !0x000fffff_fffff000 == 0); // Address must be page aligned
        self.0 &= !0x000fffff_fffff000;
        self.0 |= address.value() as u64;
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn get_address(&self) -> PhysicalAddress {
        let address = (self.0 & 0x000fffff_fffff000) as usize;
        PhysicalAddress::new(address)
    }

    pub fn set_writable(&mut self, value: bool) {
        self.0.set_bit(WRITABLE, value as u8);
    }

    pub fn set_executable(&mut self, value: bool) {
        self.0.set_bit(NO_EXECUTE, !value as u8);
    }

    pub fn set_present(&mut self, value: bool) {
        self.0.set_bit(PRESENT, value as u8);
    }

    pub fn is_present(&self) -> bool {
        self.0.get_bit_bool(PRESENT)
    }
}