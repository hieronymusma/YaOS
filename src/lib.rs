#![no_std] // don't link the Rust standard library
// #![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();    

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u64;
    unsafe {
        *vga_buffer.offset(0) = 0x4f494f4e4f414f50; 
    }
    loop {}
}