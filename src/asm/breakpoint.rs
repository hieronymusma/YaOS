#[allow(dead_code)]
pub unsafe fn int3() {
    asm!("int3", options(nomem, nostack));
}
