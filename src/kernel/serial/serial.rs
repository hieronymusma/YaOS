use core::fmt;

use crate::asm::Port;

#[derive(Debug)]
pub struct SerialPort {
    port: Port,
}

impl SerialPort {
    pub fn create_and_init(port: u16) -> Result<Self, SerialIsFaulty> {
        let port = SerialPort {
            port: Port::new(port),
        };
        let result = port.init();
        match result {
            Ok(_) => Ok(port),
            Err(err) => Err(err),
        }
    }

    fn init(&self) -> Result<(), SerialIsFaulty> {
        unsafe {
            self.port.write_offset(1, 0x00); // Disable all interrupts
            self.port.write_offset(3, 0x80); // Enable DLAB (set baud rate divisor)
            self.port.write_offset(0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
            self.port.write_offset(1, 0x00); //                  (hi byte)
            self.port.write_offset(3, 0x03); // 8 bits, no parity, one stop bit
            self.port.write_offset(2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
            self.port.write_offset(4, 0x0B); // IRQs enabled, RTS/DSR set
            self.port.write_offset(4, 0x1E); // Set in loopback mode, test the serial chip
            self.port.write_offset(0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same b   self.read_offset

            // Check if serial is faulty (i.e: not same byte as sent)
            if self.port.read() != 0xAE {
                return Err(SerialIsFaulty);
            }

            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            self.port.write_offset(4, 0x0F);
        }
        Ok(())
    }

    fn is_transmit_empty(&self) -> u8 {
        return unsafe { self.port.read_offset(5) & 0x20 };
    }

    pub fn write_char(&self, a: char) {
        while self.is_transmit_empty() == 0 {}

        unsafe {
            self.port.write(a as u8);
        }
    }

    pub fn write_string(&self, message: &str) {
        for letter in message.chars() {
            self.write_char(letter);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[derive(Debug)]
pub struct SerialIsFaulty;
