#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![deny(missing_debug_implementations)]
#![feature(pattern, let_chains)]
extern crate alloc;

pub mod node;
pub mod substring;
pub mod uri;
pub mod cfemail;
