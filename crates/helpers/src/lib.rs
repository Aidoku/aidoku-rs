#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![feature(pattern)]
extern crate alloc;
pub mod substring;
pub mod uri;

#[cfg_attr(not(test), cfg(feature = "cloudflare"))]
pub mod cfemail;
