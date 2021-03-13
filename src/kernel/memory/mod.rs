pub mod allocator;
pub mod global_descriptor_table;
pub mod paging;
pub mod physical_address;
pub mod privilege_level;
pub mod segment_selector;
pub mod task_state_segment;
pub mod virtual_address;

use virtual_address::VirtualAddress;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: VirtualAddress,
}
