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

    pub fn get_memory_map(&self) -> Option<&MemoryMapTag> {
        let mut iter = TagIterator::new(&self);
        unsafe {
            iter.find(|element| element.typ() == TagTypes::MemoryMap)
                .map(|tag| &*(tag as *const Tag as *const MemoryMapTag))
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
