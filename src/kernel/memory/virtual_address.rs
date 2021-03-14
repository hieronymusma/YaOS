use core::ops::Add;
use core::{fmt, usize};

use super::{paging::PHYSICAL_MEMORY_OFFSET, physical_address::PhysicalAddress};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub fn new(address: usize) -> VirtualAddress {
        VirtualAddress(address)
    }

    pub fn zero() -> VirtualAddress {
        VirtualAddress::new(0)
    }

    pub fn from_ptr<T>(ptr: *const T) -> VirtualAddress {
        VirtualAddress::new(ptr as usize)
    }

    pub fn from_ref<T>(reference: &T) -> VirtualAddress {
        VirtualAddress::from_ptr(reference as *const T)
    }

    pub fn get_physical_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.0 - PHYSICAL_MEMORY_OFFSET)
    }

    pub fn value(&self) -> usize {
        self.0
    }

    pub fn ptr<T>(&self) -> *mut T {
        self.value() as *mut T
    }

    pub fn get_page_table_indices(&self) -> (usize, usize, usize, usize) {
        let mut current = self.0 >> 12;

        let p1 = current & 0x1ff;
        current = current >> 9;

        let p2 = current & 0x1ff;
        current = current >> 9;

        let p3 = current & 0x1ff;
        current = current >> 9;

        let p4 = current & 0x1ff;

        (p4, p3, p2, p1)
    }
}

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtualAddress({:#x?})", self.0)
    }
}

impl Add<u64> for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        self + rhs as usize
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        VirtualAddress::new(self.0 + rhs)
    }
}
