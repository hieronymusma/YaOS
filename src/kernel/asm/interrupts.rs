pub fn enable_interrupts() {
    unsafe { asm!("sti"); }
}

pub fn disable_interrupts() {
    unsafe { asm!("cli") }
}
