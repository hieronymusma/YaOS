use super::super::VirtualAddress;
use core::fmt;

#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved_1: u32,
    privileged_stack_table: [VirtualAddress; 3],
    reserved_2: u64,
    pub interrupt_stack_table: [VirtualAddress; 7],
    reserved_3: u64,
    reserved_4: u16,
    io_map_base_address: u16,
}

impl TaskStateSegment {
    pub fn new() -> Self {
        TaskStateSegment {
            reserved_1: 0,
            privileged_stack_table: [VirtualAddress::zero(); 3],
            reserved_2: 0,
            interrupt_stack_table: [VirtualAddress::zero(); 7],
            reserved_3: 0,
            reserved_4: 0,
            io_map_base_address: 0,
        }
    }
}

impl fmt::Debug for TaskStateSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("TaskStateSegment");

        unsafe {
            s.field("reserved_1", &self.reserved_1);
            s.field("privileged_stack_table", &self.privileged_stack_table);
            s.field("reserved_2", &self.reserved_2);
            s.field("interrupt_stack_table", &self.interrupt_stack_table);
            s.field("reserved_3", &self.reserved_3);
            s.field("reserved_4", &self.reserved_4);
            s.field("io_map_base_address", &self.io_map_base_address);
        }

        s.finish()
    }
}
