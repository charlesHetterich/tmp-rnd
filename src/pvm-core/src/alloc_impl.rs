//! Bump allocator for PVM contracts
//!
//! Provides a simple bump allocator that allocates from a static buffer.
//! Memory is not freed until the contract call completes.

use core::cell::UnsafeCell;

/// Size of the static allocation buffer (32KB)
const BUFFER_SIZE: usize = 32 * 1024;

/// Static buffer for allocations
static BUFFER: StaticBuffer = StaticBuffer::new();

/// A simple bump allocator using a static buffer
struct StaticBuffer {
    buffer: UnsafeCell<[u8; BUFFER_SIZE]>,
    offset: UnsafeCell<usize>,
}

// Safety: Contracts are single-threaded
unsafe impl Sync for StaticBuffer {}

impl StaticBuffer {
    const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0u8; BUFFER_SIZE]),
            offset: UnsafeCell::new(0),
        }
    }

    fn alloc(&self, size: usize) -> Option<&'static mut [u8]> {
        self.alloc_aligned(size, 1)
    }

    fn alloc_aligned(&self, size: usize, align: usize) -> Option<&'static mut [u8]> {
        unsafe {
            let offset = *self.offset.get();

            // Align the offset
            let aligned_offset = (offset + align - 1) & !(align - 1);
            let new_offset = aligned_offset + size;

            if new_offset > BUFFER_SIZE {
                return None;
            }

            *self.offset.get() = new_offset;

            let buffer = &mut *self.buffer.get();
            Some(&mut buffer[aligned_offset..new_offset])
        }
    }

    fn reset(&self) {
        unsafe {
            *self.offset.get() = 0;
        }
    }
}

/// Allocate memory from the static buffer
pub fn alloc(size: usize) -> Option<&'static mut [u8]> {
    BUFFER.alloc(size)
}

/// Allocate aligned memory from the static buffer
pub fn alloc_aligned(size: usize, align: usize) -> Option<&'static mut [u8]> {
    BUFFER.alloc_aligned(size, align)
}

/// Reset the allocator (call at start of each contract invocation)
pub fn reset() {
    BUFFER.reset();
}

/// Global allocator implementation for contracts
pub struct PvmAllocator;

unsafe impl core::alloc::GlobalAlloc for PvmAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        match alloc_aligned(layout.size(), layout.align()) {
            Some(slice) => slice.as_mut_ptr(),
            None => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Bump allocator doesn't free individual allocations
    }
}

/// Global allocator instance
/// Only defined when:
/// - Building for riscv64 (actual contract)
/// - NOT using std (which provides its own allocator)
/// - NOT using `no-allocator` feature (for custom allocators)
#[cfg(all(target_arch = "riscv64", not(feature = "std"), not(feature = "no-allocator")))]
#[global_allocator]
static PVM_ALLOCATOR: PvmAllocator = PvmAllocator;
