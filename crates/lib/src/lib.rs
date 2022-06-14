#![no_std]
#![feature(core_intrinsics, alloc_error_handler, fmt_internals)]

// Setup allocator

#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;

#[cfg_attr(feature = "wee_alloc", global_allocator)]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set panic handlers

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handle(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn alloc_error_handle(_: core::alloc::Layout) -> ! {
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
    pub use alloc::string::String;
    pub fn format(args: core::fmt::Arguments) -> crate::std::String {
        let mut string = crate::std::String::with_capacity(args.estimated_capacity());
        string.write_fmt(args).expect("error formatting string");
        string
    }
}

pub mod prelude {
    pub use aidoku_macros::*;
    pub use aidoku_proc_macros::*;
}
