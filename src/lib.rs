#![no_std] // don't link the Rust standard library

pub(crate) mod asm;

mod ylib;
mod interrupts;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {    
    clear_screen!();
    println!("Hello World{}", "!");
    println!("How are you?");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}