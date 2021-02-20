pub mod idt;

use crate::ylib::sync::lazy::{Lazy, LazyGuard};
use idt::Idt;

static IDT: Lazy<Idt, fn() -> Idt> = Lazy::new(|| {
    let mut idt = Idt::new();

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