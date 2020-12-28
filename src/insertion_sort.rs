//! Insertion sort. O(n^2).

use super::get_sort_tests;

fn insertion_sort_1<T: PartialOrd + Clone>(src: &mut [T]) {
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

// Look throughly at `insertion_sort_2` and `insertion_sort_3`. They show the same sorting idea: you
// put `current` value in the proper place only once. To do that you have to free the place for it.
// So you move to the right all the values in range of `0..i`, which are bigger than `src[i]` (i.e.
// `current`).
//
// But these implementations have crucial things to mention. Index `j` points to numbers that
// potentially could be moved to the right if the condition is met (so `src[j+1] = src[j]`).
// So if we found such `j` for which `src[j] < current`, then we place `current` to `src[j+1]`.
// That's how values are moved to the right freeing place for `current`, which is smaller then
// them.
// However, if you reached the beginning (i.e. `j == 0` and `src[j] > current`), it means
// you found the smallest (currenty) value of the 0..i range.
// Logically, it's place is src[0] (`src[j]`), not src[1] (so `src[j+1]`).
//
// We could avoid all that stuff if `j` was `isize`, but it's quite strange to do conversion from
// `isize` to `usize` all the time.

fn insertion_sort_2<T: PartialOrd + Clone>(src: &mut [T]) {
    for i in 1..src.len() {
        let current = src.get(i).expect("out of bounds").clone();
        let mut j = i - 1;
        while Some(&current) < src.get(j) {
            src[j + 1] = src[j].clone();
            if j == 0 {
                break;
            }
            j -= 1;
        }
        let j = if j == 0 && &src[j] == &src[j + 1] {
            j
        } else {
            j + 1
        };
        src[j] = current;
    }
}

fn insertion_sort_3<T: PartialOrd + Copy>(src: &mut [T]) {
    for i in 1..src.len() {
        let current = src[i];
        let mut j = i - 1;
        loop {
            if current >= src[j] {
                j += 1;
                break;
            }
            src[j + 1] = src[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }
        src[j] = current;
    }
}

// Same as insertion_sort_1, but more readable.
fn insertion_sort_4<T: PartialOrd + Clone>(src: &mut [T]) {
    for cur in 1..src.len() {
        let mut i = cur;
        while i > 0 && src[i] < src[i - 1] {
            src.swap(i, i - 1);
            i -= 1;
        }
    }
}

#[test]
fn insertion_sort_test() {
    for (input, sorted) in get_sort_tests().iter_mut() {
        let mut input2 = input.clone();
        let mut input3 = input.clone();
        let mut input4 = input.clone();

        insertion_sort_1(input);
        insertion_sort_2(&mut input2);
        insertion_sort_3(&mut input3);
        insertion_sort_4(&mut input4);

        assert_eq!(input, sorted);
        assert_eq!(&mut input2, sorted);
        assert_eq!(&mut input3, sorted);
        assert_eq!(&mut input4, sorted);
    }
}
