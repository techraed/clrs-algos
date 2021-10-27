//! Insertion sort. O(n^2).
//! Incremental algorithm which looks for a proper place in sorted area for the value from unsorted area.

/// Insertion sort implementation.
///
/// Finds for `src[cur]` value it's place in sorted area (which is [0; cur)
/// by moving it to the left everytime there is a value bigger than it in the sorted area.
pub fn insertion_sort<T: PartialOrd + Clone>(src: &mut [T]) {
    for cur in 1..src.len() {
        let mut i = cur;
        while i > 0 && src[i] < src[i - 1] {
            src.swap(i, i - 1);
            i -= 1;
        }
    }
}

/// Explicit version of insertion sort. Implemented that "noisy" way in order to explain the idea of the algorithm.
///
/// Look thoroughly at `insertion_sort_3`. You put `current` value in the proper place only once.
/// To do that you have to free the place for it. So you move to the right all the values in range of [0;i)`,
/// which are bigger than `src[i]` (i.e.`current`).
///
/// But these implementations have crucial things to mention. Index `j` points to numbers that
/// potentially could be moved to the right if the condition is met (so `src[j+1] = src[j]`).
/// So if we found such `j` for which `src[j] < current`, then we place `current` to `src[j+1]`.
/// That's how values are moved to the right freeing place for `current`, which is smaller then
/// them.
/// However, if you reached the beginning (i.e. `j == 0` and `src[j] > current`), it means
/// you found the smallest (currently) value of the 0..i range.
/// Logically, it's place is `src[0]` (`src[j]`), not `src[1]` (`src[j+1]`).
///
/// We could avoid all that stuff after while loop if `j` was `isize`, but it's quite strange to do conversion from
/// `isize` to `usize` all the time.
#[allow(unused)]
fn insertion_sort_explicit<T: PartialOrd + Copy>(src: &mut [T]) {
    for i in 1..src.len() {
        let current = src[i];
        let mut j = i - 1;
        while j > 0 && src[j] > current {
            src[j + 1] = src[j];
            j -= 1;
        }

        if src[j] > current {
            // if we entered the path, then j == 0
            // so we found currently smallest value of the 0..i range.
            src[j + 1] = src[j];
            src[j] = current;
        } else {
            src[j + 1] = current;
        }
    }
}

/// Alternative to main implementation. It's more "noisy", then the main one.
#[allow(unused)]
fn insertion_sort_alternative<T: PartialOrd + Clone>(src: &mut [T]) {
    for i in 1..src.len() {
        let current = src.get(i).expect("out of bounds").clone();
        let mut j = i - 1;
        while Some(&current) < src.get(j) {
            src.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

#[test]
fn insertion_sort_test() {
    use crate::test_utils::test_sorting_algorithm;

    assert!(test_sorting_algorithm(insertion_sort_alternative).is_ok());
    assert!(test_sorting_algorithm(insertion_sort_explicit).is_ok());
    assert!(test_sorting_algorithm(insertion_sort).is_ok());
}
