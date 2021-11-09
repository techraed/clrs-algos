//! Radix sort. With some conditions has a O(n) time complexity.

/// There are two reference implementations:
/// 1. Using buckets: https:///github.com/weihanglo/rust-algorithm-club/tree/master/src/sorting/radix_sort
/// 2. Using count sort as a subroutine https:///github.com/myyrakle/buldak/blob/main/src/lib/radix.rs
/// Ideas to make radix sort better:
/// 1. more generic using traits
/// 2. rebasing numbers in accordance to equations from CLRS