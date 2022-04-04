#![no_std]
#![feature(core_intrinsics, alloc_error_handler)]

// Setup allocator

#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;

#[cfg_attr(feature = "wee_alloc", global_allocator)]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set panic handlers

#[cfg_attr(not(test), panic_handler)]
pub unsafe fn panic_handle(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub unsafe fn alloc_error_handle(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort()
}

// Make things public

mod structs;

pub use structs::*;

pub use aidoku_imports::error;

pub mod std {
    extern crate alloc;
    pub use aidoku_imports::*;
    pub use alloc::vec::Vec;
}

pub mod prelude {
    pub use aidoku_codegen::*;
}
