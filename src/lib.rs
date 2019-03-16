#![warn(missing_docs)]

//! Helper extensions of standard containers that allow memcopy-less operation.

mod vec;

pub use vec::{VecAllocation, VecEntry, VecHelper};
