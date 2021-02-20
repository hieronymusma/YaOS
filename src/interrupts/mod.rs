pub mod idt;

use crate::ylib::sync::lazy::{Lazy, LazyGuard};
use idt::Idt;

lazy_static!{
    static ref IDT: Idt = {
        let mut idt = Idt::new();
    
        idt.set_handler(0, divide_by_zero_handler);
    
        idt
    };    
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}