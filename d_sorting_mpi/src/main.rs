mod mergesort;

use mpi::traits::*;
use rand::seq::SliceRandom;
use std::time;

const CPUS: usize = 8;

const DO_DEBUG_LOG: bool = false;

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

// mpiexec -np 8 .\d_sorting_mpi.exe
//  mpirun -np 8 ./d_sorting_mpi
fn main() {
    let do_debug_log = DO_DEBUG_LOG;

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    if size != CPUS as i32 {
        eprintln!("\n[!] Size of MPI_COMM_WORLD must be {CPUS}, but is {size}.");
        return;
    }

    match rank {
        0 => {
            let arr_orig = generate_input();

            println!("\n== Sequential ==");
            let mut arr = arr_orig.clone();
            let now = time::Instant::now();
            mergesort::sequential(&mut arr);
            print_results(&arr, &now);

            println!("\n== Concurrent ==");
            let mut arr = arr_orig.clone();
            let now = time::Instant::now();
            mergesort::concurrent(&mut arr);
            print_results(&arr, &now);

            println!("\n== MPI ==");
            let mut arr = arr_orig.clone();
            let now = time::Instant::now();

            let chunk_size = arr.len() / (CPUS - 1);

            // Send chunks to workers
            for i in 1..CPUS {
                let start = (i - 1) * chunk_size;
                let end = if i == CPUS - 1 {
                    arr.len()
                } else {
                    start + chunk_size
                };
                world.process_at_rank(i as i32).send(&arr[start..end]);
            }

            // Get sorted chunks from workers
            for i in 1..CPUS {
                let (chunk, status) = world.process_at_rank(i as i32).receive_vec::<u16>();
                if do_debug_log {
                    println!("Master received sorted chunk {i}; {status:?}.");
                }
                let start = (i - 1) * chunk_size;
                arr[start..(start + chunk.len())].copy_from_slice(&chunk);
            }

            // Linear merge the chunks
            let mut merged_size = chunk_size;
            for _ in 1..CPUS {
                let end = (merged_size + chunk_size).min(arr.len());
                mergesort::merge(&mut arr[..end], merged_size);
                merged_size = end;
            }

            // Done
            print_results(&arr, &now);
        }
        _ => {
            let (mut chunk, status) = world.process_at_rank(0).receive_vec::<u16>();
            if do_debug_log {
                println!(
                    "Process {} got chunk of size {}; {:?}.",
                    rank,
                    chunk.len(),
                    status
                );
            }
            mergesort::sequential(&mut chunk);
            if do_debug_log {
                println!("Process {rank} sorted its chunk.");
            }
            world.process_at_rank(0).send(&chunk);
        }
    }
}
