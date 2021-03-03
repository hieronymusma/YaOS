#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(abi_x86_interrupt)]

#[macro_use]
pub mod vga_buffer;

mod asm;

#[path = "../ylib/mod.rs"]
mod ylib;

mod interrupts;
mod memory;
mod serial;
mod pic;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen!();
    println!("Starting YaOS Kernel");
    serial_println!("Starting YaOS Kernel");

    init();

    ok!("Booting finished");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init() {
    memory::global_descriptor_table::init();
    interrupts::init_idt();
    unsafe { 
        pic::PICS.lock().init();
        asm::interrupts::enable_interrupts(); 
    }
}
