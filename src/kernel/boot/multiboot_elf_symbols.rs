use core::{ops::Deref, panic, usize};

use crate::memory::physical_address::PhysicalAddress;

use super::multiboot_tags::TagTypes;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ElfSymbolsTag {
    typ: TagTypes,
    size: u32,
    number_of_entries: u32,
    entry_size: u32,
    shndx: u32,
}

impl ElfSymbolsTag {
    pub fn iter(&self) -> impl Iterator<Item = ElfSectionHeaderWrapper> {
        ElfSectionHeaderIterator::new(self).filter(|x| x.get_type() != 0x0)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ElfSectionHeader32 {
    name: u32,
    typ: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    addr_align: u32,
    entry_size: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ElfSectionHeader64 {
    name: u32,
    typ: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    addr_align: u64,
    entry_size: u64,
}

pub struct ElfSectionHeaderWrapper {
    size: u8,
    ptr: *const u8,
}

impl Deref for ElfSectionHeaderWrapper {
    type Target = dyn ElfSectionHeader;
    fn deref(&self) -> &Self::Target {
        match self.size {
            32 => unsafe { &*(self.ptr as *const ElfSectionHeader32) },
            64 => unsafe { &*(self.ptr as *const ElfSectionHeader64) },
            _ => panic!("Invalid Elfsection size"),
        }
    }
}

pub trait ElfSectionHeader {
    fn get_flags(&self) -> u64;
    fn get_addr(&self) -> PhysicalAddress;
    fn get_size(&self) -> usize;
    fn get_type(&self) -> u64;
}

impl ElfSectionHeader for ElfSectionHeader32 {
    fn get_flags(&self) -> u64 {
        self.flags.into()
    }

    fn get_addr(&self) -> PhysicalAddress {
        PhysicalAddress::from_32bit(self.addr)
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_type(&self) -> u64 {
        self.typ.into()
    }
}

impl ElfSectionHeader for ElfSectionHeader64 {
    fn get_addr(&self) -> PhysicalAddress {
        PhysicalAddress::from_64bit(self.addr)
    }

    fn get_flags(&self) -> u64 {
        self.flags
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_type(&self) -> u64 {
        self.typ.into()
    }
}

pub struct ElfSectionHeaderIterator {
    current_section: *const u8,
    entry_size: u32,
    remaining_sections: u32,
}

impl ElfSectionHeaderIterator {
    fn new(elf_symbols_tag: &ElfSymbolsTag) -> Self {
        let tag_pointer = elf_symbols_tag as *const _ as usize;
        let size_of_struct = core::mem::size_of::<ElfSymbolsTag>();
        let first_section = tag_pointer + size_of_struct;
        ElfSectionHeaderIterator {
            current_section: first_section as *const u8,
            entry_size: elf_symbols_tag.entry_size,
            remaining_sections: elf_symbols_tag.number_of_entries,
        }
    }
}

impl Iterator for ElfSectionHeaderIterator {
    type Item = ElfSectionHeaderWrapper;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_sections == 0 {
            return None;
        }
        let elf_section_size = match self.entry_size {
            40 => 32,
            64 => 64,
            _ => panic!(),
        };

        let current = ElfSectionHeaderWrapper {
            size: elf_section_size,
            ptr: self.current_section,
        };

        self.remaining_sections -= 1;
        unsafe {
            self.current_section = self.current_section.add(self.entry_size as usize);
        }

        Some(current)
    }
}
