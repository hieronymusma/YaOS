use crate::ylib::sync::mutex::{Mutex, MutexGuard};
use core::cell::Cell;
use core::ops::{Deref, DerefMut};

pub struct Lazy<T, F> {
    value: Mutex<Option<T>>,
    init_function: Cell<Option<F>>,
}

pub struct LazyGuard<'a, T>(MutexGuard<'a, Option<T>>);

unsafe impl<T, F> Sync for Lazy<T, F> {}
unsafe impl<T, F> Send for Lazy<T, F> {}

impl<T, F> Lazy<T, F> {
    pub const fn new(init_function: F) -> Lazy<T, F> {
        Lazy {
            value: Mutex::new(None),
            init_function: Cell::new(Some(init_function)),
        }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn lock(&self) -> LazyGuard<T> {
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

    pub fn get_static_ref(&'static self) -> &'static T {
        let value = self.lock();
        let ptr: *const T = value.deref();
        unsafe {
            ptr.as_ref()
                .expect("Cannot retrieve reference from static lazy.")
        }
    }
}

impl<'a, T> Deref for LazyGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.0.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for LazyGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.0.as_mut().unwrap()
    }
}
