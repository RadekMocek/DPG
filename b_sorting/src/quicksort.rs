use std::thread;

// Lomuto partition scheme
fn partition(arr: &mut [u16]) -> usize {
    let right = arr.len() - 1;
    let pivot_val = arr[right];
    let mut pivot_idx = 0;

    for j in 0..right {
        if arr[j] <= pivot_val {
            arr.swap(j, pivot_idx);
            pivot_idx += 1;
        }
    }

    arr.swap(pivot_idx, right);

    pivot_idx
}

// Sequential quicksort
pub(super) fn sequential(arr: &mut [u16]) {
    if arr.len() > 1 {
        let div = partition(arr);
        sequential(&mut arr[..div]);
        sequential(&mut arr[div + 1..]);
    }
}

// Parallel quicksort
pub(super) fn parallel(arr: &mut [u16]) {
    if arr.len() > 1 {
        let div = partition(arr);
        let (left, right) = arr.split_at_mut(div);
        thread::scope(|s| {
            s.spawn(|| parallel(left));
            parallel(&mut right[1..]);
        });
    }
}
