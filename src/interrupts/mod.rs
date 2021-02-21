pub mod idt;
pub mod idt_entry;
pub mod idt_entry_options;
pub mod interrupt_handler;
pub mod interrupt_stack_frame;

use crate::ylib::sync::lazy::Lazy;
use idt::{IDTType, IDT};

static IDT: Lazy<IDT, fn() -> IDT> = Lazy::new(|| {
    let mut idt = IDT::new();
    idt.set_handler(
        IDTType::DivideByZero,
        interrupt_handler::divide_by_zero_handler as u64,
    );
    idt.set_handler(
        IDTType::Breakpoint,
        interrupt_handler::breakpoint_handler as u64,
    );
    idt
});

pub fn init() {
    IDT.lock().load();
    println!("IDT successfully load")
}
