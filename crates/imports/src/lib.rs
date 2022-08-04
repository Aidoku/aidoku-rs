#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_debug_implementations)]
#![feature(iter_advance_by)]
extern crate alloc;

pub mod defaults;
pub mod error;
pub mod html;
pub mod json;
pub mod net;
mod std;
pub use core::fmt::Write;
pub use std::*;
