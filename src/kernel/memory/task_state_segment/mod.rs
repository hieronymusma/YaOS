use crate::ylib::sync::lazy::Lazy;

use self::task_state_segment::TaskStateSegment;

use super::virt_addr::VirtAddr;

pub mod task_state_segment;

pub const DOUBLE_FAULT_STACK_INDEX: u8 = 0;
const STACK_SIZE: usize = 4096 * 5;

pub static TSS: Lazy<TaskStateSegment, fn() -> TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let start = VirtAddr::from_ptr(unsafe { &STACK });
        let end = VirtAddr::new(start.get() + (STACK_SIZE as u64));

        print!("IRQ STACK: {:#x}-{:#x}\n", start.get(), end.get());
        end
    };

    tss
});
