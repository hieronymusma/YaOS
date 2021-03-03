pub fn halt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}