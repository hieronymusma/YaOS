use core::hint::spin_loop;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock(AtomicBool);

impl SpinLock {
    pub const fn new() -> Self {
        SpinLock(AtomicBool::new(false))
    }

    pub fn lock(&self) {
        while self
            .0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            != Ok(false)
        {
            while self.0.load(Ordering::Relaxed) {
                spin_loop();
            }
        }
    }

    pub fn release(&self) {
        self.0.store(false, Ordering::Release)
    }
}
