use core::{fmt, marker::PhantomData, ops::{Index, IndexMut}};

use crate::{memory::{allocator::{allocate_frame}, physical_address::PhysicalAddress, virtual_address::VirtualAddress}};

use super::page_table_entry::PageTableEntry;

#[repr(align(4096))]
pub struct PageTable<L: TableLevel> {
    entries: [PageTableEntry; 512],
    level: PhantomData<L>,
}

impl<L: TableLevel> fmt::Debug for PageTable<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, entry) in self.entries.iter().enumerate() {
            f.write_fmt(format_args!("Page Table Entry {}: {:#x}\n", index, entry.value()))?;
        }
        Ok(())
    }
} 

impl<L: TableLevel> PageTable<L> {
    fn allocate_internal() -> &'static mut PageTable<L> {
        let frame = allocate_frame().expect("Allocation of frame failed.");
        let address = frame.start().get_virtual_address();
    
        let page_table = unsafe { address.ptr::<PageTable<L>>().as_mut().unwrap() };
        page_table
    }

    pub fn get_address(&self) -> PhysicalAddress {
        let address = VirtualAddress::from_ptr(self);
        address.get_physical_address()
    }
}

impl<L: HierarchicalLevel> PageTable<L> {
    pub fn get_table_at(&mut self, index: usize) -> &'static mut PageTable<L::NextLevel> {
        let entry = &mut self[index];

        if !entry.is_present() {
            let new_table = PageTable::<L::NextLevel>::allocate_internal();
            let address = new_table.get_address();
            entry.set_address(&address);
            entry.set_present(true);
            entry.set_writable(true);
        }

        let next_table_address = entry.get_address().get_virtual_address();

        unsafe {
            next_table_address.ptr::<PageTable<L::NextLevel>>().as_mut().unwrap()
        }
    }
}

impl PageTable<Level4> {
    pub fn allocate() -> &'static mut PageTable<Level4> {
        PageTable::<Level4>::allocate_internal()
    }

    pub fn map(&mut self, frame: &PhysicalAddress, address: &VirtualAddress) -> &'static mut PageTableEntry {
        assert_eq!(frame.value() & 0xfff, 0);
        assert_eq!(address.value() & 0xfff, 0);

        let (p4_index, p3_index, p2_index, p1_index) = address.get_page_table_indices();

        let p3 = self.get_table_at(p4_index);
        let p2 = p3.get_table_at(p3_index);
        let p1 = p2.get_table_at(p2_index);

        let entry = &mut p1.entries[p1_index];
        entry.set_address(frame);

        entry
    }

    pub fn identity_map(&mut self, address: &PhysicalAddress) {
        let virtual_address = VirtualAddress::new(address.value());
        let entry = self.map(address, &virtual_address);
        entry.set_writable(true);
        entry.set_present(true);
    }
}

impl<L: TableLevel> Index<usize> for PageTable<L> {
    type Output = PageTableEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<L: TableLevel> IndexMut<usize> for PageTable<L> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

pub trait TableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}