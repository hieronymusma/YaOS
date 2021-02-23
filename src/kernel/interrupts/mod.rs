pub mod idt;
pub mod idt_entry;
pub mod idt_entry_options;
pub mod idt_type;
pub mod interrupt_handler;
pub mod interrupt_stack_frame;
pub mod tss;

use crate::ylib::sync::lazy::Lazy;
use idt::IDT;
use idt_type::IDTType;

use crate::memory::virt_addr::VirtAddr;
use tss::TaskStateSegment;

const DOUBLE_FAULT_STACK_INDEX: u8 = 0;

static TSS: Lazy<TaskStateSegment, fn() -> TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let start = VirtAddr::from_ptr(unsafe { &STACK });
        let end = start + STACK_SIZE;
        end
    };
    tss
});

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

pub fn init_idt() {
    let static_ref = IDT.get_static_ref();
    static_ref.load();
    ok!("Load IDT at {:p}", static_ref);
}
