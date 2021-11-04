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

/// Count sort implementation
///
/// Unfortunately it requires so additional data space.
pub fn count_sort<T: PartialOrd + Clone>(src: &mut[T]) {
    match src.len() {
        0 | 1 => {}
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1)
            }
        }
        _ => count_sort_impl(src),
    }
}

fn count_sort_impl<T: PartialOrd + Clone>(src: &mut[T]) {
    let max_value = src.iter().max().expect("src has at least 3 elements");
}
