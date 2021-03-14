pub fn enable_nxe_bit() {
    let nxe_bit = 1 << 11;
    const IA32_EFER: u32 = 0xc0000080;
    let efer = rdmsr(IA32_EFER);
    wrmsr(IA32_EFER, efer | nxe_bit);
}

fn rdmsr(msr: u32) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") msr);
    }
    ((high as u64) << 32) | (low as u64)
}

fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
    }
}

pub fn enable_write_protect_bit() {
    let mut _cr0: u64 = 0;

    const WRITE_PROTECT: u64 = 1 << 16;

    unsafe {
        asm!("mov {}, cr0", out(reg) _cr0);
        _cr0 |= WRITE_PROTECT;
        asm!("mov cr0, {}", in(reg) _cr0);
    }
}