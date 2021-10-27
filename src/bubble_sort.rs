//! Bubble sort. O(n^2).

/// Bubble sort "left-right" implementation.
///
/// "Left-right" means smaller values "bubble" to the left.
pub fn bubble_sort_rl<T: PartialOrd + Clone>(src: &mut [T]) {
    for i in 0..src.len() - 1 {
        for j in (i + 1..src.len()).rev() {
            if src[j] < src[j - 1] {
                src.swap(j, j - 1);
            }
        }
    }
}

/// Bubble sort "right-left" implementation.
///
/// "right-left"  means biggest values "bubble" to the right.
pub fn bubble_sort_lr<T: PartialOrd + Clone>(src: &mut [T]) {
    for i in (1..src.len()).rev() {
        for j in 0..i {
            if src[j] > src[j + 1] {
                src.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn bubble_sort_test() {
    use crate::test_utils::test_sorting_algorithm;

    assert!(test_sorting_algorithm(bubble_sort_lr).is_ok());
    assert!(test_sorting_algorithm(bubble_sort_rl).is_ok());
}
