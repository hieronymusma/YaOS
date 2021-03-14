use crate::boot::multiboot_header::MultibootHeader;

use self::page_table::{Level4, PageTable};
use crate::memory::physical_address::PhysicalAddress;
use crate::memory::allocator::frame_allocator::Frame;

pub mod page_fault_error_code;
mod page_table;
mod page_table_entry;

// Mapped in boot.asm
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xC0000000;

pub fn remap_kernel(multiboot_header: &MultibootHeader) {
    // Create new page table and correctly map kernel sections
    // Identity map phyiscal memory at PHYSICAL_MEMORY_OFFSET
    // Identity map vga buffer

    let p4 = PageTable::<Level4>::allocate();
    
    let memory_map = multiboot_header.get_memory_map().expect("Memory map must be provided");
    let sections = multiboot_header.get_elf_sections().expect("Elf sections of kernel must be provided");

    for memory_area in memory_map.get_available_memory_areas() {
        serial_println!("Memory area: {:#x?}", memory_area);
        for page in memory_area.pages() {
            // serial_println!("Page: {:#?}", page);
            let virtual_address = page.get_virtual_address();
            let entry = p4.map(&page, &virtual_address);
    
            entry.set_present(true);

            let section_for_range = sections.allocated().find(|x| {
                x.get_range().contains(&virtual_address)
            });
    
            if let Some(section) = section_for_range {
                let flags = section.get_flags();
                serial_println!("Inside kernel section ({:#?}): Writable: {}, Executable: {}", virtual_address, flags.is_writable(), flags.is_executable());
                entry.set_writable(flags.is_writable());
                entry.set_executable(flags.is_executable());
            }
        }
        
    }

    // Map video buffer
    let video_address = PhysicalAddress::new(0xb8000);
    p4.identity_map(&video_address);

    // Map multiboot struct
    let multiboot_address = PhysicalAddress::new(multiboot_header as *const _ as usize);
    let multiboot_address = Frame::containing_page(multiboot_address).start();
    p4.identity_map(&multiboot_address);

    serial_println!("L4 Page Table\n{:#?}", p4);
    
    let address_p4 = p4.get_address();
    crate::asm::memory::write_cr3(address_p4);
}