use core::{fmt, ops::Range, usize};

use crate::{boot::multiboot_memory_map::{MemoryAreaType, MemoryMapEntry, MemoryMapEntryIterator, MemoryMapTag}};

const FRAME_SIZE: usize = 4096;

pub struct SimpleFrameAllocator<'a> {
    frame_iter: FrameIteratorForMemoryArea,
    memory_map_iter: MemoryMapEntryIterator<'a>,
    kernel_area: Range<u64>,
    multiboot_area: Range<u64>,
}

impl<'a> SimpleFrameAllocator<'a> {
    pub fn init(
        memory_map: &'static MemoryMapTag,
        kernel_area: Range<u64>,
        multiboot_area: Range<u64>,
    ) -> Self {
        let mut memory_areas = memory_map.get_memory_areas();
        let first_memory_area = SimpleFrameAllocator::get_next_available_memory_area(&mut memory_areas);
        let memory_area_iterator = FrameIteratorForMemoryArea::new(first_memory_area, kernel_area.clone(), multiboot_area.clone());

        SimpleFrameAllocator {
            kernel_area,
            multiboot_area,
            memory_map_iter: memory_areas,
            frame_iter: memory_area_iterator,
        }
    }

    fn get_next_available_memory_area(memory_map_iter: &mut MemoryMapEntryIterator) -> Option<&'static MemoryMapEntry> {
        let mut current = memory_map_iter.next();
        while current.is_some() && current.unwrap().typ() != MemoryAreaType::Available {
            current = memory_map_iter.next();
        }
        current
    }
}

impl<'a> FrameAllocator for SimpleFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<Frame> {
        let frame = self.frame_iter.next();
        if frame.is_some() {
            return frame;
        }
        let next_memory_area = SimpleFrameAllocator::get_next_available_memory_area(&mut self.memory_map_iter);
        if next_memory_area.is_some() {
            self.frame_iter = FrameIteratorForMemoryArea::new(next_memory_area, self.kernel_area.clone(), self.multiboot_area.clone());
            return self.allocate_frame();
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!();
    }
}

pub struct Frame {
    ptr: *const u8,
    size: usize,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame({:#x?}, {:#x?})", self.ptr, self.size)
    }
}

impl Frame {
    fn new(ptr: *const u8) -> Self {
        Frame {
            ptr,
            size: FRAME_SIZE,
        }
    }

    fn start(&self) -> u64 {
        self.ptr as u64
    }

    fn end(&self) -> u64 {
        self.start() + self.size as u64
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct FrameIteratorForMemoryArea {
    memory_map_entry: Option<&'static MemoryMapEntry>,
    current_position: u64,
    kernel_area: Range<u64>,
    multiboot_area: Range<u64>,
}

impl Iterator for FrameIteratorForMemoryArea {
    type Item = Frame;
    fn next(&mut self) -> Option<Self::Item> {
        if self.memory_map_entry.is_none() {
            return None;
        }
        if self.current_position >= self.memory_map_entry.unwrap().end() {
            return None;
        }
        let frame = Frame::new(self.current_position as *const u8);
        self.current_position += FRAME_SIZE as u64;
        if self.is_frame_free(&frame) {
            return Some(frame);
        }
        self.next()
    }
}

impl FrameIteratorForMemoryArea {
    pub fn new(memory_map_entry: Option<&'static MemoryMapEntry>, kernel_area: Range<u64>, multiboot_area: Range<u64>) -> Self {
        let mut start_position = 0;
        if memory_map_entry.is_some() {
            start_position = memory_map_entry.unwrap().start();
        }
        FrameIteratorForMemoryArea {
            memory_map_entry,
            current_position: start_position,
            kernel_area,
            multiboot_area,
        }
    }

    fn is_frame_free(&self, frame: &Frame) -> bool {
        // Check start address
        let start_addr = frame.start();
        let end_addr = frame.end();

        !self.kernel_area.contains(&start_addr)
            && !self.kernel_area.contains(&end_addr)
            && !self.multiboot_area.contains(&start_addr)
            && !self.multiboot_area.contains(&end_addr)
    }
}