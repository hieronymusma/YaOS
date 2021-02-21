use super::interrupt_stack_frame::InterruptStackFrame;

pub extern "x86-interrupt" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &InterruptStackFrame) {
    println!("EXCEPTION BREAKPOINT:\n {:#?}\n", stack_frame);
}
