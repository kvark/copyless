
use {alloc::vec::Vec, core::ptr};

/// A typesafe helper that separates new value construction from
/// vector growing, allowing LLVM to ideally construct the element in place.
pub struct VecAllocation<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> VecAllocation<'a, T> {
    /// Consumes self and writes the given value into the allocation.
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

pub struct VecArrayAllocation<'a, T: 'a, const N: usize > {
    vec: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T, const N: usize> VecArrayAllocation<'a, T, N> {
    /// Consumes self and writes the given value into the allocation.
    // writing is safe because alloc() ensured enough capacity
    // and `Allocation` holds a mutable borrow to prevent anyone else
    // from breaking this invariant.
    #[inline(always)]
    pub fn init(self, value: [T; N]) -> usize {
        unsafe {
            ptr::write(self.vec.as_mut_ptr().add(self.index) as *mut [T; N], value);
            self.vec.set_len(self.index + N);
        }
        self.index
    }
}

/// An entry into a vector, similar to `std::collections::hash_map::Entry`.
pub enum VecEntry<'a, T: 'a> {
    /// Entry has just been freshly allocated.
    Vacant(VecAllocation<'a, T>),
    /// Existing entry.
    Occupied(&'a mut T),
}

impl<'a, T> VecEntry<'a, T> {
    /// Sets the value for this entry.
    #[inline(always)]
    pub fn set(self, value: T) {
        match self {
            VecEntry::Vacant(alloc) => {
                alloc.init(value);
            }
            VecEntry::Occupied(slot) => {
                *slot = value;
            }
        }
    }
}

/// Helper trait for a `Vec` type that allocates up-front.
pub trait VecHelper<T> {
    /// Grows the vector by a single entry, returning the allocation.
    fn alloc(&mut self) -> VecAllocation<T>;
    /// Either returns an existing element, or grows the vector by one.
    /// Doesn't expect indices to be higher than the current length.
    fn entry(&mut self, index: usize) -> VecEntry<T>;

    fn alloc_multiple<const N: usize>(&mut self) -> VecArrayAllocation<T, N>;
}

impl<T> VecHelper<T> for Vec<T> {
    fn alloc(&mut self) -> VecAllocation<T> {
        let index = self.len();
        if self.capacity() == index {
            self.reserve(1);
        }
        VecAllocation { vec: self, index }
    }

    fn alloc_multiple<const N: usize>(&mut self) -> VecArrayAllocation<T, N> {
        self.reserve(N);
        let index = self.len();
        VecArrayAllocation { vec: self, index }
    }

    fn entry(&mut self, index: usize) -> VecEntry<T> {
        if index < self.len() {
            VecEntry::Occupied(unsafe { self.get_unchecked_mut(index) })
        } else {
            assert_eq!(index, self.len());
            VecEntry::Vacant(self.alloc())
        }
    }
}

/// Tests are only meaningful in Release
#[cfg(not(debug_assertions))]
#[test]
fn test_zero_copy() {
    union Foo {
        small: u8,
        big: [f32; 10],
    }
    let mut vec = vec![Foo { big: [1.0; 10] }];

    // check that the new helper is not overwriting
    vec.remove(0);
    vec.alloc().init(Foo { small: 5 });
    assert_eq!(unsafe { vec[0].big[1] }, 1.0);

    // check that the regular push is overwriting
    vec.remove(0);
    vec.push(Foo { small: 5 });
    //TODO: make the expected value to be concrete
    assert_ne!(unsafe { vec[0].big[1] }, 1.0);
}

#[test]
fn test_add_multiple() {
    use alloc::vec;
    let mut vec = vec![1, 2, 3];

    // check that the new helper is not overwriting
    vec.remove(0);
    vec.alloc_multiple().init([4, 5]);
    assert_eq!(vec, [2, 3, 4, 5])

}
