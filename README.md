# slice `get_many_mut` proof of concept

This is a POC to make [`[T]::get_many_mut`](https://github.com/rust-lang/rust/issues/104642) to accept either an array of index or a tuple of index.

See the [test folder](./tests/main.rs) for example of the API usage.

This POC introduces three new traits:

- [`SliceManyIndices<'slice, Slice: ?Sized>`](./src/slice_index.rs) similar to [`SliceIndex<T>`](https://doc.rust-lang.org/stable/std/slice/trait.SliceIndex.html) which is implemented for types that represent multiple indices (array and tuple) and gets the reference of the items from the slice.
- [`ManyIndices`](./src/disjoint.rs) which is also implemented for types represents multiple indices. This trait provides runtime checking for "does the indices are distinct ?" and "does the indices in bounds ?".
- [`IndexOverlap<T>`](./src/overlap.rs) an operation that check if two index type overlap each other.

And one struct:

- [`DisjointIndices<I>`](./src/disjoint.rs) which assert that the indices of a multiple indices value are disjoint.
