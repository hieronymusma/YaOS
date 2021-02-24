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

    // unsafe {
    //     *(0xdeadbeef as *mut i32) = 42;
    // }

    fn stack_overflow() {
        stack_overflow();
    }

    // stack_overflow();

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
    interrupts::init();
}
