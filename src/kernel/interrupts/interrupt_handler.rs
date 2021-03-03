use super::interrupt_descriptor_table::interrupt_stack_frame::InterruptStackFrame;
use super::interrupt_descriptor_table::table::InterruptType;
use crate::memory::paging::page_fault_error_code::PageFaultErrorCode;
use crate::{asm::Port, pic::PICS};

const KEYBOARD_CONTROLLER_ADDRESS: u16 = 0x60;

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
    stack_frame: &InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("PAGE FAULT");
    println!("Accessed Address: {:#x?}", crate::asm::memory::read_cr2());
    println!("Error code: {:#?}", error_code);
    println!("{:#?}", stack_frame);
    crate::asm::halt::halt_loop();
}

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &InterruptStackFrame) {
    PICS.lock().send_end_of_interrupt(InterruptType::Timer);
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &InterruptStackFrame) {
    let port = Port::new(KEYBOARD_CONTROLLER_ADDRESS);
    let scancode = unsafe { port.read() };

    // MSB bit indicates if key is pressed (0) or released (1)
    if scancode & 0x80 == 0 {
        serial_println!("Press: {} ", scancode);
    } else {
        serial_println!("Release: {}", scancode & 0x7f);
    }
    PICS.lock().send_end_of_interrupt(InterruptType::Keyboard);
}
