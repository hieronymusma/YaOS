use core::{ops::Range, usize};

use crate::memory::{physical_address::PhysicalAddress, virtual_address::VirtualAddress};

use super::multiboot_elf_symbols::*;
use super::multiboot_memory_map::*;
use super::multiboot_tags::*;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MultibootHeader {
    total_size: u32,
    reserved: u32,
}

impl MultibootHeader {
    pub unsafe fn load(address: usize) -> &'static Self {
        assert!(address & 0x7 == 0);
        let header = &*(address as *const MultibootHeader);
        assert!(header.reserved == 0);
        assert!(header.has_valid_end_tag() == true);
        header
    }

    pub fn get_memory_map(&self) -> Option<&'static MemoryMapTag> {
        self.get_section::<MemoryMapTag>(TagTypes::MemoryMap)
    }

    pub fn get_elf_sections(&self) -> Option<&'static ElfSymbolsTag> {
        self.get_section::<ElfSymbolsTag>(TagTypes::ElfSymbols)
    }

    pub fn get_kernel_location(&self) -> Range<PhysicalAddress> {
        let elf_sections = self
            .get_elf_sections()
            .expect("Elf Sections must be present.");
        let kernel_start = elf_sections.used().map(|s| s.get_addr()).min().unwrap();

        let kernel_end: VirtualAddress = elf_sections
            .used()
            .map(|s| s.get_addr() + s.get_size())
            .max()
            .unwrap();

        kernel_start.get_physical_address()..kernel_end.get_physical_address()
    }

    pub fn get_multiboot_location(&self) -> Range<PhysicalAddress> {
        let multiboot_start = VirtualAddress::from_ptr(self as *const _).get_physical_address();
        let multiboot_end: PhysicalAddress = multiboot_start + self.get_size();
        multiboot_start..multiboot_end
    }

    pub fn get_size(&self) -> usize {
        self.total_size as usize
    }

    fn get_section<T>(&self, typ: TagTypes) -> Option<&'static T> {
        let mut iter = TagIterator::new(&self);
        unsafe {
            iter.find(|element| element.typ() == typ)
                .map(|tag| &*(tag as *const Tag as *const T))
        }
    }

    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag::new(TagTypes::EndTag, 8);

        let ptr = self as *const MultibootHeader;
        let end_tag_address = ptr as usize + (self.total_size - END_TAG.size()) as usize;
        let end_tag = unsafe { &*(end_tag_address as *const Tag) };

        end_tag.typ() == END_TAG.typ() && end_tag.size() == END_TAG.size()
    }
}
