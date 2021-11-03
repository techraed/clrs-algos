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

use std::cmp::Ordering;
use std::fmt::Debug;

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

/// Quick sort algorithm implementation
///
/// Parametrized by `partitioner` function. Actually `partitioner` is an enum, but under the hood runs
/// one of partitioning algorithms.
pub fn quick_sort<T: PartialOrd + Clone+Debug>(src: &mut [T], partitioner: Partitioner) {
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

fn quick_sort_impl<T: PartialOrd + Clone+Debug>(src: &mut [T], partitioner: Partitioner) {
    let (end_left, start_right) = partitioner.run(src);
    println!("{:?}", src);
    quick_sort(&mut src[..end_left], partitioner);
    quick_sort(&mut src[start_right..], partitioner);
}

impl Partitioner {
    /// Performs partitioning algorithm on `src`.
    ///
    /// Returns ending index for the left array and starting index for the right array respectively.
    ///
    /// # Note
    /// The value under ending index of the left array isn't a part of the left `slice`. For example,
    /// ```rust
    /// # let mut src = [1,2,3];
    /// # let end_left = 1;
    /// let left_array = &mut src[..end_left];
    /// ```
    pub fn run<T: PartialOrd + Clone + Debug>(self, src: &mut [T]) -> (usize, usize) {
        match self {
            Partitioner::Lomuto => {
                let q = lomuto_partitioning(src);
                (q, q + 1)
            }
            Partitioner::Hoare => {
                let q = hoare_partitioning(src);
                (q + 1, q + 1)
            }
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
    let mut greater_than_pivot_start = 0;
    for greater_than_pivot_end in 0..pivot_idx {
        if src[greater_than_pivot_end] <= src[pivot_idx] {
            // increasing left area array by one and placing to it's end found element
            greater_than_pivot_start += 1;
            src.swap(greater_than_pivot_start - 1, greater_than_pivot_end)
        }
    }
    let proper_pivot_idx = greater_than_pivot_start;
    src.swap(pivot_idx, proper_pivot_idx);
    proper_pivot_idx
}

///
/// A closer to CLRS implementation
/// ```rust
/// fn hoare_partitioning<T: PartialOrd + Clone>(src: &mut [T]) -> usize {
///     let pivot_value = src[0].clone();
///     let mut less_than_pivot_end_opt = None;
///     let mut greater_than_pivot_start = src.len();
///     loop {
///         'g: loop {
///             greater_than_pivot_start -= 1;
///             if src[greater_than_pivot_start] <= pivot_value {
///                 break 'g;
///             }
///         }
///         'l: loop {
///             less_than_pivot_end_opt = less_than_pivot_end_opt.map(increment).or(Some(0));
///             if src[less_than_pivot_end_opt.expect("some value is set")] >= pivot_value {
///                 break 'l;
///             }
///         }
/// 
///         if let Some(less_than_pivot_end) = less_than_pivot_end_opt {
///             if less_than_pivot_end < greater_than_pivot_start {
///                 src.swap(less_than_pivot_end, greater_than_pivot_start);
///             } else {
///                 break greater_than_pivot_start;
///             }
///         }
///     }
/// }
/// 
/// #[inline]
/// fn increment(num: usize) -> usize {
///     num + 1
/// }
/// ```
fn hoare_partitioning<T: PartialOrd + Clone + Debug>(src: &mut [T]) -> usize {
    let mut pivot_idx = 0;
    let mut less_than_pivot_end = 0;
    let mut greater_than_pivot_start = src.len();
    loop {
        'g: loop {
            greater_than_pivot_start -= 1;
            if src[greater_than_pivot_start] <= src[pivot_idx] {
                break 'g;
            }
        }
        'l: loop {
            less_than_pivot_end = if less_than_pivot_end == 0 { less_than_pivot_end } else { less_than_pivot_end + 1};
            if src[less_than_pivot_end] >= src[pivot_idx] {
                break 'l;
            }
        }

        match less_than_pivot_end.cmp(&greater_than_pivot_start) {
            Ordering::Less => {
                // if less_than_pivot_end == pivot_idx {
                //     pivot_idx = greater_than_pivot_start;
                // } else if greater_than_pivot_start == pivot_idx {
                //     pivot_idx = less_than_pivot_end;
                // }
                src.swap(less_than_pivot_end, greater_than_pivot_start);
            }
            Ordering::Equal | Ordering::Greater => {
                break greater_than_pivot_start;
            }
        }
    }
}

#[test]
fn quick_sort_test() {
    use crate::test_utils::get_test_vectors;

    for (input, sorted) in get_test_vectors().iter_mut() {
        let mut input2 = input.clone();
        // quick_sort(input, Partitioner::Lomuto);
        quick_sort(&mut input2, Partitioner::Hoare);

        // assert_eq!(input, sorted);
        assert_eq!(&mut input2, sorted);
    }
}
