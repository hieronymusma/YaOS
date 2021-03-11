pub mod page_fault_error_code;
mod page_table;

// Mapped in boot.asm
const PHYSICAL_MAPPING_START_ADDRESS: usize = 0xffffff8000000000;