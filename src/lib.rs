use std::ptr;

//TODO: box helper

/// A typesafe helper that separates new value construction from
/// vector growing, allowing LLVM to ideally construct the element in place.
pub struct Allocation<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> Allocation<'a, T> {
    // writing is safe because alloc() ensured enough capacity
    // and `Allocation` holds a mutable borrow to prevent anyone else
    // from breaking this invariant.
    #[inline(always)]
    pub fn init(self, value: T) -> usize {
        unsafe {
            ptr::write(self.vec.as_mut_ptr().add(self.index), value);
            self.vec.set_len(self.index + 1);
        }
        self.index
    }
}

/// An entry into a vector, similar to `std::collections::hash_map::Entry`.
pub enum VecEntry<'a, T: 'a> {
    Vacant(Allocation<'a, T>),
    Occupied(&'a mut T),
}

impl<'a, T> VecEntry<'a, T> {
    #[inline(always)]
    pub fn set(self, value: T) {
        match self {
            VecEntry::Vacant(alloc) => { alloc.init(value); }
            VecEntry::Occupied(slot) => { *slot = value; }
        }
    }
}

/// Helper trait for a `Vec` type that allocates up-front.
pub trait VecHelper<T> {
    /// Growns the vector by a single entry, returning the allocation.
    fn alloc(&mut self) -> Allocation<T>;
    /// Either returns an existing elemenet, or grows the vector by one.
    /// Doesn't expect indices to be higher than the current length.
    fn entry(&mut self, index: usize) -> VecEntry<T>;
}

impl<T> VecHelper<T> for Vec<T> {
    fn alloc(&mut self) -> Allocation<T> {
        let index = self.len();
        if self.capacity() == index {
            self.reserve(1);
        }
        Allocation {
            vec: self,
            index,
        }
    }

    fn entry(&mut self, index: usize) -> VecEntry<T> {
        if index < self.len() {
            VecEntry::Occupied(unsafe {
                self.get_unchecked_mut(index)
            })
        } else {
            assert_eq!(index, self.len());
            VecEntry::Vacant(self.alloc())
        }
    }
}
