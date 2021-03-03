use core::{marker::PhantomData, usize};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MultibootHeader {
    total_size: u32,
    reserved: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Tag {
    typ: TagTypes,
    size: u32,
}

impl MultibootHeader {
    pub unsafe fn load(address: usize) -> &'static Self {
        assert!(address & 0x7 == 0);
        let header = &*(address as *const MultibootHeader);
        assert!(header.reserved == 0);
        assert!(header.has_valid_end_tag() == true);
        header
    }

    pub unsafe fn iterate_tags(&self) {
        let tag_iterator = TagIterator::new(&self);
        for tag in tag_iterator {
            println!("Tag {:#x?}, Size {:#x?}", tag.typ as u32, tag.size);
        }
    }

    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag {
            typ: TagTypes::EndTag,
            size: 8,
        };

        let ptr = self as *const MultibootHeader;
        let end_tag_address = ptr as usize + (self.total_size - END_TAG.size) as usize;
        let end_tag = unsafe { &*(end_tag_address as *const Tag) };

        unsafe { end_tag.typ == END_TAG.typ && end_tag.size == END_TAG.size }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
enum TagTypes {
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

struct TagIterator<'a> {
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
