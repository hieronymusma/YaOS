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

use core::panic::PanicInfo;

use boot::multiboot_memory_map::MemoryMapTag;
use memory::allocator::init_allocator;

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    clear_screen!();
    println!("Starting YaOS Kernel");
    serial_println!("Starting YaOS Kernel");

    print_memory_map(multiboot_information_address);

    init(multiboot_information_address);

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

   
    let kernel_location = multiboot_header.get_kernel_location();
    let multiboot_location = multiboot_header.get_multiboot_location();

    serial_println!(
        "kernel_start: {:#x?}, kernel_end: {:#x?}",
        kernel_location.start,
        kernel_location.end
    );
    serial_println!(
        "multiboot_start: {:#x?}, multiboot_end: {:#x?}",
        multiboot_location.start,
        multiboot_location.end
    );
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    asm::halt::halt_loop();
}

fn init(multiboot_information_address: usize) {
    memory::global_descriptor_table::init();
    interrupts::init_idt();
    unsafe {
        pic::PICS.lock().init();
        asm::interrupts::enable_interrupts();
    }

    let multiboot_header =
        unsafe { boot::multiboot_header::MultibootHeader::load(multiboot_information_address) };

    let memory_map: &'static MemoryMapTag = multiboot_header
        .get_memory_map()
        .expect("Memory map must be provided by bootloader.");

    let kernel_area = multiboot_header.get_kernel_location();
    let multiboot_area = multiboot_header.get_multiboot_location();

    init_allocator(memory_map, kernel_area, multiboot_area)
}
