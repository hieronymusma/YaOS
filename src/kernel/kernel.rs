#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(abi_x86_interrupt)]

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

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    clear_screen!();
    println!("Starting YaOS Kernel");
    serial_println!("Starting YaOS Kernel");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    serial_println!("kernel sections:");
    let mut count = 0;
    for section in elf_sections_tag.sections() {
        count += 1;
        serial_println!(
            "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.flags()
        );
    }
    serial_println!("Count {}", count);

    let kernel_start = elf_sections_tag
        .sections()
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.start_address() + s.size())
        .max()
        .unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size() as usize);

    serial_println!(
        "kernel_start: {:#x?}, kernel_end: {:#x?}",
        kernel_start,
        kernel_end
    );
    serial_println!(
        "multiboot_start: {:#x?}, multiboot_end: {:#x?}",
        multiboot_start,
        multiboot_end
    );

    let multiboot_header =
        unsafe { boot::multiboot_header::MultibootHeader::load(multiboot_information_address) };
    println!("{:#x?}", multiboot_header);
    let map = multiboot_header
        .get_memory_map()
        .expect("Memory map must be provided by bootloader.");

    println!("memory areas:");
    for entry in map.get_available_memory_areas() {
        println!(
            "    start: 0x{:x}, length: 0x{:x}",
            entry.start(),
            entry.size()
        )
    }

    init();

    ok!("Booting finished");

    asm::halt::halt_loop();
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
