pub unsafe fn outb(address: u16, value: u8) {
    asm!("out dx, al", in("dx") address, in("al") value);
}

pub unsafe fn inb(address: u16) -> u8 {
    let mut result: u8 = 0;
    asm!("in al, dx", out("al") result, in("dx") address);
    result
}
