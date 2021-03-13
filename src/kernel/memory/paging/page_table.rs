use core::ops::{Index, IndexMut};

use super::page_table_entry::PageTableEntry;

#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::invalid(); 512]
        }
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}