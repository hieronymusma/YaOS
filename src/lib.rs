#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(abi_x86_interrupt)]

#[macro_use]
pub mod vga_buffer;

mod asm;
mod ylib;

mod interrupts;
mod memory;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen!();
    println!("Hello World{}", "!");
    println!("How are you?");

    init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    // unsafe {
    //     asm!("int 3", options(nomem, nostack))
    // }

    println!("We did not crash!");

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
