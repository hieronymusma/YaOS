use crate::memory::physical_address::PhysicalAddress;

pub fn read_cr2() -> u64 {
    let mut _cr2: u64 = 0;
    unsafe {
        asm!("mov {}, cr2", out(reg) _cr2);
    }
    _cr2
}

pub fn read_cr3() -> PhysicalAddress {
    let mut _cr3: usize = 0;
    unsafe {
        asm!("mov {}, cr3", out(reg) _cr3);
    }
    PhysicalAddress::new(_cr3)
}

pub fn write_cr3(address: PhysicalAddress) {
    unsafe {
        asm!("mov cr3, {}", in(reg) address.value());
    }
}