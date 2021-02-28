use super::entry::*;
use super::interrupt_types::*;
use crate::memory::virt_addr::*;

use core::ops::{Index, IndexMut};

use crate::memory::DescriptorTablePointer;

pub struct InterruptDescriptionTable([IDTEntry; 16]);

impl InterruptDescriptionTable {
    pub fn new() -> InterruptDescriptionTable {
        InterruptDescriptionTable([IDTEntry::missing(); 16])
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

impl Index<IDTType> for InterruptDescriptionTable {
    type Output = IDTEntry;
    fn index(&self, index: IDTType) -> &IDTEntry {
        &self.0[index as usize]
    }
}

impl IndexMut<IDTType> for InterruptDescriptionTable {
    fn index_mut(&mut self, index: IDTType) -> &mut IDTEntry {
        &mut self.0[index as usize]
    }
}