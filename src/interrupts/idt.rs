use core::fmt;

pub struct IDT([IDTEntry; 16]);

impl IDT {
    pub fn new() -> IDT {
        IDT([IDTEntry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry: IDTType, handler: HandlerFunc) -> &mut IDTEntryOptions {
        self.0[entry as usize] = IDTEntry::new(IDT::get_cs(), handler);
        unsafe { &mut self.0[entry as usize].options }
    }

    // Should be 'static
    pub fn load(&self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { IDT::load_idt(&ptr); }
    }

    fn get_cs() -> SegmentSelector {
        let segment: u16;
        unsafe { asm!("mov {0:x}, cs", out(reg) segment, options(nostack, nomem)) };
        SegmentSelector(segment)
    }

    pub unsafe fn load_idt(gdt: &DescriptorTablePointer) {
        asm!("lidt [{}]", in(reg) gdt, options(nostack));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IDTType {
    DivideByZero = 0
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: VirtAddr,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(addr: u64) -> VirtAddr {
        Self::try_new(addr).expect(
            "address passed to VirtAddr::new must not contain any data \
             in bits 48 to 64",
        )
    }

    pub fn try_new(addr: u64) -> Result<VirtAddr, VirtAddrNotValid> {
        match addr & (u64::MAX << 47) {
            0 | 0x1ffff => Ok(VirtAddr(addr)),     // address is canonical
            1 => Ok(VirtAddr::new_truncate(addr)), // address needs sign extension
            other => Err(VirtAddrNotValid(other)),
        }
    }

    #[inline]
    pub const fn new_truncate(addr: u64) -> VirtAddr {
        // By doing the right shift as a signed operation (on a i64), it will
        // sign extend the value, repeating the leftmost bit.
        VirtAddr(((addr << 16) as i64 >> 16) as u64)
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtAddr({:#x})", self.0)
    }
}

/// A passed `u64` was not a valid virtual address.
///
/// This means that bits 48 to 64 are not
/// a valid sign extension and are not null either. So automatic sign extension would have
/// overwritten possibly meaningful bits. This likely indicates a bug, for example an invalid
/// address calculation.
#[derive(Debug)]
pub struct VirtAddrNotValid(u64);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: IDTEntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32   
}

impl IDTEntry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        IDTEntry {
            gdt_selector: gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: IDTEntryOptions::new(),
            reserved: 0
        }
    }

    fn missing() -> Self {
        IDTEntry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: IDTEntryOptions::minimal(),
            reserved: 0,
        }
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn() -> !;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    pub fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        SegmentSelector(index << 3 | (rpl as u16))
    }
}

pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct IDTEntryOptions(u16);

impl IDTEntryOptions {
    fn minimal() -> Self {
        IDTEntryOptions(0b1110_0000_0000)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.set_bit(15, present as u8);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.set_bit(8, !disable as u8);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u8) -> &mut Self {
        self.set_bit(13, dpl & 0x1);
        self.set_bit(14, dpl & 0x2);
        self
    }

    pub fn set_stack_index(&mut self, index: u8) -> &mut Self {
        self.set_bit(0, index & 0x1);
        self.set_bit(1, index & 0x2);
        self.set_bit(2, index & 0x3);
        self
    }

    fn set_bit(&mut self, bit_number: u8, target_value: u8) {
        if target_value != 0 {
            self.0 |= 1 << bit_number;
        } else {
            self.0 &= !(1 << bit_number);
        }
    }
}