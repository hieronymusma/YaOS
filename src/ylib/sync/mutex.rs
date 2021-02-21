use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};

use super::spin_lock::SpinLock;

pub struct Mutex<T> {
    lock: SpinLock,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: ?Sized> {
    lock: &'a SpinLock,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: SpinLock::new(),
            data: UnsafeCell::new(user_data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.lock.lock();
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.release();
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}
