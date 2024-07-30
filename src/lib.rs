#![feature(slice_ptr_get)]

mod disjoint;
mod overlap;
mod slice_index;

pub use disjoint::{DisjointIndices, ManyIndices};
pub use overlap::IndexOverlap;
pub use slice_index::SliceManyIndices;

/* -------------------------------------------------------------------------- */

/// An extension trait to add the `get_many_mut` methods to slice.
pub trait SliceExt {
    /// Returns mutable references to many indices at once,
    /// or [`None`] if `indices` is not disjoint or not in bounds.
    fn get_many_mut_poc<'slice, I>(&'slice mut self, indices: I) -> Option<I::Output>
    where
        I: ManyIndices + SliceManyIndices<'slice, Self>;

    /// Returns mutable references to many indices at once, without doing any checks.
    ///
    /// For a safe alternative see [`get_many_mut_poc`].
    ///
    /// # Safety
    ///
    /// Calling this method with overlapping or out-of-bounds indices is *undefined behavior*
    /// even if the resulting references are not used.
    unsafe fn get_many_mut_unchecked_poc<'slice, I>(&'slice mut self, indices: I) -> I::Output
    where
        I: SliceManyIndices<'slice, Self>;
}

impl<T> SliceExt for [T] {
    fn get_many_mut_poc<'slice, I>(&'slice mut self, indices: I) -> Option<I::Output>
    where
        I: ManyIndices + SliceManyIndices<'slice, Self>,
    {
        if !indices.is_disjoint_and_in_bounds(..self.len()) {
            return None;
        }

        // SAFETY: we have checked that `indices` is disjoint and in bounds.
        Some(unsafe { SliceManyIndices::get_many_unchecked_mut(indices, self) })
    }

    unsafe fn get_many_mut_unchecked_poc<'slice, I>(&'slice mut self, indices: I) -> I::Output
    where
        I: SliceManyIndices<'slice, Self>,
    {
        // SAFETY: the caller ensures that `indices` is disjoint and in bounds.
        unsafe { SliceManyIndices::get_many_unchecked_mut(indices, self) }
    }
}

impl SliceExt for str {
    fn get_many_mut_poc<'slice, I>(&'slice mut self, indices: I) -> Option<I::Output>
    where
        I: ManyIndices + SliceManyIndices<'slice, Self>,
    {
        if !indices.is_disjoint_and_in_bounds(..self.len()) {
            return None;
        }

        // SAFETY: we have checked that `indices` is disjoint and in bounds.
        Some(unsafe { SliceManyIndices::get_many_unchecked_mut(indices, self) })
    }

    unsafe fn get_many_mut_unchecked_poc<'slice, I>(&'slice mut self, indices: I) -> I::Output
    where
        I: SliceManyIndices<'slice, Self>,
    {
        // SAFETY: the caller ensures that `indices` is disjoint and in bounds.
        unsafe { SliceManyIndices::get_many_unchecked_mut(indices, self) }
    }
}

/* -------------------------------------------------------------------------- */
