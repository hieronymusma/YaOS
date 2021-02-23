use crate::memory::virt_addr::*;
use core::fmt;

#[derive(Clone)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: VirtAddr,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: VirtAddr,
    pub stack_segment: u64,
}

impl fmt::Debug for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u64);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("InterruptStackFrame");

        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &Hex(self.code_segment));
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &Hex(self.stack_segment));
        s.finish()
    }
}
