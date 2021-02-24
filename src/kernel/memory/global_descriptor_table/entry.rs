use super::super::privilege_level::PrivilegeLevel;
use crate::ylib::utilities::bit_manipulator::BitManipulation;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Entry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: Access,
    limit_and_flags: FlagAndLimit,
    base_high: u8,
}

impl Entry {
    pub fn empty() -> Self {
        Entry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: Access::new(),
            limit_and_flags: FlagAndLimit::new(),
            base_high: 0,
        }
    }

    pub fn null_segment() -> Entry {
        Entry::empty()
    }

    pub fn kernel_code_segment() -> Entry {
        let mut entry = Entry::empty();
        entry.set_base(0);
        entry.set_limit(core::u32::MAX);

        entry.limit_and_flags.set_code_segment();
        entry.limit_and_flags.set_granularity(Granularity::ByteWise);

        entry.access.set_presence(1);
        entry.access.set_privilege_level(PrivilegeLevel::Ring0);
        entry
            .access
            .set_descriptor_type(DescriptorType::NonSystemSegment);
        entry.access.set_executable(true);
        entry
            .access
            .set_conforming_bit(ConformingMode::ExecutionOnlyFromSameRing);
        entry.access.set_read_write_bit(1);
        entry
    }

    fn set_limit(&mut self, limit: u32) {
        self.limit_low = limit as u16;
        self.limit_and_flags.set_upper_limit(limit);
    }

    fn set_base(&mut self, base: u32) {
        self.base_low = base as u16;
        self.base_mid = (base >> 16) as u8;
        self.base_high = (base >> 24) as u8;
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct FlagAndLimit(u8);

#[allow(dead_code)]
impl FlagAndLimit {
    fn new() -> FlagAndLimit {
        FlagAndLimit(0)
    }

    fn set_upper_limit(&mut self, limit: u32) {
        let limit = ((limit >> 16) & 0x4) as u8;
        self.0.set_bit(0, limit.get_bit(0));
        self.0.set_bit(1, limit.get_bit(1));
        self.0.set_bit(2, limit.get_bit(2));
        self.0.set_bit(3, limit.get_bit(3));
    }

    fn set_granularity(&mut self, granularity: Granularity) {
        self.0.set_bit(7, granularity as u8);
    }

    fn set_data_segment(&mut self) {
        self.0.set_bit(5, 0);
        self.0.set_bit(6, 1);
    }

    fn set_code_segment(&mut self) {
        self.0.set_bit(5, 1);
        self.0.set_bit(6, 0);
    }
}

#[allow(dead_code)]
enum Granularity {
    ByteWise = 0,
    PageWise = 1,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Access(u8);

#[allow(dead_code)]
impl Access {
    fn new() -> Access {
        Access(0)
    }

    fn set_presence(&mut self, is_present: u8) {
        self.0.set_bit(7, is_present);
    }

    fn set_privilege_level(&mut self, privilege_level: PrivilegeLevel) {
        let privilege_level = privilege_level as u8;
        let first_bit = privilege_level & 0x1;
        let second_bit = privilege_level & 0x2;
        self.0.set_bit(5, first_bit);
        self.0.set_bit(6, second_bit);
    }

    fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) {
        self.0.set_bit(4, descriptor_type as u8);
    }

    fn set_executable(&mut self, executable: bool) {
        self.0.set_bit(3, executable as u8);
    }

    fn set_direction_bit(&mut self, direction: GrowDirection) {
        self.0.set_bit(2, direction as u8);
    }

    fn set_conforming_bit(&mut self, conforming_mode: ConformingMode) {
        self.0.set_bit(2, conforming_mode as u8);
    }

    fn set_read_write_bit(&mut self, value: u8) {
        self.0.set_bit(1, value);
    }
}

#[allow(dead_code)]
enum ConformingMode {
    ExecutionOnlyFromSameRing = 0,
    EqualOrLowerRingAllowed = 1,
}

#[allow(dead_code)]
enum GrowDirection {
    Upwards = 0,
    Downwards = 1,
}

#[allow(dead_code)]
enum DescriptorType {
    NonSystemSegment = 0,
    SystemSegment = 1,
}
