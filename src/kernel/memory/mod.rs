pub mod global_descriptor_table;
pub mod privilege_level;
pub mod segment_selector;
pub mod task_state_segment;
pub mod virt_addr;

use virt_addr::VirtAddr;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: VirtAddr,
}
