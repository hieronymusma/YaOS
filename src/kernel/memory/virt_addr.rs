use core::fmt;

use core::ops::Add;

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

    pub fn zero() -> VirtAddr {
        VirtAddr(0)
    }

    pub fn from_ptr<T>(ptr: *const T) -> VirtAddr {
        VirtAddr::new(ptr as u64)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtAddr(0x{:x})", self.0)
    }
}

// impl Add<u64> for VirtAddr {
//     type Output = Self;
//     fn add(self, rhs: u64) -> Self::Output {
//         VirtAddr::new(self.0 + rhs)
//     }
// }

// impl Add<usize> for VirtAddr {
//     type Output = Self;
//     fn add(self, rhs: usize) -> Self::Output {
//         self + rhs as u64
//     }
// }

/// A passed `u64` was not a valid virtual address.
///
/// This means that bits 48 to 64 are not
/// a valid sign extension and are not null either. So automatic sign extension would have
/// overwritten possibly meaningful bits. This likely indicates a bug, for example an invalid
/// address calculation.
#[derive(Debug)]
pub struct VirtAddrNotValid(u64);
