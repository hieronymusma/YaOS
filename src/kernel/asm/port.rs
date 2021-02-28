#[derive(Debug)]
pub struct Port {
    address: u16,
}

impl Port {
    pub const fn new(address: u16) -> Self {
        Port { address }
    }

    pub unsafe fn write(&self, value: u8) {
        self.write_offset(0, value);
    }

    pub unsafe fn write_offset(&self, offset: u8, value: u8) {
        asm!("out dx, al", in("dx") self.address + offset as u16, in("al") value);
    }

    pub unsafe fn read(&self) -> u8 {
        self.read_offset(0)
    }

    pub unsafe fn read_offset(&self, offset: u8) -> u8 {
        let mut _result: u8 = 0;
        asm!("in al, dx", out("al") _result, in("dx") self.address + offset as u16);
        _result
    }
}
