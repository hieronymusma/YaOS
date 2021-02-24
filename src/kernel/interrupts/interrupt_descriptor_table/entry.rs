use core::option;

use super::entry_options::*;
use crate::memory::privilege_level::*;
use crate::memory::segment_selector::*;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: IDTEntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl IDTEntry {
    pub fn new(gdt_selector: SegmentSelector, handler: u64) -> Self {
        IDTEntry {
            gdt_selector: gdt_selector,
            pointer_low: handler as u16,
            pointer_middle: (handler >> 16) as u16,
            pointer_high: (handler >> 32) as u32,
            options: IDTEntryOptions::new(),
            reserved: 0,
        }
    }

    pub fn missing() -> Self {
        IDTEntry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: IDTEntryOptions::minimal(),
            reserved: 0,
        }
    }

    #[allow(dead_code)]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        // Retrieve local variable first to access unaligned struct
        let mut options = self.options;
        options.set_present(present);
        self
    }

    #[allow(dead_code)]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        // Retrieve local variable first to access unaligned struct
        let mut options = self.options;
        options.disable_interrupts(disable);
        self
    }

    pub unsafe fn set_stack_index(&mut self, index: u8) -> &mut Self {
        let mut options = self.options;
        options.set_stack_index(index + 1);
        self
    }
}
