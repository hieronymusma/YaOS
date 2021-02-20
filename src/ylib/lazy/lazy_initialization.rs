use crate::ylib::sync::mutex::{Mutex, MutexGuard};
use core::ops::{Deref, DerefMut};
use core::cell::Cell;

pub struct LazyInitializer<T, F> {
    value: Mutex<Option<T>>,
    init_function: Cell<Option<F>>,
}

pub struct LazyGuard<'a, T>(MutexGuard<'a, Option<T>>);

unsafe impl<T, F> Sync for LazyInitializer<T, F> {}
unsafe impl<T, F> Send for LazyInitializer<T, F> {}

impl<T, F> LazyInitializer<T, F> {
    pub const fn new(init_function: F) -> LazyInitializer<T, F> {
        LazyInitializer {
            value: Mutex::new(None),
            init_function: Cell::new(Some(init_function)),
        }
    }
}

impl<T, F: FnOnce() -> T> LazyInitializer<T, F> {
    pub fn get(&self) -> LazyGuard<T> {
        {
            let mut value = self.value.lock();
            if value.is_none() {
                let new_value = match self.init_function.take() {
                    Some(f) => f(),
                    None => panic!("Lazy instance has previously been poisoned"),
                };
                value.get_or_insert(new_value);
            }
        }
        LazyGuard(self.value.lock())
    }
}

impl<'a, T> Deref for LazyGuard<'a, T>
{
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T { 
        &*self.0.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for LazyGuard<'a, T>
{
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { 
        &mut *self.0.as_mut().unwrap()
    }
}