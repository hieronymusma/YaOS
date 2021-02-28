#[derive(Debug, Clone, Copy)]
pub struct IDTEntryOptions(u16);

impl IDTEntryOptions {
    pub const fn minimal() -> Self {
        IDTEntryOptions(0b1110_0000_0000)
    }

    pub fn present() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.set_bit(15, present as u8);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.set_bit(8, !disable as u8);
        self
    }

    #[allow(dead_code)]
    pub fn set_privilege_level(&mut self, dpl: u8) -> &mut Self {
        self.set_bit(13, dpl & 0x1);
        self.set_bit(14, dpl & 0x2);
        self
    }

    pub unsafe fn set_stack_index(&mut self, index: u8) -> &mut Self {
        let index = index + 1;
        self.set_bit(0, index & 0x1);
        self.set_bit(1, (index >> 1) & 0x1);
        self.set_bit(2, (index >> 2) & 0x1);
        self
    }

    fn set_bit(&mut self, bit_number: u8, target_value: u8) {
        if target_value == 1 {
            self.0 |= 1 << bit_number;
        } else {
            self.0 &= !(1 << bit_number);
        }
    }
}
