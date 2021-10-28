//! Quick sort. O(n*log n) in the best and the average case. In the worst case - Θ(n^2).
//!
//! Quick sort algorithm is "in-place" algorithm which uses a partitioning procedure under the hood to perform sorting.
//! It should be mentioned, that the algorithm is quite effective, despite having a worst case complexity same as insertion sort.
//! That is because the average case doesn't have significant difference from the best case and often we face the average.
//!
//! Partitioning is the procedure that divides "in-place" the input into 2 areas. Elements in the first area are less then elements in the last area.
//! What's more, we recursively perform partitioning on each these 2 areas. So in the end we have guarantee that there is a sequence of areas of length
//! *N* which varies from 1 to input.length/2 and elements of the left sequence are smaller than elements of the right sequence.
//!
//! Obviously, recursion tree has O(log n) height. Each height requires O(n) computations - O(n) * O(log n) = O(n * log n).

/// Partitioner providing different types of partitioning.
///
/// The core of the quick sort is partitioning. We can implement different partitioning algorithms, which should follow the idea stated in the module [doc](index.html).
#[derive(Clone, Copy)]
pub enum Partitioner {
    /// Partitioning algorithm provided by Nico Lomuto.
    ///
    /// The main idea is that we choose the pivot value (which in current implementation is the last one in the array) and
    /// compare all the elements of the array with the pivot. If the comparing value is less then the pivot, it goes to the left
    /// subarray. Otherwise, it goes to the right subarray if it is larger than the pivot. All mutations are performed "in place".
    /// In the end, we place the pivot on such position that each of `(0..pivot)` is less than each of `(pivot+1..)`.
    /// Then we return to the basic sorting procedure the index of the pivot, which is next used to divide the array to subarrays in order to
    /// perform partitioning on them.
    Lomuto,
    /// Tony Hoare's partitioning algorithm.
    ///
    /// The idea is the same, but has one significant difference with Nico Lomuto's version: we do not monitor pivots value. We actually don't care about
    /// placing it to some position. The main thing is to form 2 subarrays where values of the left one are less than values of the right one. However, Hoare's
    /// algorithm returns the index of the first element of the second array.
    Hoare,
}

// todo docs
pub fn quick_sort<T: PartialOrd + Clone>(src: &mut [T], partitioner: Partitioner) {
    match src.len() {
        0 | 1 => {}
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1)
            }
        }
        _ => quick_sort_impl(src, partitioner),
    }
}

fn quick_sort_impl<T: PartialOrd + Clone>(src: &mut [T], partitioner: Partitioner) {
    let q = partitioner.run(src);
    quick_sort(&mut src[..q], partitioner);
    quick_sort(&mut src[q + 1..], partitioner);
}

impl Partitioner {
    pub fn run<T: PartialOrd + Clone>(self, src: &mut [T]) -> usize {
        match self {
            Partitioner::Lomuto => lomuto_partitioning(src),
            Partitioner::Hoare => todo!(),
        }
    }
}

/// Basically, it `more_than_pivot_start` is the index of the first element if the right array.
/// It means that last element of the left array is at index `more_than_pivot_start - 1`.
/// When less (or equal) than pivot element is found we place it to the end of the left array.
///
/// A closer to CLRS version:
/// ```rust
/// fn lomuto_partitioning<T: PartialOrd + Clone>(src: &mut [T]) -> usize {
///     let pivot_idx = src.len() - 1;
///     let mut less_than_pivot_offset_opt = None;
///     for more_than_pivot_offset in 0..pivot_idx {
///         if src[more_than_pivot_offset] <= src[pivot_idx] {
///             less_than_pivot_offset_opt = less_than_pivot_offset_opt.map(increment).or(Some(0));
///             src.swap(less_than_pivot_offset_opt.expect("less than pivot array is founded"), more_than_pivot_offset)
///         }
///     }
///     let pivot_proper_idx = less_than_pivot_offset_opt.map_or(0, increment);
///     src.swap(pivot_idx, pivot_proper_idx);
///     pivot_proper_idx
/// }
///
/// #[inline]
/// fn increment(num: usize) -> usize {
///     num + 1
/// }
/// ```
fn lomuto_partitioning<T: PartialOrd + Clone>(src: &mut [T]) -> usize {
    let pivot_idx = src.len() - 1;
    let mut more_than_pivot_start = 0;
    for more_than_pivot_end in 0..pivot_idx {
        if src[more_than_pivot_end] <= src[pivot_idx] {
            // increasing left area array by one and placing to it's end found element
            more_than_pivot_start += 1;
            src.swap(more_than_pivot_start - 1, more_than_pivot_end)
        }
    }
    let proper_pivot_idx = more_than_pivot_start;
    src.swap(pivot_idx, proper_pivot_idx);
    proper_pivot_idx
}

#[test]
fn quick_sort_test() {
    use crate::test_utils::get_test_vectors;

    for (input, sorted) in get_test_vectors().iter_mut() {
        quick_sort(input, Partitioner::Lomuto);
        assert_eq!(input, sorted)
    }
}
