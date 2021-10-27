//! Heap sort. O(n*log n).
//!
//! Notable that this is "in-place" algorithm with a quite effective time complexity.
//! However, to reach this we need to maintain all the data in the [heap](https://en.wikipedia.org/wiki/Heap_(data_structure)) data structure.

/// Heap sort implementation.
///
/// We can classify heaps in to two different kinds:
/// - min heap, where nodes are presented in a non-decreasing order;
/// - max heap, where nodes are presented in a non-increasing order.
/// This implementation uses max heap to sort `src`.
///
/// We could create a heap for this task, but as long as we only need to sort `src`, then we omit explicitly creating the data structure.
///
/// So basically heap sort has 2 procedures phases:
/// - building max heap.
/// - max heapify.
///
/// The most basic one is heapify procedure, which recovers heap nodes order. So if we have a max heap and one of subtrees has wrong nodes order
/// (parent node is less than the child node), in this case we "max heapify" this subtree. Heapifying basically moves down out-of-order node until
/// it reaches the position, where it is larger than its child nodes.
///
/// Build max heap procedure uses heapify to create the appropriate order in `src`. It starts building the heap from leaves, "max heapifying" subtrees with
/// "leaf parent - leaves" structure. Starting from the bottom, where we have guarantees that subtrees are "max heapfied", we go to the top each time making
/// the part of `src` that maintains the order larger and larger until we reach the top.
///
/// Finally, how heap sort works:
/// 1. Build max heap.
/// 2. Swap the first element, which is the largest in the heap, with the last.
/// 3. Call max heapify starting from the first element, which violated the order. But heapify work on the different scope now - `src[..last_swapped]`.
/// So the idea is that we put to the end biggest values, which were in the root of the heap, and maintain the order in the other "unsorted" part of the `src`.
/// 4. Repeat until heap has only one element.
///
/// # NOTE
///
/// Heapify requires that only one minimal subtree (parent and its 2 children) has order violation, but other subtree (with root in parents children)
/// maintain the max heap order. Also, heapify should be called only in the parent, which violates the order, because otherwise you can skip a violated
/// subtree.
pub fn heap_sort<T: PartialOrd + Clone>(src: &mut [T]) {
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

fn heap_sort_impl<T: PartialOrd + Clone>(src: &mut [T]) {
    build_max_heap(src);
    let mut heap_size = src.len();
    for node_index in (1..src.len()).rev() {
        src.swap(0, node_index);
        heap_size -= 1;
        max_heapify(&mut src[..heap_size], 0);
    }
}

fn build_max_heap<T: PartialOrd + Clone>(src: &mut [T]) {
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
