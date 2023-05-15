use core::mem::MaybeUninit;

use vc6_sys as winapi;

pub struct CriticalSection {
    crit: MaybeUninit<winapi::CRITICAL_SECTION>,
    once: crate::once::Once,
}

impl CriticalSection {
    pub const fn new() -> Self {
        CriticalSection {
            crit: MaybeUninit::uninit(),
            once: crate::once::Once::new(),
        }
    }

    /// Enters a critical section and return a guard object which
    /// automatically leaves the critical section on `Drop`
    pub fn lock(&'static self) -> SectionGuard {
        let ptr = self.crit.as_ptr() as *mut _;

        self.once.call(|| unsafe {
            winapi::InitializeCriticalSection(ptr);
        });

        unsafe { self.enter(); }
        SectionGuard { ptr }
    }

    /// Enters a critical section. This is a low-level API and should be
    /// avoided in favour of `lock` if possible.
    ///
    /// It is up to the user to ensure this call is always paired with a
    /// `leave` call. `enter` is not memory-unsafe itself, but it is marked
    /// unsafe for parity with `leave`.
    pub unsafe fn enter(&'static self) {
        let ptr = self.crit.as_ptr() as *mut _;
        unsafe { winapi::EnterCriticalSection(ptr); }
    }

    /// Leaves a critical section. Must only be called while currently in a
    /// critical section entered with `enter`.
    pub unsafe fn leave(&'static self) {
        let ptr = self.crit.as_ptr() as *mut _;
        unsafe { winapi::LeaveCriticalSection(ptr); }
    }
}

pub struct SectionGuard {
    ptr: *mut winapi::CRITICAL_SECTION,
}

impl Drop for SectionGuard {
    fn drop(&mut self) {
        unsafe { winapi::LeaveCriticalSection(self.ptr); }
    }
}
