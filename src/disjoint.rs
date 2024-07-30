use std::ops::{self, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::overlap::IndexOverlap;
use crate::SliceManyIndices;

/* -------------------------------------------------------------------------- */

/// Asserts that the indices in `I` are all disjoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisjointIndices<I>(I);

impl<I> DisjointIndices<I> {
    /// # Safety
    ///
    /// All indices in `I` must be disjoint (i.e. a single item may not be borrowed twice).
    pub unsafe fn new_unchecked(indices: I) -> Self {
        Self(indices)
    }

    pub fn get(self) -> I {
        self.0
    }
}

impl<I: ManyIndices> DisjointIndices<I> {
    pub fn new(indices: I) -> Option<Self> {
        if indices.is_disjoint() {
            // SAFETY: `indices` are disjoint.
            Some(unsafe { Self::new_unchecked(indices) })
        } else {
            None
        }
    }
}

/* -------------------------------------------------------------------------- */

/// Represents multiple indices for a slice that may borrow the same item multiple times.
///
/// # Safety
///
/// - `is_disjoint` must returns `true` only if the indices are disjoint (i.e. a single item may not be borrowed twice).
/// - `is_in_bounds` must returns `true` only if all the indices are in bounds.
/// - `is_disjoint_and_in_bounds` must be equivalent to `is_disjoint() && is_in_bounds()`.
pub unsafe trait ManyIndices {
    /// Returns `true` if all indices are disjoint (i.e. a single item may not be borrowed twice).
    fn is_disjoint(&self) -> bool;

    /// Returns `true` if all indices are in `bounds`.
    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool;

    /// Returns `true` if both the indices are disjoint and in bounds.
    ///
    /// The default implementation is `is_disjoint() && is_in_bounds()` but can be
    /// re-implemented for a more efficient way.
    fn is_disjoint_and_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.is_disjoint() && self.is_in_bounds(bounds)
    }
}

unsafe impl<I> ManyIndices for DisjointIndices<I>
where
    I: ManyIndices,
{
    fn is_disjoint(&self) -> bool {
        // NOTE: `DisjointIndices` asserts that the indices are disjoint.
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.0.is_in_bounds(bounds)
    }
}

unsafe impl ManyIndices for usize {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        bounds.contains(self)
    }
}

unsafe impl ManyIndices for Range<usize> {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.start < bounds.end && self.end <= bounds.end
    }
}

unsafe impl ManyIndices for RangeInclusive<usize> {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        *self.start() < bounds.end && *self.end() < bounds.end
    }
}

unsafe impl ManyIndices for RangeTo<usize> {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.end <= bounds.end
    }
}

unsafe impl ManyIndices for RangeToInclusive<usize> {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.end < bounds.end
    }
}

unsafe impl ManyIndices for RangeFrom<usize> {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.start < bounds.end
    }
}

unsafe impl ManyIndices for RangeFull {
    fn is_disjoint(&self) -> bool {
        true
    }

    fn is_in_bounds(&self, _bounds: ops::RangeTo<usize>) -> bool {
        true
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl<I, const N: usize> ManyIndices for [I; N]
where
    I: ManyIndices + IndexOverlap<I>,
{
    fn is_disjoint(&self) -> bool {
        for (i, idx) in self.iter().enumerate() {
            for idx2 in &self[..i] {
                if idx.overlap(idx2) {
                    return false;
                }
            }
        }

        true
    }

    fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        self.iter().all(|i| i.is_in_bounds(bounds))
    }

    fn is_disjoint_and_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
        for (i, idx) in self.iter().enumerate() {
            if !idx.is_in_bounds(bounds) {
                return false;
            }

            for idx2 in &self[..i] {
                if idx.overlap(idx2) {
                    return false;
                }
            }
        }

        true
    }
}

/* -------------------------------------------------------------------------- */

impl<'slice, Slice: ?Sized, I> SliceManyIndices<'slice, Slice> for DisjointIndices<I>
where
    I: SliceManyIndices<'slice, Slice>,
{
    type Output = I::Output;

    unsafe fn get_many_unchecked_mut(self, slice: &'slice mut Slice) -> Self::Output {
        // SAFETY: contract uphold by the caller.
        unsafe { SliceManyIndices::get_many_unchecked_mut(self.0, slice) }
    }
}

/* -------------------------------------------------------------------------- */

macro_rules! impl_tuple {

    ( $($I:ident)+ ) => {
        impl_tuple!(@recursive $($I)*);
    };

    (@recursive $first:ident $($next:ident)* ) => {
        impl_tuple!(@prepare [] $first $($next)*);
        impl_tuple!(@recursive $($next)*);
    };

    (@recursive ) => { /* stop condition */ };


    (@prepare [$($prepared:tt)*] $current:ident $($next:ident)*  ) => {
        impl_tuple!{
            @prepare
                [
                    $($prepared)*
                    $current => [$($next)*]
                ]

                $( $next )*
        }
    };

    // Stop condition
    (@prepare [$($prepared:tt)*] ) => {
        impl_tuple!(@impl $($prepared)*);
    };

    (@impl
        $( $I:ident => [$($INext:ident)*] )*
    ) => {
        #[allow(non_snake_case)]
        unsafe impl<$( $I, )*> ManyIndices for ($( $I, )*)
        where
            $( $I: ManyIndices $( + IndexOverlap<$INext> )* ,)*
        {
            fn is_disjoint(&self) -> bool {
                let ( $($I,)* ) = self;

                true $(
                    && ManyIndices::is_disjoint($I)
                )* $($(
                    && !IndexOverlap::overlap($I, $INext)
                )*)*
            }

            fn is_in_bounds(&self, bounds: ops::RangeTo<usize>) -> bool {
                let ( $($I,)* ) = self;

                true $(
                    && ManyIndices::is_in_bounds($I, bounds)
                )*
            }
        }
    };
}

impl_tuple!(I0 I1 I2 I3 I4 I5 I6 I7 I8 I9 I10 I11);

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use crate::DisjointIndices;

    use super::ManyIndices;

    #[test]
    fn disjoint_indices() {
        assert!(DisjointIndices::new([0, 5, 9]).is_some());
        assert!(DisjointIndices::new([0..4, 4..6]).is_some());
        assert!(DisjointIndices::new([0..=2, 3..=5]).is_some());
        assert!(DisjointIndices::new([..]).is_some());
        assert!(DisjointIndices::new([.., ..]).is_none());
        assert!(DisjointIndices::new([2..5, 4..8]).is_none());
        assert!(DisjointIndices::new([2..=5, 5..=8]).is_none());

        assert!(DisjointIndices::new((0, 5, 9)).is_some());
        assert!(DisjointIndices::new((4..8, 8..10)).is_some());
        assert!(DisjointIndices::new((5, ..9)).is_none());
        assert!(DisjointIndices::new((10.., 2..10)).is_some());
    }

    #[test]
    fn array_index() {
        let array = [2, 7, 9];

        assert!(array.is_disjoint());
        assert!(!array.is_in_bounds(..9));
        assert!(array.is_in_bounds(..10));
        assert!(!array.is_disjoint_and_in_bounds(..9));
        assert!(array.is_disjoint_and_in_bounds(..10));
    }

    #[test]
    fn array_index_overlap() {
        let array = [2, 9, 9];

        assert!(!array.is_disjoint());
        assert!(!array.is_in_bounds(..9));
        assert!(array.is_in_bounds(..10));
        assert!(!array.is_disjoint_and_in_bounds(..9));
        assert!(!array.is_disjoint_and_in_bounds(..10));
    }

    #[test]
    fn array_range() {
        let array = [2..5, 7..9, 9..10];

        assert!(array.is_disjoint());
        assert!(!array.is_in_bounds(..9));
        assert!(array.is_in_bounds(..10));
        assert!(!array.is_disjoint_and_in_bounds(..9));
        assert!(array.is_disjoint_and_in_bounds(..10));
    }

    #[test]
    fn array_range_overlap() {
        let array = [2..8, 7..9, 9..10];

        assert!(!array.is_disjoint());
        assert!(!array.is_in_bounds(..9));
        assert!(array.is_in_bounds(..10));
        assert!(!array.is_disjoint_and_in_bounds(..9));
        assert!(!array.is_disjoint_and_in_bounds(..10));
    }

    #[test]
    fn tuple() {
        let tuple = (2, 5, 7..9);

        assert!(tuple.is_disjoint());
        assert!(!tuple.is_in_bounds(..8));
        assert!(tuple.is_in_bounds(..9));
        assert!(!tuple.is_disjoint_and_in_bounds(..8));
        assert!(tuple.is_disjoint_and_in_bounds(..9));
    }

    #[test]
    fn tuple_range() {
        let tuple = (2..5, 5..7, 7..=12);

        assert!(tuple.is_disjoint());
        assert!(!tuple.is_in_bounds(..12));
        assert!(tuple.is_in_bounds(..13));
        assert!(!tuple.is_disjoint_and_in_bounds(..12));
        assert!(tuple.is_disjoint_and_in_bounds(..13));
    }

    #[test]
    fn tuple_index_overlap() {
        let tuple = (2, 2, 7..=12);

        assert!(!tuple.is_disjoint());
        assert!(!tuple.is_in_bounds(..12));
        assert!(tuple.is_in_bounds(..13));
        assert!(!tuple.is_disjoint_and_in_bounds(..12));
        assert!(!tuple.is_disjoint_and_in_bounds(..13));
    }

    #[test]
    fn tuple_range_overlap() {
        let tuple = (2..=7, 7..12);

        assert!(!tuple.is_disjoint());
        assert!(!tuple.is_in_bounds(..11));
        assert!(tuple.is_in_bounds(..12));
        assert!(!tuple.is_disjoint_and_in_bounds(..12));
        assert!(!tuple.is_disjoint_and_in_bounds(..13));
    }
}

/* -------------------------------------------------------------------------- */
