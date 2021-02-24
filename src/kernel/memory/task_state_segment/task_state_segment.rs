use super::super::VirtAddr;

#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved_1: u32,
    privileged_stack_table: [VirtAddr; 3],
    reserved_2: u64,
    pub interrupt_stack_table: [VirtAddr; 7],
    reserved_3: u64,
    reserved_4: u16,
    io_map_base_address: u16,
}

impl TaskStateSegment {
    pub fn new() -> Self {
        TaskStateSegment {
            reserved_1: 0,
            privileged_stack_table: [VirtAddr::zero(); 3],
            reserved_2: 0,
            interrupt_stack_table: [VirtAddr::zero(); 7],
            reserved_3: 0,
            reserved_4: 0,
            io_map_base_address: 0,
        }
    }
}
