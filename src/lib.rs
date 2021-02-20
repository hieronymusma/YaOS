#![no_std] // don't link the Rust standard library
#![feature(asm)]
#![feature(llvm_asm)]

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

    divide_by_zero();

    println!("We did not crash!");
 
    loop {}
}

fn divide_by_zero() {
    unsafe {
        llvm_asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}