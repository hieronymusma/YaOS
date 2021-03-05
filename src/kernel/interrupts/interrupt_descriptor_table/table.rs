use core::{
    ops::{Index, IndexMut},
    usize,
};

use super::entry::*;
use crate::memory::virtual_address::*;

use crate::memory::DescriptorTablePointer;

#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero_error: IDTEntry<HandlerFunc>,
    pub debug: IDTEntry<HandlerFunc>,
    pub non_maskable_interrupt: IDTEntry<HandlerFunc>,
    pub breakpoint: IDTEntry<HandlerFunc>,
    pub overflow: IDTEntry<HandlerFunc>,
    pub bound_range_exceeded: IDTEntry<HandlerFunc>,
    pub invalid_opcode: IDTEntry<HandlerFunc>,
    pub device_not_available: IDTEntry<HandlerFunc>,
    pub double_fault: IDTEntry<NonReturnHandlerFuncWithErrorCode>,
    pub coprocessor_segment_overrun: IDTEntry<HandlerFunc>,
    pub invalid_tss: IDTEntry<HandlerFuncWithErrorCode>,
    pub segment_not_present: IDTEntry<HandlerFuncWithErrorCode>,
    pub stack_segment_fault: IDTEntry<HandlerFuncWithErrorCode>,
    pub general_protection_fault: IDTEntry<HandlerFuncWithErrorCode>,
    pub page_fault: IDTEntry<PageFaultHandlerFunc>,
    reserved_1: IDTEntry<HandlerFunc>,
    pub x87_floating_point: IDTEntry<HandlerFunc>,
    pub alignment_check: IDTEntry<HandlerFuncWithErrorCode>,
    pub machine_check: IDTEntry<NonReturnHandlerFunc>,
    pub simd_floating_point: IDTEntry<HandlerFunc>,
    pub virtualization: IDTEntry<HandlerFunc>,
    reserved_2: [IDTEntry<HandlerFunc>; 9],
    pub security_exception: IDTEntry<HandlerFuncWithErrorCode>,
    reserved_3: IDTEntry<HandlerFunc>,
    // PIC interrupts are mapped to this
    pub interrupts: [IDTEntry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_by_zero_error: IDTEntry::missing(),
            debug: IDTEntry::missing(),
            non_maskable_interrupt: IDTEntry::missing(),
            breakpoint: IDTEntry::missing(),
            overflow: IDTEntry::missing(),
            bound_range_exceeded: IDTEntry::missing(),
            invalid_opcode: IDTEntry::missing(),
            device_not_available: IDTEntry::missing(),
            double_fault: IDTEntry::missing(),
            coprocessor_segment_overrun: IDTEntry::missing(),
            invalid_tss: IDTEntry::missing(),
            segment_not_present: IDTEntry::missing(),
            stack_segment_fault: IDTEntry::missing(),
            general_protection_fault: IDTEntry::missing(),
            page_fault: IDTEntry::missing(),
            reserved_1: IDTEntry::missing(),
            x87_floating_point: IDTEntry::missing(),
            alignment_check: IDTEntry::missing(),
            machine_check: IDTEntry::missing(),
            simd_floating_point: IDTEntry::missing(),
            virtualization: IDTEntry::missing(),
            reserved_2: [IDTEntry::missing(); 9],
            security_exception: IDTEntry::missing(),
            reserved_3: IDTEntry::missing(),
            interrupts: [IDTEntry::missing(); 256 - 32],
        }
    }

    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtualAddress::from_ref(self),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe {
            InterruptDescriptorTable::load_idt(&ptr);
        }
    }

    unsafe fn load_idt(gdt: &DescriptorTablePointer) {
        asm!("lidt [{}]", in(reg) gdt, options(nostack));
    }
}

impl Index<InterruptType> for InterruptDescriptorTable {
    type Output = IDTEntry<HandlerFunc>;
    fn index(&self, index: InterruptType) -> &Self::Output {
        &self.interrupts[index as usize]
    }
}

impl IndexMut<InterruptType> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: InterruptType) -> &mut Self::Output {
        &mut self.interrupts[index as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum InterruptType {
    // PIC Timer interrupt is the first entry in the idt.interrupts field
    Timer = 0,
    Keyboard = 1,
}

impl InterruptType {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}
