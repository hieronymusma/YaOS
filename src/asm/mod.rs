#[link(name = "x86_64_asm", kind = "static")]
extern "C" {
    #[cfg_attr(
        any(target_env = "gnu", target_env = "musl"),
        link_name = "_x86_64_asm_get_cs"
    )]
    pub(crate) fn x86_64_asm_get_cs() -> u16;

    #[cfg_attr(
        any(target_env = "gnu", target_env = "musl"),
        link_name = "_x86_64_asm_lidt"
    )]
    pub(crate) fn x86_64_asm_lidt(idt: *const crate::interrupts::idt::DescriptorTablePointer);
}