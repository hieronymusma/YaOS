#[derive(Debug, Clone, Copy)]
pub enum IDTType {
    DivideByZero = 0x00,
    Breakpoint = 0x03,
    DoubleFault = 0x08,
}
