use core::ops::Range;

use crate::{boot::multiboot_memory_map::MemoryMapTag, ylib::sync::mutex::Mutex};

use self::frame_allocator::{Frame, FrameAllocator, SimpleFrameAllocator};

use super::physical_address::PhysicalAddress;

pub mod frame_allocator;

pub static FRAME_ALLOCATOR: Mutex<Option<SimpleFrameAllocator>> = Mutex::new(None);

pub fn init_allocator(memory_map: &'static MemoryMapTag, kernel_area: Range<PhysicalAddress>, multiboot_area: Range<PhysicalAddress>) {
    let mut allocator = FRAME_ALLOCATOR.lock();

    if allocator.is_some() {
        panic!("Allocator already initialized.");
    }

    let initialized_allocator = SimpleFrameAllocator::init(memory_map, kernel_area, multiboot_area);
    allocator.get_or_insert(initialized_allocator);
}

pub fn allocate_frame() -> Option<Frame> {
    let mut allocator = FRAME_ALLOCATOR.lock();
    assert!(allocator.is_some());
    allocator.as_mut().unwrap().allocate_frame()
}

pub fn deallocate_frame(frame: Frame) {
    let mut allocator = FRAME_ALLOCATOR.lock();
    assert!(allocator.is_some());
    allocator.as_mut().unwrap().deallocate_frame(frame);
} 