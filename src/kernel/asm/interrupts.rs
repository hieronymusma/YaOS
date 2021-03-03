pub unsafe fn enable_interrupts() {
    asm!("sti")
}

pub unsafe fn disable_interrupts() {
    asm!("cli");
}
