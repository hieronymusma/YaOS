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
}
