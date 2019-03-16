#![warn(missing_docs)]

//! Helper extensions of standard containers that allow memcopy-less operation.

mod boxed;
mod vec;

pub use boxed::{BoxAllocation, BoxHelper};
pub use vec::{VecAllocation, VecEntry, VecHelper};
