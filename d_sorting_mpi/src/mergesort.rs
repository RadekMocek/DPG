use crate::CPUS;
use std::thread;

pub(super) fn merge(arr: &mut [u16], div: usize) {
    let arr1 = arr[..div].to_owned();
    let arr2 = arr[div..].to_owned();

    let arr1len = arr1.len();
    let arr2len = arr2.len();

    let mut idx1 = 0;
    let mut idx2 = 0;

    for i in 0..arr.len() {
        if idx2 >= arr2len || (idx1 < arr1len && arr1[idx1] <= arr2[idx2]) {
            arr[i] = arr1[idx1];
            idx1 += 1;
        } else {
            arr[i] = arr2[idx2];
            idx2 += 1;
        }
    }
}

pub(super) fn sequential(arr: &mut [u16]) {
    let arrlen = arr.len();
    if arrlen > 1 {
        let div = arrlen / 2;
        sequential(&mut arr[..div]);
        sequential(&mut arr[div..]);
        merge(arr, div);
    }
}

pub(super) fn concurrent(arr: &mut [u16]) {
    concurrent_impl(arr, 0);
}

fn concurrent_impl(arr: &mut [u16], curr_cpus: u32) {
    let arrlen = arr.len();
    if arrlen > 1 {
        if curr_cpus < CPUS.ilog2() {
            let div = arrlen / 2;
            let (left, right) = arr.split_at_mut(div);
            thread::scope(|s| {
                s.spawn(|| concurrent_impl(left, curr_cpus + 1));
                concurrent_impl(right, curr_cpus + 1);
            });
            merge(arr, div);
        } else {
            sequential(arr);
        }
    }
}
