pub mod interrupt_description_table;

use crate::ylib::sync::lazy::Lazy;
use interrupt_description_table::interrupt_handler;
use interrupt_description_table::interrupt_types::IDTType;
use interrupt_description_table::table::InterruptDescriptionTable;

static IDT: Lazy<InterruptDescriptionTable, fn() -> InterruptDescriptionTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptionTable::new();
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

pub fn init_idt() {
    let static_ref = IDT.get_static_ref();
    static_ref.load();
    ok!("Load IDT at {:p}", static_ref);
}
