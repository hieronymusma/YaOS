use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Drop, Deref, DerefMut};

pub struct Mutex<T>
{
    lock: AtomicBool,
    data: UnsafeCell<T>
}

pub struct MutexGuard<'a, T: ?Sized> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }
    
    pub fn lock(&self) -> MutexGuard<T> {
        self.obtain_lock();
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    fn obtain_lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) != Ok(false)
        {
            while self.lock.load(Ordering::Relaxed) {
                spin_loop();            
            }   
        }
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T>
{
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T { &*self.data }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T>
{
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { &mut *self.data }
}