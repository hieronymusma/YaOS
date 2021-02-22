use super::idt_entry::*;
use super::idt_type::*;
use crate::memory::segment_selector::*;
use crate::ylib::primitives::virt_addr::*;

pub struct IDT([IDTEntry; 16]);

impl IDT {
    pub fn new() -> IDT {
        IDT([IDTEntry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: IDTType, handler: u64) -> &mut IDTEntry {
        self.0[entry as usize] = IDTEntry::new(SegmentSelector::get_cs(), handler);
        &mut self.0[entry as usize]
    }

    // Should be 'static
    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe {
            IDT::load_idt(&ptr);
        }
    }

    unsafe fn load_idt(gdt: &DescriptorTablePointer) {
        asm!("lidt [{}]", in(reg) gdt, options(nostack));
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: VirtAddr,
}
