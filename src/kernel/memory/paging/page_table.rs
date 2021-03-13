#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {}
