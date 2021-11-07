//! Count sort. With some conditions has a O(n) time complexity.
//!
//! One of the main features of the algorithms is that to sort then input array you don't have to do any comparisons. Actually there is a [theorem](https://stackoverflow.com/questions/61330147/comparison-based-sorting-is-wc-min-time-nlogn-so-what-about-best-average-case)
//! that states that the best time in worst case for comparison algorithms is *Ω(n * log n)*.
//! The idea is quite simple:
//! - you count number of each element in the input array;
//! - you count number of elements less than some current element;
//! - if you have 3 elements less than the element `x`, then the first `x` entry in the input has a position `input[3]`.
//!
//! Count sort is one of the algorithms working with a O(n) time. Actually, it works for that with some condition on input data.
//! That is if we have 1) *n* elements where each has a value from range of `1..=k`, where `k` is the largest element of the input
//! and 2) *k = O(n)* , then we have *O(n+k) = O(n+n) = O(n)* time complexity.
//!
//! Should be mentioned that it is a [stable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability) sorting algorithm, which is an important feature
//! for some other algorithms that can use count sort, for example, [radix sort](todo).

use std::convert::TryInto;

use num::{FromPrimitive, ToPrimitive};

/// Count sort entry function
///
/// If `src` inner data has a primitive type and can be casted to usize, then the sort will be performed.
/// Otherwise no mutations will be made on `src`.
pub fn count_sort<T: FromPrimitive + ToPrimitive + TryInto<usize> + Ord + Copy + Default>(src: &mut [T]) {
    let min_element = src.iter().min().copied().map(TryInto::<usize>::try_into).map(Result::ok).flatten();
    let max_element = src.iter().max().copied();
    if min_element.is_some() && max_element.map(|element| element.to_usize()).flatten().is_some() {
        count_sort_impl(src, max_element.expect("is some value"))
    }
}

/// Count sort implementation
///
/// Because of conditions stated in the module [doc]() we parametrize function with a number type value.
/// Unfortunately this algorithm requires additional space, which is a good example of space - time tradeoff.
fn count_sort_impl<T: FromPrimitive + ToPrimitive + Ord + Copy + Default>(src: &mut [T], max_element: T) {
    match src.len() {
        0 | 1 => {}
        _ => count_sort_proc(src, max_element),
    }
}

/// Count sort recursive procedure
/// todo передай сразу usize элементы
fn count_sort_proc<T: FromPrimitive + ToPrimitive + Ord + Copy + Default>(src: &mut [T], max_element: T) {
    let mut sorted: Vec<T> = vec![T::default(); src.len()];
    let max_element = max_element.to_usize().expect("rec proc callable for types, that can be converted to usize");
    let mut keys_count: Vec<usize> = vec![0; max_element + 1];
    for key in &src[..] {
        if let Some(key) = key.to_usize() {
            keys_count[key] += 1;
        }
    }
    for idx in 0..keys_count.len() {
        if idx != 0 {
            keys_count[idx] += keys_count[idx - 1];
        }
    }
    for key in &src[..] {
        if let Some(key) = key.to_usize() {
            sorted[keys_count[key] - 1] = T::from_usize(key).expect("key was derived from T");
            keys_count[key] -= 1;
        }
    }
    src.copy_from_slice(&sorted[..])
}

#[test]
fn count_sort_test() {
    use crate::test_utils::get_test_vectors;

    for (input, sorted) in get_test_vectors().iter_mut() {
        count_sort(input);
        if input.iter().all(|&v| v >= 0) {
            assert_eq!(input, sorted);
        }
    }
}
