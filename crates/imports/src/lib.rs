#![no_std]
extern crate alloc;

pub mod defaults;
pub mod error;
pub mod html;
pub mod json;
pub mod net;
mod std;
pub use core::fmt::Write;
pub use std::*;
