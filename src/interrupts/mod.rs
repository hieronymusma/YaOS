pub mod idt;

use crate::ylib::sync::lazy::Lazy;

static IDT: Lazy<idt::Idt, fn() -> idt::Idt> = Lazy::new(|| {
    let mut idt = idt::Idt::new();

    idt.set_handler(0, divide_by_zero_handler);

    idt
});

pub fn init() {
    IDT.lock().load();
}

extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}