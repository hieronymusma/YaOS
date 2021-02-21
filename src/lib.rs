#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(abi_x86_interrupt)]

mod ylib;

#[macro_use]
pub mod vga_buffer;

mod interrupts;
mod memory;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen!();
    println!("Hello World{}", "!");
    println!("How are you?");

    init();

    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    println!("We did not crash!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init() {
    interrupts::init();
}
