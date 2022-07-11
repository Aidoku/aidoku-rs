#![doc = include_str!("../../../README.md")]
#![no_std]
#![feature(
    core_intrinsics,
    alloc_error_handler,
    fmt_internals,
    panic_info_message
)]

// Setup allocator

#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;

#[cfg_attr(feature = "wee_alloc", global_allocator)]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set panic handlers

fn as_abort<T: AsRef<str>>(message: T, file: T, line: u32, column: u32) -> ! {
    extern "C" {
        #[link_name = "abort"]
        fn _abort(message: *const u8, file: *const u8, line: i32, column: i32);
    }
    extern crate alloc;
    use alloc::alloc::{alloc_zeroed, dealloc};
    use core::{alloc::Layout, ptr::copy};

    let message = message.as_ref();
    let file = file.as_ref();

    // Basically, AssemblyScript places 4 bytes before the string slice to denote
    // its length. This is why we need the extra 8 bytes.
    let layout =
        Layout::from_size_align(8 + message.len() + file.len(), core::mem::align_of::<u8>())
            .unwrap();

    unsafe {
        let message_len_ptr = alloc_zeroed(layout) as *mut i32;
        *message_len_ptr = i32::try_from(message.len()).unwrap_or(-1);

        let message_ptr = message_len_ptr.add(1) as *mut u8;
        copy::<u8>(message.as_ptr(), message_ptr, message.len());

        let file_len_ptr = message_len_ptr.add(message.len()) as *mut i32;
        *file_len_ptr = i32::try_from(file.len()).unwrap_or(-1);

        let file_ptr = file_len_ptr.add(1) as *mut u8;
        copy::<u8>(file.as_ptr(), file_ptr, file.len());

        _abort(
            message_ptr,
            file_ptr,
            line.try_into().unwrap_or(-1),
            column.try_into().unwrap_or(-1),
        );

        dealloc(message_len_ptr as *mut u8, layout);
        dealloc(message_ptr, layout);
        dealloc(file_len_ptr as *mut u8, layout);
        dealloc(file_ptr, layout);
    }
    core::intrinsics::abort()
}

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handle(info: &core::panic::PanicInfo) -> ! {
    use aidoku_imports::Write;
    let (file, line, col) = if let Some(location) = info.location() {
        (location.file(), location.line(), location.column())
    } else {
        ("", 0, 0)
    };

    let message = if let Some(args) = info.message() {
        let mut string = crate::std::String::with_capacity(args.estimated_capacity());
        string.write_fmt(*args).unwrap_or_default();
        string
    } else {
        crate::std::String::new()
    };

    as_abort(message, crate::std::String::from(file), line, col)
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn alloc_error_handle(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort()
}

// Make things public

mod structs;

pub use structs::*;

/// Error module for Aidoku operations.
pub use aidoku_imports::error;

/// The Aidoku standard module, which includes all functions exported from
/// Aidoku as well as a few common types.
pub mod std {
    extern crate alloc;
    pub use aidoku_imports::*;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
    pub fn format(args: core::fmt::Arguments) -> crate::std::String {
        let mut string = crate::std::String::with_capacity(args.estimated_capacity());
        string.write_fmt(args).expect("error formatting string");
        string
    }
}

/// The Aidoku prelude, which includes [format!](aidoku_macros::format),
/// [println!](aidoku_macros::println), as well as procedural macros which
/// are required for interop with the app.
pub mod prelude {
    pub use aidoku_macros::*;
    pub use aidoku_proc_macros::*;
}

#[cfg(feature = "helpers")]
pub mod helpers {
    pub use aidoku_helpers::*;
}
