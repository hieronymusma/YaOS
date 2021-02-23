pub mod idt;
pub mod idt_entry;
pub mod idt_entry_options;
pub mod idt_type;
pub mod interrupt_handler;
pub mod interrupt_stack_frame;

use crate::ylib::sync::lazy::Lazy;
use idt::IDT;
use idt_type::IDTType;

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
    idt.set_handler(
        IDTType::DoubleFault,
        interrupt_handler::double_fault_handler as u64,
    );
    idt
});

pub fn init() {
    let static_ref = IDT.get_static_ref();
    static_ref.load();
    ok!("Load IDT at {:p}", static_ref);
}
