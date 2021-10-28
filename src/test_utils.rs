#[cfg(test)]
pub(crate) fn test_sorting_algorithm(f: fn(&mut [i32])) -> Result<(), ()> {
    let mut test_vectors = get_test_vectors();
    for (input, sorted) in test_vectors.iter_mut() {
        f(input);
        if !input.starts_with(sorted) {
            return Err(());
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn get_test_vectors() -> [(Vec<i32>, Vec<i32>); 18] {
    [
        (vec![9, 2, 3, 4, 1, 6, 8, 19, 20, 34], vec![1, 2, 3, 4, 6, 8, 9, 19, 20, 34]),
        (vec![10, 80, 30, 70, 40, 50, 90], vec![10, 30, 40, 50, 70, 80, 90]),
        (vec![2, 3, 4, 5, 10, 1, 11], vec![1, 2, 3, 4, 5, 10, 11]),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        (vec![1, 5, 3, 4], vec![1, 3, 4, 5]),
        (vec![1, 2, 3, 0, 5], vec![0, 1, 2, 3, 5]),
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![3, 1, 2], vec![1, 2, 3]),
        (vec![2, 1, 3], vec![1, 2, 3]),
        (vec![6, 1, 7, 9, 3, 8, 2, 5, 4, 0], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        (vec![3, 2], vec![2, 3]),
        (vec![8, 3, 7, 9, 6, 1, 9, 10], vec![1, 3, 6, 7, 8, 9, 9, 10]),
        (vec![8, 2, 78, 892, 11, 0, 34], vec![0, 2, 8, 11, 34, 78, 892]),
        (
            vec![9, 03, 83, 9, 2, 0, 1, 65, 2, 822, 9, 11, 22, 3, 3, 3, 47],
            vec![0, 1, 2, 2, 3, 3, 3, 3, 9, 9, 9, 11, 22, 47, 65, 83, 822],
        ),
        (vec![-6, 9, 0, 1, 17, 91, 0, 178], vec![-6, 0, 0, 1, 9, 17, 91, 178]),
        (vec![-3, -2, -1, -9, -5, -1, -19, -33], vec![-33, -19, -9, -5, -3, -2, -1, -1]),
        (vec![-5, -6, -7, 0, 0, 0, 0, -8, 1, 2, 3], vec![-8, -7, -6, -5, 0, 0, 0, 0, 1, 2, 3]),
        (vec![2; 5], vec![2; 5]),
    ]
}
