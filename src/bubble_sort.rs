//! Bubble sort. O(n^2).

use super::get_sort_tests;

// Smaller values "bubble" to the left
fn bubble_sort_1<T: PartialOrd + Clone>(src: &mut [T]) {
    for i in 0..src.len() - 1 {
        for j in (i + 1..src.len()).rev() {
            if src[j] < src[j - 1] {
                src.swap(j, j - 1);
            }
        }
    }
}

// Biggest values "bubble" to the right
fn bubble_sort_2<T: PartialOrd + Clone>(src: &mut [T]) {
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
    for (input, sorted) in get_sort_tests().iter_mut() {
        let mut input2 = input.clone();

        bubble_sort_1(input);
        bubble_sort_2(&mut input2);

        assert_eq!(input, sorted);
        assert_eq!(&mut input2, sorted);
    }
}
