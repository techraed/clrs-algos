//! Count sort. With some conditions has a O(n) time complexity.
//!
//! There is a [theorem](https://stackoverflow.com/questions/61330147/comparison-based-sorting-is-wc-min-time-nlogn-so-what-about-best-average-case)
//! that states that the best time in worst case for comparison algorithms is *Ω(n * log n)*.
//!
//! One of the main features of this algorithm is that to sort the input array you don't have to do any comparisons. The idea is quite simple:
//! - you count number of each element in the input array;
//! - you count number of elements less than some current element;
//! - if you have 3 elements less than the element `x`, then the first `x` entry in the input has a position `input[3]`.
//!
//! Count sort is one of the algorithms working with a O(n) time. Actually, it works for that with some condition on input data.
//! That is if we have 1) *n* elements where each has a value from range of `1..=k`, where `k` is the largest element of the input
//! and 2) *k = O(n)* , then we have *O(n+k) = O(n+n) = O(n)* time complexity.
//!
//! Should be mentioned that it is a [stable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability) sorting algorithm, which is an important feature
//! for some other algorithms that can use count sort, for example, [radix sort](../radix_sort/index.html).

use std::convert::TryInto;

use num::FromPrimitive;

/// Count sort entry function
///
/// If `src` length is greater than 1 and its inner data has a primitive type, **instances** of which are in the range of `usize`, then the sort will be performed.
/// Otherwise no mutations will be made on `src`.
pub fn count_sort<T: FromPrimitive + TryInto<usize> + Ord + Copy>(src: &mut [T]) {
    let min_element = src.iter().min().copied().map(TryInto::<usize>::try_into).map(Result::ok).flatten();
    let max_element = src.iter().max().copied();
    if min_element.is_some() && src.len() > 1 {
        // `src` of length 1 is sorted
        count_sort_impl(src, max_element.expect("there is at least one element in src"))
    }
}

/// Count sort implementation
///
/// Because of conditions stated in the module [doc](index.html) we parametrize function with a number type value.
/// Unfortunately this algorithm requires *O(max_element)* additional space, which is a good example of space - time tradeoff.
/// 
/// A closer to CLRS version:
/// ```rust
/// # use std::convert::TryInto;
///
/// # use num::FromPrimitive;
/// fn count_sort_impl<T: FromPrimitive + TryInto<usize> + Ord + Copy + Default>(src: &mut [T], max_element: T) {
///     let mut sorted: Vec<T> = vec![T::default(); src.len()];
///     let max_element = max_element
///         .try_into()
///         .ok()
///         .expect("this fn is callable for types, that can be converted to usize");
///     let mut keys_count: Vec<usize> = vec![0; max_element + 1];
///     for &key in &src[..] {
///         if let Ok(key) = key.try_into() {
///             keys_count[key] += 1;
///         }
///     }
///     for idx in 0..keys_count.len() {
///         if idx != 0 {
///             keys_count[idx] += keys_count[idx - 1];
///         }
///     }
///     for &key in &src[..] {
///         if let Ok(key_usize) = key.try_into() {
///             sorted[keys_count[key_usize] - 1] = key;
///             keys_count[key_usize] -= 1;
///         }
///     }
///     src.copy_from_slice(&sorted[..])
/// }
/// ```
fn count_sort_impl<T: FromPrimitive + TryInto<usize> + Ord + Copy>(src: &mut [T], max_element: T) {
     let max_element = max_element
        .try_into()
        .ok()
        .expect("this fn is callable for types, that can be converted to usize");
    let mut keys_count: Vec<Option<usize>> = vec![None; max_element + 1];
    for &key in &src[..] {
        if let Ok(key) = key.try_into() {
            keys_count[key] = keys_count[key].or(Some(0)).map(|count| count + 1);
        }
    }

    let mut key = 0;
    let mut src_idx = 0;
    while key < keys_count.len() {
        let count = keys_count[key];
        if let Some(mut count) = count {
            while count != 0 {
                src[src_idx] = T::from_usize(key).expect("key was derived from T");
                src_idx += 1;
                count -= 1;
            }
        }
        key += 1;
    }
}

#[test]
fn count_sort_test() {
    use crate::test_utils::get_test_vectors;

    for (input, sorted) in get_test_vectors().iter_mut() {
        count_sort(input);

        if input.iter().any(|&v| v < 0) {
            continue;
        }

        assert_eq!(input, sorted);
    }
}
