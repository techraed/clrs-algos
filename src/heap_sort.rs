//! Heap sort. O(n*log n).
//!
//! Notable that this is "in-place" algorithm with a quite effective time complexity.
//! However, to reach this we need to maintain all the data in the [heap[(https://en.wikipedia.org/wiki/Heap_(data_structure)) data structure.

/// Heap implementation using vector as underlying type storing node values
pub struct Heap<T>
where
    T: PartialOrd + Clone,
{
    inner: Vec<T>,
    heap_size: usize,
}
