use crate::{asm::Port, interrupts::interrupt_descriptor_table::table::InterruptType};

// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;

/// Command sent to acknowledge an interrupt.
const CMD_END_OF_INTERRUPT: u8 = 0x20;

// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

const PIC1_ADDRESS: u16 = 0x20;
const PIC2_ADDRESS: u16 = 0xa0;

struct PIC {
    interrupt_offset: u8,
    command: Port,
    data: Port,
}

impl PIC {
    fn new(offset: u8, address: u16) -> Self {
        PIC {
            interrupt_offset: offset,
            command: Port::new(address),
            data: Port::new(address + 1),
        }
    }

    pub fn send_end_of_interrupt(&self) {
        unsafe {
            self.command.write(CMD_END_OF_INTERRUPT);
        }
    }
}

#[allow(non_camel_case_types)]
pub struct x86PIC {
    pic1: PIC,
    pic2: PIC,
}

impl x86PIC {
    pub fn new(interrupt_offset_pic1: u8, interrupt_offset_pic2: u8) -> Self {
        x86PIC {
            pic1: PIC::new(interrupt_offset_pic1, PIC1_ADDRESS),
            pic2: PIC::new(interrupt_offset_pic2, PIC2_ADDRESS),
        }
    }

    pub unsafe fn init(&self) {
        // Code is used from https://docs.rs/crate/pic8259_simple/0.2.0/source/src/lib.rs

        // Older motherboards require a wait
        let wait_port: Port = Port::new(0x80);
        let wait = || wait_port.write(0);

        // Save our original interrupt masks, because I'm too lazy to
        // figure out reasonable values.  We'll restore these when we're
        // done.
        let saved_mask1 = self.pic1.data.read();
        let saved_mask2 = self.pic2.data.read();

        // Tell each PIC that we're going to send it a three-byte
        // initialization sequence on its data port.
        self.pic1.command.write(CMD_INIT);
        wait();
        self.pic2.command.write(CMD_INIT);
        wait();

        // Byte 1: Set up our base offsets.
        self.pic1.data.write(self.pic1.interrupt_offset);
        wait();
        self.pic2.data.write(self.pic2.interrupt_offset);
        wait();

        // Byte 2: Configure chaining between PIC1 and PIC2.
        self.pic1.data.write(4);
        wait();
        self.pic2.data.write(2);
        wait();

        // Byte 3: Set our mode.
        self.pic1.data.write(MODE_8086);
        wait();
        self.pic2.data.write(MODE_8086);
        wait();

        // Restore our saved masks.
        self.pic1.data.write(saved_mask1);
        self.pic2.data.write(saved_mask2);

        ok!("PICs initialized");
    }

    pub fn send_end_of_interrupt(&self, interrupt: InterruptType) {
        // Ignore if interrupt is not from pics
        if interrupt.as_usize() > 15 {
            return;
        }
        // Is interrupt from second PIC?
        if interrupt.as_usize() > 7 {
            self.pic2.send_end_of_interrupt();
        }
        self.pic1.send_end_of_interrupt();
    }
}
