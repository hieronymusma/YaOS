use crate::memory::physical_address::PhysicalAddress;

use super::multiboot_tags::*;
use core::{marker::PhantomData, usize};

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
        self.get_memory_areas()
            .filter(|x| x.typ() == MemoryAreaType::Available)
    }

    pub fn get_memory_areas(&self) -> MemoryMapEntryIterator<'a> {
        MemoryMapEntryIterator::new(self)
    }
}

#[derive(Debug)]
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
    pub fn size(&self) -> usize {
        self.length as usize
    }

    pub fn start(&self) -> PhysicalAddress {
        PhysicalAddress::from_64bit(self.address)
    }

    pub fn end(&self) -> PhysicalAddress {
        self.start() + self.size()
    }

    pub fn pages(&self) -> PageIterator {
        PageIterator {
            current: self.start(),
            end: self.end().round_up(),
        }
    }

    pub fn typ(&self) -> MemoryAreaType {
        match self.typ {
            1 => MemoryAreaType::Available,
            3 => MemoryAreaType::AcpiAvailable,
            4 => MemoryAreaType::ReservedHibernate,
            5 => MemoryAreaType::Defective,
            _ => MemoryAreaType::Reserved,
        }
    }
}

pub struct PageIterator {
    current: PhysicalAddress,
    end: PhysicalAddress,
}

impl Iterator for PageIterator {
    type Item = PhysicalAddress;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let current = Some(self.current);
            self.current += 4096;
            current
        } else {
            None
        }
    }
}

/// An enum of possible reported region types.
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryAreaType {
    /// A reserved area that must not be used.
    Reserved,

    /// Available memory free to be used by the OS.
    Available,

    /// Usable memory holding ACPI information.
    AcpiAvailable,

    /// Reserved memory which needs to be preserved on hibernation.
    ReservedHibernate,

    /// Memory which is occupied by defective RAM modules.
    Defective,
}
