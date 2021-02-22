use crate::ylib::sync::spin_lock::SpinLock;
use core::cell::Cell;
use core::ops::Deref;

pub struct Lazy<T, F> {
    value: Option<T>,
    init_function: Cell<Option<F>>,
    lock: SpinLock,
}

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

            if self.value.is_none() {
                let new_value = match self.init_function.take() {
                    Some(f) => f(),
                    None => panic!("Lazy instance has previously been poisoned"),
                };
                unsafe {
                    // SAFETY: Init is locked by spinlock.
                    let mut_option = &mut *(&self.value as *const Option<T> as *mut Option<T>);
                    mut_option.get_or_insert(new_value);
                }
            }

            self.lock.release();
        }
    }

    pub fn get_static_ref(&'static self) -> &'static T {
        self.init();
        self.deref()
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        self.init();
        self.value.as_ref().unwrap()
    }
}
