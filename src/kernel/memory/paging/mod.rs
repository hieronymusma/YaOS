pub mod page_fault_error_code;
mod page_table;
mod page_table_entry;

// Mapped in boot.asm
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xC0000000;
