pub mod entry;
pub mod table;

use crate::ylib::sync::lazy::Lazy;

use self::entry::Entry;
use self::table::GlobalDescriptorTable;

use super::segment_selector::SegmentSelector;

static GDT: Lazy<(GlobalDescriptorTable, Selectors), fn() -> (GlobalDescriptorTable, Selectors)> =
    Lazy::new(|| {
        let mut gdt = GlobalDescriptorTable::new();

        gdt.add_entry(Entry::null_segment());
        let cs = gdt.add_entry(Entry::kernel_code_segment());

        (
            gdt,
            Selectors {
                cs,
                // tss
            },
        )
    });

struct Selectors {
    cs: SegmentSelector,
    // tss: SegmentSelector,
}

pub fn init() {
    let static_ref = GDT.get_static_ref();
    &(static_ref.0).load();
    unsafe {
        SegmentSelector::set_cs(static_ref.1.cs);
    }
    ok!("Load GDT at {:p}", &static_ref.0);
}
