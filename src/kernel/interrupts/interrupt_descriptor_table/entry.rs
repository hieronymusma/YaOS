use super::entry_options::*;
use crate::memory::privilege_level::*;
use crate::memory::segment_selector::*;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    pub options: IDTEntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl IDTEntry {
    fn with_handler(gdt_selector: SegmentSelector, handler: u64) -> Self {
        IDTEntry {
            gdt_selector: gdt_selector,
            pointer_low: handler as u16,
            pointer_middle: (handler >> 16) as u16,
            pointer_high: (handler >> 32) as u32,
            options: IDTEntryOptions::present(),
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

    pub fn set_handler(&mut self, handler: u64) -> &mut IDTEntryOptions {
        *self = IDTEntry::with_handler(SegmentSelector::get_cs(), handler);
        return unsafe { &mut self.options };
    }
}
