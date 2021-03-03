use core::fmt;

// Access violation occured
const PROTECTION_VIOLATION: u64 = 1;

// Memory access which caused the page fault was a write
const CAUSED_BY_WRITE: u64 = 1 << 1;

// Access occured in User Mode
const USER_MODE: u64 = 1 << 2;

// Reserved bit in page table was set
const MALFORMED_TABLE: u64 = 1 << 3;

#[repr(transparent)]
pub struct PageFaultErrorCode(u64);

impl fmt::Debug for PageFaultErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debugger = f.debug_struct("PageFaultErrorCode");

        let protection_violation = self.0 & PROTECTION_VIOLATION;
        let caused_by_write = (self.0 & CAUSED_BY_WRITE) >> 1;
        let user_mode = (self.0 & USER_MODE) >> 2;
        let malformed_table = (self.0 & MALFORMED_TABLE) >> 3;

        debugger.field("PROTECTION_VIOLATION", &protection_violation);
        debugger.field("CAUSED_BY_WRITE", &caused_by_write);
        debugger.field("USER_MODE", &user_mode);
        debugger.field("MALFORMED_TABLE", &malformed_table);

        debugger.finish()
    }
}
