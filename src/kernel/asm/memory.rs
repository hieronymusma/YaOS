pub fn read_cr2() -> u64 {
    let mut _cr2: u64 = 0;
    unsafe {
        asm!("mov {}, cr2", out(reg) _cr2);
    }
    _cr2
}
