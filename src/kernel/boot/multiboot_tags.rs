use super::multiboot_header::*;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Tag {
    typ: TagTypes,
    size: u32,
}

impl Tag {
    pub const fn new(typ: TagTypes, size: u32) -> Self {
        Tag { typ, size }
    }

    pub fn typ(&self) -> TagTypes {
        self.typ
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum TagTypes {
    EndTag = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Modules = 3,
    BasicMemoryInformation = 4,
    BiosBootDevice = 5,
    MemoryMap = 6,
    VBEInfo = 7,
    FramebufferInfo = 8,
    ElfSymbols = 9,
    APMTable = 10,
}

pub struct TagIterator<'a> {
    current: *const Tag,
    lifetime: PhantomData<&'a Tag>,
}

impl<'a> TagIterator<'a> {
    pub fn new(multiboot_header: &MultibootHeader) -> Self {
        let multiboot_ptr = multiboot_header as *const _ as usize;
        let first_tag_ptr = (multiboot_ptr + 8) as *const Tag;
        TagIterator {
            current: first_tag_ptr,
            lifetime: PhantomData,
        }
    }
}

impl<'a> Iterator for TagIterator<'a> {
    type Item = &'a Tag;
    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe { &*self.current };
        let current_ptr = current as *const _ as usize;
        match current.typ {
            TagTypes::EndTag => None,
            _ => {
                let offset = ((current.size + 7) & !7) as usize; // use correct alignment
                let new_ptr = (current_ptr + offset) as *const Tag;
                self.current = unsafe { &*new_ptr };
                Some(current)
            }
        }
    }
}
