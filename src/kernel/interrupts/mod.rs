pub mod interrupt_descriptor_table;
pub mod interrupt_handler;

use crate::ylib::sync::lazy::Lazy;
use interrupt_descriptor_table::table::{InterruptDescriptorTable, InterruptType};

static IDT: Lazy<InterruptDescriptorTable, fn() -> InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.divide_by_zero_error
        .set_handler(interrupt_handler::divide_by_zero_handler);
    idt.breakpoint
        .set_handler(interrupt_handler::breakpoint_handler);

    let double_fault_options = idt
        .double_fault
        .set_handler(interrupt_handler::double_fault_handler);

    unsafe {
        double_fault_options.set_stack_index(0);
    }

    idt.page_fault
        .set_handler(interrupt_handler::page_fault_handler);

    idt[InterruptType::Timer].set_handler(interrupt_handler::timer_handler);

    idt
});

pub fn init_idt() {
    let static_ref = IDT.get_static_ref();
    static_ref.load();
    ok!("Load IDT at {:p}", static_ref);
}

pub fn disable_interrupts_for<F, R>(func: F) -> R
where F: Fn() -> R {
    let interrupts_enabled = crate::asm::flags::are_interrupts_enabled();

    if interrupts_enabled {
        crate::asm::interrupts::disable_interrupts();
    }
    let ret = func();

    if interrupts_enabled {
        crate::asm::interrupts::enable_interrupts();
    }
    ret
}