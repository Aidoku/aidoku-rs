//! Utilities for commonly used functions when creating Aidoku sources.
extern crate alloc;

#[cfg(feature = "imports")]
pub mod cfemail;
#[cfg(feature = "imports")]
pub mod date;
#[cfg(feature = "imports")]
pub mod element;

pub mod uri;
