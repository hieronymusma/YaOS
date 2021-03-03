use crate::ylib::sync::{lazy::Lazy, mutex::Mutex};

use self::pic::x86PIC;

pub mod pic;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Lazy<Mutex<x86PIC>, fn() -> Mutex<x86PIC>> = Lazy::new(|| {
    let pics = x86PIC::new(PIC_1_OFFSET, PIC_2_OFFSET);
    Mutex::new(pics)
});