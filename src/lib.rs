#![allow(unsafe_code)]

use mimalloc_sys as ffi;

use std::alloc::{GlobalAlloc, Layout};

pub struct Mimalloc;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessInfo {
    elapsed_msecs: usize,
    user_msecs: usize,
    system_msecs: usize,
    current_rss: usize,
    peak_rss: usize,
    current_commit: usize,
    peak_commit: usize,
    page_faults: usize,
}

unsafe impl GlobalAlloc for Mimalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::mi_malloc_aligned(layout.size(), layout.align()).cast::<u8>()
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ffi::mi_zalloc_aligned(layout.size(), layout.align()).cast::<u8>()
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ffi::mi_realloc_aligned(ptr.cast::<ffi::c_void>(), new_size, layout.align()).cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::mi_free(ptr as *mut _);
    }
}

impl Mimalloc {
    /// Returns the current process stats.
    pub fn process_info() -> ProcessInfo {
        let mut stats = ProcessInfo::default();

        unsafe {
            ffi::mi_process_info(
                &mut stats.elapsed_msecs,
                &mut stats.user_msecs,
                &mut stats.system_msecs,
                &mut stats.current_rss,
                &mut stats.peak_rss,
                &mut stats.current_commit,
                &mut stats.peak_commit,
                &mut stats.page_faults,
            );
        }

        stats
    }
}
