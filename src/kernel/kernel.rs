#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

#[macro_use]
pub mod vga_buffer;

#[macro_use]
mod serial;

mod asm;

#[path = "../ylib/mod.rs"]
mod ylib;

mod boot;
mod interrupts;
mod memory;
mod pic;

use core::{ops::Range, panic::PanicInfo};

use boot::multiboot_memory_map::MemoryMapTag;
use memory::{
    allocator::frame_allocator::FrameAllocator, physical_address::PhysicalAddress,
    virtual_address::VirtualAddress,
};

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    clear_screen!();
    println!("Starting YaOS Kernel");
    serial_println!("Starting YaOS Kernel");

    print_memory_map(multiboot_information_address);

    init();

    ok!("Booting finished");

    asm::halt::halt_loop();
}

fn print_memory_map(multiboot_information_address: usize) {
    let multiboot_header =
        unsafe { boot::multiboot_header::MultibootHeader::load(multiboot_information_address) };

    let map: &'static MemoryMapTag = multiboot_header
        .get_memory_map()
        .expect("Memory map must be provided by bootloader.");

    println!("memory areas:");
    for entry in map.get_available_memory_areas() {
        println!(
            "    start: {:#x?}, length: 0x{:#x?}",
            entry.start(),
            entry.size()
        )
    }

    let elf_sections = multiboot_header
        .get_elf_sections()
        .expect("Elf Sections must be present.");

    for elf_section in elf_sections.allocated() {
        serial_println!(
            "    addr: {:#x?}, size: {:#x?}, flags: {:#x?}, typ: {:#x?}",
            elf_section.get_addr(),
            elf_section.get_size(),
            elf_section.get_flags(),
            elf_section.get_type()
        );
    }

    let kernel_start = elf_sections.used().map(|s| s.get_addr()).min().unwrap();

    let kernel_end: VirtualAddress = elf_sections
        .used()
        .map(|s| s.get_addr() + s.get_size())
        .max()
        .unwrap();

    let multiboot_start = VirtualAddress::new(multiboot_information_address).get_physical_address();
    let multiboot_end: PhysicalAddress = multiboot_start + multiboot_header.get_size();

    serial_println!(
        "kernel_start: {:#x?}, kernel_end: {:#x?}",
        kernel_start.get_physical_address(),
        kernel_end.get_physical_address()
    );
    serial_println!(
        "multiboot_start: {:#x?}, multiboot_end: {:#x?}",
        multiboot_start,
        multiboot_end
    );

    test_alloc(
        map,
        kernel_start.get_physical_address()..kernel_end.get_physical_address(),
        multiboot_start..multiboot_end,
    );
}

pub fn test_alloc(
    map: &'static MemoryMapTag,
    kernel_area: Range<PhysicalAddress>,
    multiboot_area: Range<PhysicalAddress>,
) {
    let mut allocator = memory::allocator::frame_allocator::SimpleFrameAllocator::init(
        map,
        kernel_area.clone(),
        multiboot_area.clone(),
    );
    for _i in 0..260 {
        let frame = allocator.allocate_frame();
        match frame {
            Some(x) => serial_println!("{:#?}", x),
            None => {
                serial_println!("Cannot allocate further frames.");
                break;
            }
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    asm::halt::halt_loop();
}

fn init() {
    memory::global_descriptor_table::init();
    interrupts::init_idt();
    unsafe {
        pic::PICS.lock().init();
        asm::interrupts::enable_interrupts();
    }
}
