use super::super::privilege_level::PrivilegeLevel;
use crate::ylib::utilities::bit_manipulator::BitManipulation;

const CONFORMING_BIT: u64 = (1 << 42);
const EXECUTABLE_BIT: u64 = (1 << 43);
const DESCRIPTOR_TYPE: u64 = (1 << 44);
const PRIVILEGE_LEVEL_BIT_0: u64 = (1 << 45);
const PRIVILEGE_LEVEL_BIT_1: u64 = (1 << 46);
const PRESENT_BIT: u64 = (1 << 47);
const CODE_SEGMENT_BIT: u64 = (1 << 53);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Entry(u64);

impl Entry {
    pub fn empty() -> Self {
        Entry(0)
    }

    pub fn null_segment() -> Entry {
        Entry::empty()
    }

    pub fn kernel_code_segment() -> Entry {
        let mut entry = Entry::empty();
        entry.0 |= EXECUTABLE_BIT;
        entry.0 |= DESCRIPTOR_TYPE; // CODE_SEGMENT
        entry.0 |= PRESENT_BIT;
        entry.0 |= CODE_SEGMENT_BIT;
        entry
    }
}
