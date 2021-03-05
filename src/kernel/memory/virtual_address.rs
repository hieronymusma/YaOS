use core::{fmt, usize};
use core::ops::Add;

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