use super::multiboot_tags::*;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryMapTag {
    typ: TagTypes,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_entry: MemoryMapEntry,
}

impl<'a> MemoryMapTag {
    pub fn get_available_memory_areas(&self) -> impl Iterator<Item = &'static MemoryMapEntry> {
        self.get_memory_areas().filter(|x| x.typ == 1)
    }

    pub fn get_memory_areas(&self) -> MemoryMapEntryIterator<'a> {
        MemoryMapEntryIterator::new(self)
    }
}

pub struct MemoryMapEntryIterator<'a> {
    current: *const MemoryMapEntry,
    end: usize,
    entry_size: u32,
    lifetime: PhantomData<&'a MemoryMapTag>,
}

impl<'a> MemoryMapEntryIterator<'a> {
    pub fn new(map: &MemoryMapTag) -> Self {
        MemoryMapEntryIterator {
            current: &map.first_entry as *const MemoryMapEntry,
            end: (map as *const _ as usize) + map.size as usize,
            entry_size: map.entry_size,
            lifetime: PhantomData,
        }
    }
}

impl<'a> Iterator for MemoryMapEntryIterator<'a> {
    type Item = &'static MemoryMapEntry;
    fn next(&mut self) -> Option<Self::Item> {
        // Check if end is reached
        if self.current as usize >= self.end {
            return None;
        }
        let current_element = unsafe { &*self.current };
        let current_ptr = self.current as usize;
        let new_ptr = (current_ptr + self.entry_size as usize) as *const MemoryMapEntry;
        self.current = unsafe { &*new_ptr };
        Some(current_element)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryMapEntry {
    address: u64,
    length: u64,
    typ: u32,
    zero: u32,
}

impl MemoryMapEntry {
    pub fn size(&self) -> u64 {
        self.length
    }

    pub fn start(&self) -> u64 {
        self.address
    }
}
