use core::ops::Range;

/// A slice-like object.
///
/// This can be implemented for a real slice or a slice-like object that behaves like a slice.
pub trait SliceLike {
    /// A type for the items contained in the slice.
    type Item;

    /// Returns the length of the slice.
    fn len(&self) -> usize;

    /// Returns whether the slice is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Indexes the slice.
    fn index(&self, range: Range<usize>) -> Self;

    /// Copies to another slice.
    fn copy_to_slice(&self, output: &mut [Self::Item]);
}

impl<T: Copy> SliceLike for &'_ [T] {
    type Item = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn index(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    fn copy_to_slice(&self, output: &mut [T]) {
        output.copy_from_slice(self);
    }
}
