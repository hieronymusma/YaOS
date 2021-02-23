use super::entry::Entry;

const MAX_ENTRIES: usize = 8;

pub struct GlobalDescriptorTable {
    table: [Entry; MAX_ENTRIES],
    next: u8,
}

impl GlobalDescriptorTable {
    pub fn new() -> Self {
        GlobalDescriptorTable {
            table: [Entry::empty(); MAX_ENTRIES],
            next: 0,
        }
    }
}
