fn read_flags() -> u64 {
    let r: u64;
    unsafe {
        asm!("pushf; pop {}", out(reg) r)
    };
    r
}

pub fn are_interrupts_enabled() -> bool {
    let flags = read_flags();
    let interrupt_enable = flags & FlagMask::InterruptEnable.get_mask();
    match interrupt_enable {
        0 => false,
        _ => true
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
enum FlagMask {
    InterruptEnable = 0x0200
}

impl FlagMask {
    fn get_mask(&self) -> u64 {
        *self as u64
    }
}