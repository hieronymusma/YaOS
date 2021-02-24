pub mod entry;
pub mod table;

use crate::ylib::sync::lazy::Lazy;

use self::entry::Entry;
use self::table::GlobalDescriptorTable;
use super::{task_state_segment::TSS, VirtAddr};

use super::{
    segment_selector::SegmentSelector, task_state_segment::task_state_segment::TaskStateSegment,
};

// const DOUBLE_FAULT_STACK_INDEX: usize = 0;
// const STACK_SIZE: usize = 4096 * 5;

// static TSS: Lazy<TaskStateSegment, fn() -> TaskStateSegment> = Lazy::new(|| {
//     let mut tss = TaskStateSegment::new();

//     tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX] = {
//         static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

//         let start = VirtAddr::from_ptr(unsafe { &STACK });
//         let end = start + STACK_SIZE;
//         end
//     };

//     tss
// });

static GDT: Lazy<(GlobalDescriptorTable, Selectors), fn() -> (GlobalDescriptorTable, Selectors)> =
    Lazy::new(|| {
        let mut gdt = GlobalDescriptorTable::new();

        gdt.add_entry(Entry::null_segment());
        let cs = gdt.add_entry(Entry::kernel_code_segment());
        let tss = gdt.add_tss(TSS.get_static_ref());

        (gdt, Selectors { cs, tss })
    });

struct Selectors {
    cs: SegmentSelector,
    tss: SegmentSelector,
}

pub fn init() {
    let static_ref = GDT.get_static_ref();
    &(static_ref.0).load();
    unsafe {
        SegmentSelector::set_cs(static_ref.1.cs);
        SegmentSelector::load_tss(static_ref.1.tss);
    }
    ok!("Load GDT at {:p}", &static_ref.0);
}
