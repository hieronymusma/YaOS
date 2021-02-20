#![no_std] // don't link the Rust standard library

pub(crate) mod asm;

mod ylib;

#[macro_use]
pub mod vga_buffer;

mod interrupts;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {    
    clear_screen!();
    println!("Hello World{}", "!");
    println!("How are you?");
    
    interrupts::init();

    println!("Interrupt table is set up!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}