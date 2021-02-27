pub mod serial;

pub use serial::*;

use crate::ylib::sync::{lazy::Lazy, mutex::Mutex};
use core::fmt;

const COM1_ADDRESS: u16 = 0x3f8;

static COM1: Lazy<Mutex<SerialPort>, fn() -> Mutex<SerialPort>> = Lazy::new(|| {
    let serial = SerialPort::create_and_init(COM1_ADDRESS);
    let serial = serial.expect("Could not init COM1.");
    Mutex::new(serial)
});

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_serial_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    use core::fmt::Write;
    COM1.lock().write_fmt(args).unwrap();
}
