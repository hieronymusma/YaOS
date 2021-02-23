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

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen!();
    println!("Starting YaOS Kernel");

    init();

    ok!("Booting finished");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_impl(info);
}

fn panic_impl(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init() {
    interrupts::init();
}
