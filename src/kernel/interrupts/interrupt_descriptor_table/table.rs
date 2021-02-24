use super::entry::*;
use super::interrupt_types::*;
use crate::memory::segment_selector::*;
use crate::memory::virt_addr::*;

use crate::memory::DescriptorTablePointer;

pub struct InterruptDescriptionTable([IDTEntry; 16]);

impl InterruptDescriptionTable {
    pub fn new() -> InterruptDescriptionTable {
        InterruptDescriptionTable([IDTEntry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: IDTType, handler: u64) -> &mut IDTEntry {
        self.0[entry as usize] = IDTEntry::new(SegmentSelector::get_cs(), handler);
        &mut self.0[entry as usize]
    }

    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe {
            InterruptDescriptionTable::load_idt(&ptr);
        }
    }

    unsafe fn load_idt(gdt: &DescriptorTablePointer) {
        asm!("lidt [{}]", in(reg) gdt, options(nostack));
    }
}
