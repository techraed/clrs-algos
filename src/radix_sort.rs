//! Radix sort. With some conditions has a O(n) time complexity.

use num::PrimInt;

const BASE_10: u8 = 10;

/// There are two reference implementations:
/// 1. Using count sort as a subroutine: https:///github.com/weihanglo/rust-algorithm-club/tree/master/src/sorting/radix_sort + https://brilliant.org/wiki/radix-sort/
/// 2. Using buckets https:///github.com/myyrakle/buldak/blob/main/src/lib/radix.rs + https://blog.logrocket.com/radix-sort-no-comparisons-required/
/// Ideas to make radix sort better:
/// 1. more generic using traits
/// 2. rebasing numbers in accordance to equations from CLRS

/// Radix sort implementation sorting primitive numbers in base 10.
///
/// Current implementations sorts `src` using bucket method. The idea is to create a bucket per digit of base 10 (i.e. one bucket for each of `0..base`)
/// and sort numbers in `src` by each of their digits. Sorting by a digit using buckets simply means that we store a number in a bucket, which serves
/// current sorting digit. For more explanation [see](https://blog.logrocket.com/radix-sort-no-comparisons-required/).
pub fn radix_sort<T: PrimInt + Ord + Copy>(src: &mut [T]) {
    let max_digits = count_max_digits(src);
    todo!()
}

fn count_max_digits<T: PrimInt + Ord + Copy>(src: &mut[T]) -> usize {
    let mut max = src.iter().max().copied().expect("at least one element is in src");
    let divisor = T::from(BASE_10).expect("BASE value suits any number type width");
    let mut max_digits = 1;
    // todo or we could do (max.to_f64().unwrap().log10() + 1) as usize
    loop {
        max = max / divisor;
        if max == T::zero() {
            break max_digits
        }
        max_digits += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::count_max_digits;

    #[test]
    fn test_digits_count() {
        // just some simple tests
        assert_eq!(count_max_digits(&mut [123123]), 6);
        assert_eq!(count_max_digits(&mut [0, 0, 0]), 1);
        assert_eq!(count_max_digits(&mut [u128::MAX]), 39);
    }
}