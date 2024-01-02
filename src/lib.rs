#![allow(unsafe_code)]

use mimalloc_sys as ffi;

use std::alloc::{GlobalAlloc, Layout};

pub struct Mimalloc;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Process information (time and memory usage)
pub struct ProcessInfo {
    /// Elapsed wall-clock time of the process in milli-seconds.
    pub elapsed_msecs: usize,
    /// User time in milli-seconds (as the sum over all threads).
    pub user_msecs: usize,
    /// System time in milli-seconds.
    pub system_msecs: usize,
    /// Current working set size (touched pages).
    pub current_rss: usize,
    /// Peak working set size (touched pages).
    pub peak_rss: usize,
    /// Current committed memory (backed by the page file).
    pub current_commit: usize,
    /// Peak committed memory (backed by the page file).
    pub peak_commit: usize,
    /// Count of hard page faults.
    pub page_faults: usize,
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
    /// Return process information (time and memory usage).
    ///
    /// The `current_rss` is precise on Windows and MacOSX; other systems estimate this using current_commit.
    ///
    /// The commit is precise on Windows but estimated on other systems as the amount of read/write accessible memory reserved by mimalloc.
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockalloc::Mockalloc;

    #[global_allocator]
    static GLOBAL: Mockalloc<Mimalloc> = Mockalloc(Mimalloc);

    #[mockalloc::test]
    fn test_alloc() {
        let mut a = Vec::new();
        let mut b = Vec::new();

        for i in 0..1_000_000 {
            let p = Box::new(i);
            if i % 2 == 0 {
                a.push(p);
            } else {
                b.push(p);
            }
        }
    }
}
