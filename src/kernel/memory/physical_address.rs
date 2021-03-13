use core::convert::TryFrom;
use core::{
    fmt::{self},
    usize,
};

use super::{paging::PHYSICAL_MEMORY_OFFSET, virtual_address::VirtualAddress};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub fn new(address: usize) -> PhysicalAddress {
        PhysicalAddress(address)
    }

    pub fn invalid() -> PhysicalAddress {
        PhysicalAddress(usize::MAX)
    }

    pub fn from_64bit(address: u64) -> PhysicalAddress {
        match usize::try_from(address) {
            Ok(address) => PhysicalAddress::new(address),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn from_32bit(address: u32) -> PhysicalAddress {
        match usize::try_from(address) {
            Ok(address) => PhysicalAddress::new(address),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_virtual_address(&self) -> VirtualAddress {
        VirtualAddress::new(self.0 + PHYSICAL_MEMORY_OFFSET)
    }
}

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicalAddress({:#x?})", self.0)
    }
}

impl core::ops::Add<usize> for PhysicalAddress {
    type Output = PhysicalAddress;
    fn add(self, rhs: usize) -> Self::Output {
        PhysicalAddress::new(self.0 + rhs)
    }
}

impl core::ops::AddAssign<usize> for PhysicalAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
