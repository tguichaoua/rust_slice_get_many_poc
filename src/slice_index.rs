/* -------------------------------------------------------------------------- */

use std::mem;
use std::slice::SliceIndex;

pub trait SliceManyIndices<'slice, Slice: ?Sized> {
    type Output;

    /// Returns mutable references to many indices at once, without doing any checks.
    ///
    /// # Safety
    ///
    /// Calling this method with overlapping or out-of-bounds indices is *undefined behavior*
    /// even if the resulting references are not used.
    unsafe fn get_many_unchecked_mut(self, slice: &'slice mut Slice) -> Self::Output;
}

/* -------------------------------------------------------------------------- */

impl<'slice, T: 'slice, I, const N: usize> SliceManyIndices<'slice, [T]> for [I; N]
where
    I: SliceIndex<[T], Output: 'slice>,
{
    type Output = [&'slice mut I::Output; N];

    unsafe fn get_many_unchecked_mut(self, slice: &'slice mut [T]) -> Self::Output {
        // NOTE: adapted from [T]::get_many_unchecked_mut, I did not ensures this code is 100% safe or 100% optimal.

        {
            let slice: *mut [T] = slice;
            let mut arr: mem::MaybeUninit<[&'slice mut I::Output; N]> = mem::MaybeUninit::uninit();
            let arr_ptr = arr.as_mut_ptr();

            // SAFETY: Caller ensure the indices are all in bounds and didn't overlap.
            unsafe {
                for (i, idx) in self.into_iter().enumerate() {
                    *(*arr_ptr).get_unchecked_mut(i) = &mut *slice.get_unchecked_mut(idx);
                }

                arr.assume_init()
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

macro_rules! impl_tuple {
    ( $($I:ident)+ ) => {
        #[allow(non_snake_case)]
        impl<'slice, T, $($I,)*> SliceManyIndices<'slice, [T]> for ($($I,)*)
        where
            $( $I: SliceIndex<[T], Output: 'slice>, )*
        {
            type Output = ( $( &'slice mut $I::Output, )* );

            unsafe fn get_many_unchecked_mut(self, slice: &'slice mut [T]) -> Self::Output {
                // NOTE: adapted from [T]::get_many_unchecked_mut, I did not ensures this code is 100% safe or 100% optimal.

                {
                    let slice: *mut [T] = slice;
                    let ( $( $I, )* ) = self;

                    // SAFETY: We expect the indices to contain disjunct values that are
                    // in bounds of `slice`.
                    unsafe {
                        (
                            $(
                                &mut *slice.get_unchecked_mut($I),
                            )*
                        )
                    }
                }
            }
        }

        impl_tuple!(@recursive $($I)*);
    };

    () => {/* stop condition */};

    (@recursive $_:ident $($I:ident)*) => {
        impl_tuple!($($I)*);
    };
}

impl_tuple!(I0 I1 I2 I3 I4 I5 I6 I7 I8 I9 I10 I11);

/* -------------------------------------------------------------------------- */
