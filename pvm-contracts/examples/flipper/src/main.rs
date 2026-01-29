//! Flipper - A simple example contract
//!
//! This contract stores a boolean value that can be flipped.

#![no_std]
#![no_main]

extern crate alloc;

// Storage struct
#[pvm::storage]
pub struct Flipper {
    value: bool,
}

// Constructor
#[pvm::init]
pub fn new() -> Flipper {
    Flipper { value: false }
}

// Flip the value
#[pvm::call]
pub fn flip(state: &mut Flipper) {
    state.value = !state.value;
}

// Get the current value
#[pvm::call]
pub fn get(state: &Flipper) -> bool {
    state.value
}

// Event for when value is flipped
#[pvm::event]
pub struct Flipped {
    new_value: bool,
}

// ============================================================================
// Contract entry points
// ============================================================================

#[global_allocator]
static ALLOCATOR: pvm::alloc_impl::PvmAllocator = pvm::alloc_impl::PvmAllocator;

// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("unimp");
        core::hint::unreachable_unchecked()
    }

    #[cfg(not(target_arch = "riscv64"))]
    loop {}
}

/// Call entry point - dispatches to the appropriate function
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
    pvm::alloc_impl::reset();

    let data = pvm::input();
    if data.len() < 4 {
        pvm::revert(b"no selector");
    }

    let selector: [u8; 4] = [data[0], data[1], data[2], data[3]];

    match selector {
        __PVM_SELECTOR_FLIP => __pvm_call_flip(),
        __PVM_SELECTOR_GET => __pvm_call_get(),
        _ => pvm::revert(b"unknown selector"),
    }
}

/// Deploy entry point - initializes the contract
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {
    pvm::alloc_impl::reset();
    __pvm_init_new();
}
