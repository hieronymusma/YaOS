pub mod handler_funcs;
pub mod idt;
pub mod idt_entry;
pub mod idt_entry_options;

use crate::ylib::sync::lazy::Lazy;
use idt::{IDTType, IDT};

static IDT: Lazy<IDT, fn() -> IDT> = Lazy::new(|| {
    let mut idt = IDT::new();
    idt.set_handler(IDTType::DivideByZero, divide_by_zero_handler);
    idt
});

pub fn init() {
    IDT.lock().load();
    println!("IDT successfully load")
}

extern "x86-interrupt" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}
