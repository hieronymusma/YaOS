pub mod idt;

static IDT: idt::Idt = {
    let mut idt = idt::Idt::new();

    idt.set_handler(0, divide_by_zero_handler);

    idt
};

extern "C" fn divide_by_zero_handler() -> ! {
    // println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}