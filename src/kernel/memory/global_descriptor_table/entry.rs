use crate::{
    memory::task_state_segment::task_state_segment::TaskStateSegment,
    ylib::utilities::bit_manipulator::BitManipulation,
};

#[allow(dead_code)]
const CONFORMING_BIT: u64 = 1 << 42;
const EXECUTABLE_BIT: u64 = 1 << 43;
const DESCRIPTOR_TYPE: u64 = 1 << 44;
#[allow(dead_code)]
const PRIVILEGE_LEVEL_BIT_0: u64 = 1 << 45;
#[allow(dead_code)]
const PRIVILEGE_LEVEL_BIT_1: u64 = 1 << 46;
const PRESENT_BIT: u64 = 1 << 47;
const CODE_SEGMENT_BIT: u64 = 1 << 53;

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

    pub fn tss_segment(tss: &'static TaskStateSegment) -> [Entry; 2] {
        let mut low_entry = Entry::empty();
        let mut high_entry = Entry::empty();

        // Base address
        let ptr = tss as *const TaskStateSegment;
        let ptr = ptr as u64;

        low_entry.0 |= (ptr << 16) & (0xffff << 16);
        low_entry.0 |= ((ptr >> 16) << 32) & (0xff << 32);
        low_entry.0 |= ((ptr >> 24) << 56) & (0xff << 56);
        high_entry.0 |= (ptr >> 32) & (0xffffffff);

        // Segment limit
        let size = (core::mem::size_of::<TaskStateSegment>() - 1) as u64;
        low_entry.0 |= size & 0xffff;
        low_entry.0 |= ((size >> 16) << 48) & (0xf << 48);

        // Access byte
        let mut access: u8 = 0;
        access |= 0x9; // 386-TSS
        access.set_bit(7, 1); // Present bit

        low_entry.0 |= (access as u64) << 40;

        [low_entry, high_entry]
    }
}
