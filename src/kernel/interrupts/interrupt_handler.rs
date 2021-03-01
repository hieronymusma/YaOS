use super::interrupt_descriptor_table::interrupt_stack_frame::InterruptStackFrame;

pub extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: &InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE BY ZERO");
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &InterruptStackFrame) {
    println!("EXCEPTION BREAKPOINT:\n {:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT (error_code: {})\n{:#?}",
        error_code, stack_frame
    );
}

pub extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: &InterruptStackFrame,
    _error_code: u64,
) {
    panic!("PAGE FAULT");
}
