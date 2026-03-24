mod mergesort;
mod quicksort;

use rand::seq::SliceRandom;
use std::time;

const NUM_VAL_MIN: u16 = 1;
const NUM_VAL_MAX: u16 = 10_000;
const NUM_AMOUNT: usize = 1000;

fn generate_input() -> [u16; NUM_AMOUNT] {
    let mut all_numbers: Vec<_> = (NUM_VAL_MIN..=NUM_VAL_MAX).collect();
    all_numbers.shuffle(&mut rand::rng());
    all_numbers[..NUM_AMOUNT].try_into().unwrap()
}

fn print_results(arr: &[u16], instant: &time::Instant) {
    println!("Is sorted: {}", arr.is_sorted());
    println!("~ Elapsed: {:.2?}", instant.elapsed());
    println!(" As nanos: {:?}", instant.elapsed().as_nanos());
}

fn main() {
    if let Ok(cpus_estimate) = std::thread::available_parallelism() {
        println!("CPUS estimate: {cpus_estimate}");
    }

    let arr_orig = generate_input();

    println!("\n== Sequential quicksort ==");
    let mut arr = arr_orig.clone();
    let now = time::Instant::now();
    quicksort::sequential(&mut arr);
    print_results(&arr, &now);

    println!("\n== Parallel quicksort ==");
    let mut arr = arr_orig.clone();
    let now = time::Instant::now();
    quicksort::parallel(&mut arr);
    print_results(&arr, &now);

    println!("\n== Sequential mergesort ==");
    let mut arr = arr_orig.clone();
    let now = time::Instant::now();
    mergesort::sequential(&mut arr);
    print_results(&arr, &now);

    println!("\n== Parallel mergesort ==");
    let mut arr = arr_orig.clone();
    let now = time::Instant::now();
    mergesort::parallel(&mut arr);
    print_results(&arr, &now);
}
