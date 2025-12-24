#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "helpers")]
pub mod helpers;

#[cfg(feature = "imports")]
pub mod imports;

#[cfg(feature = "imports")]
mod macros;

mod structs;

pub use structs::*;

// talc allocator
#[cfg(target_family = "wasm")]
#[cfg(feature = "talc")]
#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

// panic handler
#[cfg(target_family = "wasm")]
#[cfg(not(feature = "test"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	unsafe extern "C" {
		fn abort();
		fn print(string: *const u8, size: usize);
	}

	let message = prelude::format!("{}", info);
	unsafe {
		// print the error message to aidoku logs
		print(message.as_ptr(), message.len());
		// tell aidoku we're aborting so that the unreachable instruction is not executed
		abort();
	};

	core::arch::wasm32::unreachable()
}

// re-export dependencies for `register_source` macro to access
pub use postcard;
pub use serde;

/// Re-export of `alloc` crate.
pub mod alloc {
	#![allow(hidden_glob_reexports)]
	extern crate alloc;
	pub use alloc::*;

	pub use alloc::boxed::Box;
	pub use alloc::string::String;
	pub use alloc::vec::Vec;
}

/// The prelude macros.
pub mod prelude {
	pub use super::alloc::format;
	#[cfg(feature = "imports")]
	pub use crate::{bail, debug, error, println, register_source};
}
