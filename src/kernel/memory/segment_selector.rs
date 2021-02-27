use super::privilege_level::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    pub fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        SegmentSelector(index << 3 | (rpl as u16))
    }

    pub fn from_value(value: u16) -> Self {
        SegmentSelector(value)
    }

    pub fn get_cs() -> Self {
        let segment: u16;
        unsafe { asm!("mov {0:x}, cs", out(reg) segment, options(nostack, nomem)) };
        SegmentSelector::from_value(segment)
    }

    pub unsafe fn set_cs(sel: SegmentSelector) {
        asm!(
            "push {sel}",
            "lea {tmp}, [1f + rip]",
            "push {tmp}",
            "retfq",
            "1:",
            sel = in(reg) u64::from(sel.0),
            tmp = lateout(reg) _,
        );
    }

    pub unsafe fn load_tss(sel: SegmentSelector) {
        asm!("ltr {0:x}", in(reg) sel.0, options(nostack, nomem));
    }
}
