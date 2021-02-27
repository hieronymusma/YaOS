use super::interrupt_descriptor_table::interrupt_stack_frame::InterruptStackFrame;

pub extern "x86-interrupt" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &InterruptStackFrame) {
    let mut rsp: u64;
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp);
    }
    println!("RSP: {:#x}", rsp);
    println!("EXCEPTION BREAKPOINT:\n {:#?}\n", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &InterruptStackFrame,
    error_code: u64,
) -> ! {
    let mut rsp: u64;
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp);
    }
    println!("RSP: {:#x}", rsp);
    panic!(
        "EXCEPTION: DOUBLE FAULT (error_code: {})\n{:#?}",
        error_code, stack_frame
    );
}

pub extern "x86-interrupt" fn page_fault_handler(_stack_frame: &InterruptStackFrame, _error_code: u64) {
    println!("PAGE FAULT");
}