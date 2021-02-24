pub mod entry;
pub mod table;

use crate::ylib::sync::lazy::Lazy;

use self::entry::Entry;
use self::table::GlobalDescriptorTable;

static GDT: Lazy<GlobalDescriptorTable, fn() -> GlobalDescriptorTable> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();

    gdt.add_entry(Entry::null_segment());
    gdt.add_entry(Entry::kernel_code_segment());

    gdt
});

pub fn init() {
    let static_ref = GDT.get_static_ref();
    static_ref.load();
    ok!("Load GDT at {:p}", static_ref);
}
