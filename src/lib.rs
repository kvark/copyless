#![warn(missing_docs)]
#![no_std]

//! Helper extensions of standard containers that allow memcopy-less operation.

extern crate alloc;

pub use self::{
    boxed::{BoxAllocation, BoxHelper},
    vec::{VecAllocation, VecEntry, VecHelper},
};

mod boxed;
mod vec;
