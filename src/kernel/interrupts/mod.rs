pub mod interrupt_descriptor_table;
pub mod interrupt_handler;

use crate::ylib::sync::lazy::Lazy;
use interrupt_descriptor_table::interrupt_types::IDTType;
use interrupt_descriptor_table::table::InterruptDescriptionTable;

static IDT: Lazy<InterruptDescriptionTable, fn() -> InterruptDescriptionTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptionTable::new();

    idt[IDTType::DivideByZero].set_handler(interrupt_handler::divide_by_zero_handler as u64);
    idt[IDTType::Breakpoint].set_handler(interrupt_handler::breakpoint_handler as u64);

    let double_fault_options = idt[IDTType::DoubleFault].set_handler(interrupt_handler::double_fault_handler as u64);

    unsafe {
        double_fault_options.set_stack_index(0);
    }

    idt[IDTType::PageFault].set_handler(interrupt_handler::page_fault_handler as u64);

    idt
});

pub fn init_idt() {
    let static_ref = IDT.get_static_ref();
    static_ref.load();
    ok!("Load IDT at {:p}", static_ref);
}
