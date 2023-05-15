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

    pub fn enter(&'static self) -> SectionGuard {
        let ptr = self.crit.as_ptr() as *mut _;

        self.once.call(|| unsafe {
            winapi::InitializeCriticalSection(ptr);
        });

        unsafe { winapi::EnterCriticalSection(ptr); }
        SectionGuard { ptr }
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
