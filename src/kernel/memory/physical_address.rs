use core::fmt::{self, Debug};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicalAddress({:#x?})", self.0)
    }
}