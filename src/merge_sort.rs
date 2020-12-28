use super::get_sort_tests;

fn merge_sort<T: PartialOrd + Clone + Default>(src: &mut [T]) {
    match src.len() {
        0 | 1 => return,
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1)
            }
        }
        _ => merge_sort_impl(src)
    }
}

fn merge_sort_impl<T: PartialOrd + Clone + Default>(src: &mut [T]) {
    // middle element index is q-1
    let q = (src.len() + 1)/2;
    merge_sort(&mut src[..q]);
    merge_sort(&mut src[q..]);
    merge(src, q);
}

fn merge<T: PartialOrd + Clone + Default>(src: &mut [T], mid: usize) {
    let mut left = vec![T::default(); src[..mid].len()];
    let mut right = vec![T::default(); src[mid..].len()]; 
    left.clone_from_slice(&src[..mid]);
    right.clone_from_slice(&src[mid..]);
    
    let mut i = 0;
    let mut j = 0;
    for k in 0..src.len() {
        if i == left.len() {
            src[k] = std::mem::take(&mut right[j]);
            j += 1;
        } else if j == right.len() {
            src[k] = std::mem::take(&mut left[i]);
            i += 1;
        } else if left[i] <= right[j] {
            src[k] = std::mem::take(&mut left[i]);
            i += 1;
        } else {
            src[k] = std::mem::take(&mut right[j]);
            j += 1;
        }
    }
}

#[test]
fn merge_sort_test() {
    for (input, sorted) in get_sort_tests().iter_mut() {
        merge_sort(input);
        assert_eq!(input, sorted);
    }   
}
