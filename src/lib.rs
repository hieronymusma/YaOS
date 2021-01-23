#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main() {}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
