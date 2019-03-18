use std::{alloc, mem, ptr};


/// A typesafe helper that stores the allocated pointer without the data initialized.
pub struct BoxAllocation<T>(*mut T);

impl<T> BoxAllocation<T> {
    /// Consumes self and writes the given value into the allocation.
    pub fn init(mut self, value: T) -> Box<T> {
        let ptr = mem::replace(&mut self.0, ptr::null_mut());
        unsafe {
            ptr::write(ptr, value);
            Box::from_raw(ptr)
        }
    }
}

impl<T> Drop for BoxAllocation<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let layout = alloc::Layout::new::<T>();
            unsafe {
                alloc::dealloc(self.0 as *mut u8, layout);
            }
        }
    }
}


/// Helper trait for a `Box` type that allocates up-front.
pub trait BoxHelper<T> {
    /// Allocates the storage without providing any data.
    fn alloc() -> BoxAllocation<T>;
}

impl<T> BoxHelper<T> for Box<T> {
    fn alloc() -> BoxAllocation<T> {
        let layout = alloc::Layout::new::<T>();
        BoxAllocation(unsafe {
            alloc::alloc(layout) as *mut T
        })
    }
}
