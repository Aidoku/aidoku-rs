//! Wrapper around imported source API functions.

pub mod canvas;
pub mod defaults;
pub mod error;
pub mod html;
pub mod js;
pub mod net;
pub mod std;

/// A standard descriptor, used for data exchange between the runner and the source (reference id).
///
/// Valid descriptors will always be positive.
pub(crate) type Rid = i32;

/// An error code, descriptor, or result value.
///
/// Error codes are negative, while descriptors are positive.
/// A zero value indicates success.
pub(crate) type FFIResult = i32;

/// A dropped pointer to pass back to the source runner.
pub(crate) type Ptr = i32;
