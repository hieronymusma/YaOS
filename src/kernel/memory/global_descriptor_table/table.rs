use crate::memory::{privilege_level::PrivilegeLevel, segment_selector::SegmentSelector};

use super::entry::Entry;

use super::super::DescriptorTablePointer;
use super::super::VirtAddr;

const MAX_ENTRIES: usize = 8;

pub struct GlobalDescriptorTable {
    table: [Entry; MAX_ENTRIES],
    next: usize,
}

impl GlobalDescriptorTable {
    pub fn new() -> Self {
        GlobalDescriptorTable {
            table: [Entry::empty(); MAX_ENTRIES],
            next: 0,
        }
    }

    pub fn add_entry(&mut self, entry: Entry) -> SegmentSelector {
        if self.next >= MAX_ENTRIES {
            panic!("MAX_ENTRIES in GlobalDescriptorTable reached.");
        }
        self.table[self.next] = entry;
        self.next += 1;
        SegmentSelector::new((self.next - 1) as u16, PrivilegeLevel::Ring0)
    }

    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self.table.as_ptr() as u64),
            limit: ((size_of::<Entry>() * self.next) - 1) as u16,
        };

        unsafe {
            GlobalDescriptorTable::load_idt(&ptr);
        }
    }

    unsafe fn load_idt(gdt: &DescriptorTablePointer) {
        asm!("lgdt [{}]", in(reg) gdt, options(nostack));
    }
}
