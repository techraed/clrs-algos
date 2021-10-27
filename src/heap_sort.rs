//! Heap sort. O(n*log n).
//!
//! Notable that this is "in-place" algorithm with a quite effective time complexity.
//! However, to reach this we need to maintain all the data in the [heap[(https://en.wikipedia.org/wiki/Heap_(data_structure)) data structure.

pub fn heap_sort<T: PartialOrd + Clone + std::fmt::Debug>(src: &mut [T]) {
    match src.len() {
        0 | 1 => {}
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1)
            }
        }
        _ => heap_sort_impl(src),
    }
}

fn heap_sort_impl<T: PartialOrd + Clone + std::fmt::Debug>(src: &mut [T]) {
    build_max_heap(src);
    let mut heap_size = src.len();
    for node_index in (1..src.len()).rev() {
        src.swap(0, node_index);
        heap_size -= 1;
        max_heapify(&mut src[..heap_size], 0);
    }
}

fn build_max_heap<T: PartialOrd + Clone + std::fmt::Debug>(src: &mut [T]) {
    let heap_size = src.len();
    let last_leaf_index = heap_size - 1;
    let last_leaf_parent_index = match last_leaf_index % 2 {
        0 => last_leaf_index / 2 - 1,
        1 => last_leaf_index / 2,
        _ => unreachable!(),
    };
    for node_index in (0..=last_leaf_parent_index).rev() {
        max_heapify(src, node_index);
    }
}

fn max_heapify<T: PartialOrd + Clone>(src: &mut [T], start_from: usize) {
    let mut largest_index = start_from;
    let left_child_index = largest_index * 2 + 1;
    let right_child_index = largest_index * 2 + 2;

    if let Some(left_child) = src.get(left_child_index) {
        if left_child > &src[largest_index] {
            largest_index = left_child_index;
        }
    }
    if let Some(right_child) = src.get(right_child_index) {
        if right_child > &src[largest_index] {
            largest_index = right_child_index;
        }
    }
    if largest_index != start_from {
        src.swap(start_from, largest_index);
        max_heapify(src, largest_index);
    }
}

#[test]
fn heap_sort_test() {
    use crate::test_utils::test_sorting_algorithm;

    assert!(test_sorting_algorithm(heap_sort).is_ok());
}

// There are basically 2 heap types:
// - min heap, where nodes are presented in a non-decreasing order;
// - max heap,
