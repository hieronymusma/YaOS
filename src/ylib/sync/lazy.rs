use crate::ylib::sync::spin_lock::SpinLock;
use core::cell::Cell;
use core::ops::Deref;

pub struct Lazy<T, F> {
    value: Option<T>,
    init_function: Cell<Option<F>>,
    lock: SpinLock,
}

// pub struct LazyGuard<'a, T>(MutexGuard<'a, Option<T>>);

unsafe impl<T, F> Sync for Lazy<T, F> {}
unsafe impl<T, F> Send for Lazy<T, F> {}

impl<T, F> Lazy<T, F> {
    pub const fn new(init_function: F) -> Lazy<T, F> {
        Lazy {
            value: None,
            init_function: Cell::new(Some(init_function)),
            lock: SpinLock::new(),
        }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    fn init(&self) {
        {
            self.lock.lock();
            // SAFETY: Init is locked by spinlock.
            unsafe {
                let option_ptr = &self.value as *const Option<T>;
                let mut_option_ptr = option_ptr as *mut Option<T>;
                let value = &mut *mut_option_ptr;

                if value.is_none() {
                    let new_value = match self.init_function.take() {
                        Some(f) => f(),
                        None => panic!("Lazy instance has previously been poisoned"),
                    };
                    value.get_or_insert(new_value);
                }
            }
            self.lock.release();
        }
    }

    pub fn get_static_ref(&'static self) -> &'static T {
        self.init();
        self.value.as_ref().unwrap()
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        self.init();
        &*self.value.as_ref().unwrap()
    }
}
