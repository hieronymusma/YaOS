use core::result;

use crate::asm::in_out::*;

#[derive(Debug)]
pub struct SerialPort {
    port_address: u16
}

impl SerialPort {
    pub fn create_and_init(port: u16) -> Result<Self, SerialIsFaulty> {
        let port = SerialPort {
            port_address: port,
        };
        let result = port.init();
        match result {
            Ok(_) => Ok(port),
            Err(err) => Err(err)
        }
    }

    fn init(&self) -> Result<(), SerialIsFaulty>  {
        unsafe {
            outb(self.port_address + 1, 0x00);    // Disable all interrupts
            outb(self.port_address + 3, 0x80);    // Enable DLAB (set baud rate divisor)
            outb(self.port_address + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
            outb(self.port_address + 1, 0x00);    //                  (hi byte)
            outb(self.port_address + 3, 0x03);    // 8 bits, no parity, one stop bit
            outb(self.port_address + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
            outb(self.port_address + 4, 0x0B);    // IRQs enabled, RTS/DSR set
            outb(self.port_address + 4, 0x1E);    // Set in loopback mode, test the serial chip
            outb(self.port_address + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
          
            // Check if serial is faulty (i.e: not same byte as sent)
            if inb(self.port_address + 0) != 0xAE {
               return Err(SerialIsFaulty);
            }
          
            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            outb(self.port_address + 4, 0x0F);
        }
        Ok(())
    }

    fn is_transmit_empty(&self) -> u8 {
        return unsafe { inb(self.port_address + 5) & 0x20 };
     }
      
    pub fn write_serial(&self, a: char) {
        while self.is_transmit_empty() == 0 { }
      
        unsafe { outb(self.port_address,a as u8); }
     }
}

#[derive(Debug)]
pub struct SerialIsFaulty;