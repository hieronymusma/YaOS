use core::marker::PhantomData;

use super::{entry_options::*, interrupt_stack_frame::InterruptStackFrame};
use crate::memory::privilege_level::*;
use crate::memory::segment_selector::*;

pub type HandlerFunc = extern "x86-interrupt" fn(&InterruptStackFrame);
pub type HandlerFuncWithErrorCode =
    extern "x86-interrupt" fn(&InterruptStackFrame, error_code: u64);
pub type NonReturnHandlerFunc = extern "x86-interrupt" fn(&InterruptStackFrame) -> !;
pub type NonReturnHandlerFuncWithErrorCode =
    extern "x86-interrupt" fn(&InterruptStackFrame, error_code: u64) -> !;

#[repr(C, packed)]
pub struct IDTEntry<T> {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    pub options: IDTEntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    handler_type: PhantomData<T>,
}

impl<T> IDTEntry<T> {
    fn with_handler(gdt_selector: SegmentSelector, handler: u64) -> Self {
        IDTEntry {
            gdt_selector: gdt_selector,
            pointer_low: handler as u16,
            pointer_middle: (handler >> 16) as u16,
            pointer_high: (handler >> 32) as u32,
            options: IDTEntryOptions::present(),
            reserved: 0,
            handler_type: PhantomData,
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
            handler_type: PhantomData,
        }
    }

    fn set_handler_internal(&mut self, handler: u64) -> &mut IDTEntryOptions {
        *self = IDTEntry::with_handler(SegmentSelector::get_cs(), handler);
        return unsafe { &mut self.options };
    }
}

// Cannot derive Clone trait automatically because of type parameter
impl<T> Clone for IDTEntry<T> {
    fn clone(&self) -> Self {
        IDTEntry {
            ..*self
        }
    }
}

impl<T> Copy for IDTEntry<T> { }

macro_rules! set_handler_fn_impl {
    ($t:ty) => {
        impl IDTEntry<$t> {
            #[allow(dead_code)]
            pub fn set_handler(&mut self, handler: $t) -> &mut IDTEntryOptions {
                self.set_handler_internal(handler as u64)
            }
        }
    };
}

set_handler_fn_impl!(HandlerFunc);
set_handler_fn_impl!(HandlerFuncWithErrorCode);
set_handler_fn_impl!(NonReturnHandlerFunc);
set_handler_fn_impl!(NonReturnHandlerFuncWithErrorCode);
