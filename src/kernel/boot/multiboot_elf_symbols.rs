use core::{ops::Deref, panic, usize};

use crate::{memory::virtual_address::VirtualAddress, ylib::utilities::bit_manipulator::{BitManipulation}};

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
    pub fn used(&self) -> impl Iterator<Item = ElfSectionHeaderWrapper> {
        ElfSectionHeaderIterator::new(self).filter(|section| {
            section.get_type() != ElfSectionType::Unused && !section.get_flags().is_none()
        })
    }

    pub fn allocated(&self) -> impl Iterator<Item = ElfSectionHeaderWrapper> {
        ElfSectionHeaderIterator::new(self).filter(|section| {
            section.get_type() != ElfSectionType::Unused && section.get_flags().is_allocated()
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ElfSectionHeader32 {
    name: u32,
    typ: ElfSectionType,
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
    typ: ElfSectionType,
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
    fn get_flags(&self) -> ElfSectionFlags;
    fn get_addr(&self) -> VirtualAddress;
    fn get_size(&self) -> usize;
    fn get_type(&self) -> ElfSectionType;
}

impl ElfSectionHeader for ElfSectionHeader32 {
    fn get_flags(&self) -> ElfSectionFlags {
        ElfSectionFlags(self.flags as u64)
    }

    fn get_addr(&self) -> VirtualAddress {
        VirtualAddress::new(self.addr as usize)
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_type(&self) -> ElfSectionType {
        self.typ
    }
}

impl ElfSectionHeader for ElfSectionHeader64 {
    fn get_addr(&self) -> VirtualAddress {
        VirtualAddress::new(self.addr as usize)
    }

    fn get_flags(&self) -> ElfSectionFlags {
        ElfSectionFlags(self.flags)
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_type(&self) -> ElfSectionType {
        self.typ
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

/// An enum abstraction over raw ELF section types.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum ElfSectionType {
    /// This value marks the section header as inactive; it does not have an
    /// associated section. Other members of the section header have undefined
    /// values.
    Unused = 0,

    /// The section holds information defined by the program, whose format and
    /// meaning are determined solely by the program.
    ProgramSection = 1,

    /// This section holds a linker symbol table.
    LinkerSymbolTable = 2,

    /// The section holds a string table.
    StringTable = 3,

    /// The section holds relocation entries with explicit addends, such as type
    /// Elf32_Rela for the 32-bit class of object files. An object file may have
    /// multiple relocation sections.
    RelaRelocation = 4,

    /// The section holds a symbol hash table.
    SymbolHashTable = 5,

    /// The section holds dynamic linking tables.
    DynamicLinkingTable = 6,

    /// This section holds information that marks the file in some way.
    Note = 7,

    /// A section of this type occupies no space in the file but otherwise resembles
    /// `ProgramSection`. Although this section contains no bytes, the
    /// sh_offset member contains the conceptual file offset.
    Uninitialized = 8,

    /// The section holds relocation entries without explicit addends, such as type
    /// Elf32_Rel for the 32-bit class of object files. An object file may have
    /// multiple relocation sections.
    RelRelocation = 9,

    /// This section type is reserved but has unspecified semantics.
    Reserved = 10,

    /// This section holds a dynamic loader symbol table.
    DynamicLoaderSymbolTable = 11,

    /// Values in this inclusive range (`[0x6000_0000, 0x6FFF_FFFF)`) are
    /// reserved for environment-specific semantics.
    EnvironmentSpecific = 0x6000_0000,

    /// Values in this inclusive range (`[0x7000_0000, 0x7FFF_FFFF)`) are
    /// reserved for processor-specific semantics.
    ProcessorSpecific = 0x7000_0000,
}

pub struct ElfSectionFlags(u64);

impl ElfSectionFlags {
    pub fn is_writable(&self) -> bool {
        self.0.is_bit_set(0x1)
    }

    pub fn is_allocated(&self) -> bool {
        self.0.is_bit_set(0x2)
    }

    pub fn is_executable(&self) -> bool {
        self.0.is_bit_set(0x4)
    }

    pub fn is_none(&self) -> bool {
        !self.is_allocated() && !self.is_executable() && !self.is_writable()
    }
}

impl core::fmt::Debug for ElfSectionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_none() {
            return f.write_str("NONE");
        }

        let mut _is_first = true;

        if self.is_writable() {
            if !_is_first {
                f.write_str(" | ")?
            }
            _is_first = false;
            f.write_str("WRITABLE")?
        }

        if self.is_executable() {
            if !_is_first {
                f.write_str(" | ")?
            }
            _is_first = false;
            f.write_str("EXECUTABLE")?
        }

        if self.is_allocated() {
            if !_is_first {
                f.write_str(" | ")?
            }
            _is_first = false;
            f.write_str("ALLOCATED")?
        }

        Ok(())
    }
}
