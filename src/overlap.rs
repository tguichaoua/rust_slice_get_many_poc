use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/* -------------------------------------------------------------------------- */

/// An operation that check if two indices or range of indices overlap each other
/// i.e. if they may borrow the same item from a slice.
///
/// # Safety
///
/// The implementation must be correct.
pub unsafe trait IndexOverlap<T> {
    fn overlap(&self, other: &T) -> bool;
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &RangeFull) -> bool {
        true
    }
}

unsafe impl IndexOverlap<usize> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &usize) -> bool {
        true
    }
}

unsafe impl IndexOverlap<Range<usize>> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &Range<usize>) -> bool {
        true
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &RangeInclusive<usize>) -> bool {
        true
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &RangeFrom<usize>) -> bool {
        true
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &RangeTo<usize>) -> bool {
        true
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for RangeFull {
    #[inline]
    fn overlap(&self, _other: &RangeToInclusive<usize>) -> bool {
        true
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for usize {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for usize {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        self == other
    }
}

unsafe impl IndexOverlap<Range<usize>> for usize {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        other.contains(self)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for usize {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        other.contains(self)
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for usize {
    #[inline]
    fn overlap(&self, other: &RangeFrom<usize>) -> bool {
        other.contains(self)
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for usize {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        other.contains(self)
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for usize {
    #[inline]
    fn overlap(&self, other: &RangeToInclusive<usize>) -> bool {
        other.contains(self)
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<Range<usize>> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        self.contains(&other.start) || other.contains(&self.start)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        self.contains(other.start()) || other.contains(&self.start)
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFrom<usize>) -> bool {
        other.start < self.end
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        other.contains(&self.start)
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for Range<usize> {
    #[inline]
    fn overlap(&self, other: &RangeToInclusive<usize>) -> bool {
        other.contains(&self.start)
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<Range<usize>> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFrom<usize>) -> bool {
        other.start <= *self.end()
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        other.contains(self.start())
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for RangeInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeToInclusive<usize>) -> bool {
        other.contains(self.start())
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<Range<usize>> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, _other: &RangeFrom<usize>) -> bool {
        true
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        self.start < other.end
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for RangeFrom<usize> {
    #[inline]
    fn overlap(&self, other: &RangeToInclusive<usize>) -> bool {
        self.start <= other.end
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<Range<usize>> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFrom<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        // If one of the range's `end` is `0`, then it is empty and will borrow no item.
        // If it borrow no item, it cannot overlap.
        self.end != 0 && other.end != 0
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for RangeTo<usize> {
    #[inline]
    fn overlap(&self, _other: &RangeToInclusive<usize>) -> bool {
        // If `RangeTo::end` is `0`, it is empty and borrow no item.
        self.end != 0
    }
}

/* -------------------------------------------------------------------------- */

unsafe impl IndexOverlap<RangeFull> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFull) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<usize> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &usize) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<Range<usize>> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &Range<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeInclusive<usize>> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeInclusive<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeFrom<usize>> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeFrom<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeTo<usize>> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, other: &RangeTo<usize>) -> bool {
        // NOTE: reuse the implementation above.
        other.overlap(self)
    }
}

unsafe impl IndexOverlap<RangeToInclusive<usize>> for RangeToInclusive<usize> {
    #[inline]
    fn overlap(&self, _other: &RangeToInclusive<usize>) -> bool {
        true
    }
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    // TODO
}

/* -------------------------------------------------------------------------- */
