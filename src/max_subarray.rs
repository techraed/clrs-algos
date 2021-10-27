//! Finding maximum subarray algorithms.
//!
//! There are two main implementations:
//! - the one using Divide & Conquer (& Combine) strategy, which has O(n*log n) complexity;
//! - the Kadane's algorithm implementation, which has O(n) complexity.

use std::ops::Add;

/// Kadane's max subarray algorithm implementation finding solution within O(n) time.
///
/// Notice that by utilizing an additional index we reduced solution to O(n) comparing to [find_max_sum_subarray_dc](fn.find_max_sum_subarray_dc.html).
pub fn find_max_sum_subarray_kadane<T: PartialOrd + Copy + Default + Add<Output = T>>(src: &[T]) -> (Option<&[T]>, T) {
    let mut max_sum = T::default();
    let mut cur_sum = T::default();

    let mut left = None;
    let mut right = None;

    let mut cur_left = 0;
    for i in 0..src.len() {
        cur_sum = cur_sum + src[i];
        if cur_sum >= max_sum {
            max_sum = cur_sum;
            right = Some(i);
            left = Some(cur_left);
        } else if cur_sum < T::default() {
            cur_left = i + 1;
            cur_sum = T::default();
        }
    }
    if left.is_none() && right.is_none() {
        return (None, T::default());
    }
    let left = left.expect("none returned earlier");
    let right = right.map(|i| i + 1).expect("none returned earlier");
    return (Some(&src[left..right]), max_sum);
}

/// Divide and conquer implementation. O(n*log n)
///
/// Finds maximum sum sub-array. If there are several subarrays with maximum sum, then returns the
/// longest. If there are no matching sub-arrays, returns 0. By saying "no matching", I mean all the
/// values are less than `T::default()`. Some implementations can still work then, comparing array
/// elements with a kind of `T::MIN`. However, I decided to make a more generic approach to that.
///
/// Clone instead of copy is more preferable for real life generic solutions, but
/// it makes it harder to implement.
pub fn find_max_sum_subarray_dc<T: Ord + Copy + Default + Add<Output = T>>(src: &[T]) -> (Option<&[T]>, T) {
    if src.len() == 1 {
        if src[0] >= T::default() {
            return (Some(src), src[0]);
        }
        return (None, T::default());
    }
    let mid = src.len() / 2;
    let l = find_max_sum_subarray_dc(&src[0..mid]);
    let r = find_max_sum_subarray_dc(&src[mid..]);
    let c = find_max_sum_cross_subarray(src, mid);

    [l, r, c]
        .iter()
        .max_by(|(x, x_sum), (y, y_sum)| {
            x_sum.cmp(&y_sum).then_with(|| {
                // if they have same sum, find longest sub-array
                let x = x.map(|a| a.len());
                let y = y.map(|a| a.len());
                x.cmp(&y)
            })
        })
        .map(|a| *a)
        .unwrap_or((None, T::default()))
}

fn find_max_sum_cross_subarray<T: Ord + Copy + Default + Add<Output = T>>(src: &[T], mid: usize) -> (Option<&[T]>, T) {
    let mut left = None;
    let mut right = None;

    let mut cur_sum = T::default();
    let mut left_sum = T::default();
    for i in (0..mid).rev() {
        cur_sum = cur_sum + src[i];
        if cur_sum >= left_sum {
            left_sum = cur_sum;
            left = Some(i);
        }
    }

    cur_sum = T::default();
    let mut right_sum = T::default();
    for j in mid..src.len() {
        cur_sum = cur_sum + src[j];
        if cur_sum >= right_sum {
            right_sum = cur_sum;
            right = Some(j);
        }
    }

    if left.is_none() && right.is_none() {
        return (None, T::default());
    }
    let left = left.unwrap_or(mid);
    let right = right.map(|i| i + 1).unwrap_or(mid);
    return (Some(&src[left..right]), left_sum + right_sum);
}

#[test]
fn base_max_subarray_test() {
    let test_cases = [
        (
            vec![
                22, -27, 38, -34, 49, 40, 13, -44, -13, 28, 46, 7, -26, 42, 29, 0, -6, 35, 23, -37, 10, 12, -2, 18, -12, -49, -10, 37, -5, 17, 6, -11, -22,
                -17, -50, -40, 44, 14, -41, 19, -15, 45, -23, 48, -1, -39, -46, 15, 3, -32, -29, -48, -19, 27, -33, -8, 11, 21, -43, 24, 5, 34, -36, -9, 16,
                -31, -7, -24, -47, -14, -16, -18, 39, -30, 33, -45, -38, 41, -3, 4, -25, 20, -35, 32, 26, 47, 2, -4, 8, 9, 31, -28, 36, 1, -21, 30, 43, 25,
                -20, -42,
            ],
            239,
        ),
        (vec![-3, -4, -5, -6, -7], 0),
        (vec![0, 0, 1, 2], 3),
        (vec![1, 2, 3, 4], 10),
        (vec![0; 10], 0),
        (vec![0, 0, 0, -1, 0, 0], 0),
        (vec![100, 0, 0, 0, 0, 0, 0, 0], 100),
        (vec![-2, -3, -100, 0], 0),
        (vec![1, 2, 3, -100, 6], 6),
        (vec![0, -1], 0),
        (vec![-1, 0], 0),
        (vec![1, 2, 3, -2, 5], 9),
        (vec![10, -2, -3], 10),
    ];
    for (case_num, (case, res)) in test_cases.iter().enumerate() {
        let (slice1, sum1) = find_max_sum_subarray_dc(case);
        let (slice2, sum2) = find_max_sum_subarray_kadane(case);
        // consider in some cases difference
        if slice1 != slice2 {
            println!("Case {}. D&C result {:?} -- Kadane result {:?}", case_num + 1, slice1, slice2);
        }
        assert_eq!(*res, sum1);
        assert_eq!(sum1, sum2);
    }
}
