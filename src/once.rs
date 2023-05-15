use core::sync::atomic::{AtomicU8, Ordering};

use vc6_sys as winapi;

#[derive(Default)]
pub struct Once {
    lock: AtomicU8,
}

impl Once {
    pub const fn new() -> Self {
        Once {
            lock: AtomicU8::new(0),
        }
    }

    pub fn call(&self, func: impl FnOnce()) {
        while self.lock.load(Ordering::SeqCst) != 2 {
            let val = self.lock.compare_exchange(
                0,
                1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            match val {
                Ok(_) => {
                    // we got it!
                    func();
                    self.lock.store(2, Ordering::SeqCst);
                    return;
                }
                Err(_) => {
                    // another thread got it.
                    // sleeping for 1 millisecond is the closest thing to a
                    // yield we have on this platform
                    unsafe { winapi::Sleep(1); }
                }
            }
        }
    }
}
