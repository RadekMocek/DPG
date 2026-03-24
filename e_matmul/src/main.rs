mod matrix;
mod mxmul;

use crate::matrix::SquareMx;
use mpi::traits::*;
use std::time;

const DO_PRINT_MXS: bool = false;
const DO_DEBUG_LOG: bool = false;

const CPUS: usize = 8;

const MX_SIZE: usize = 3000;
const ITEM_VALUE_MIN: u32 = 0;
const ITEM_VALUE_MAX: u32 = 10;

// mpiexec -np 8 .\e_matmul.exe
//  mpirun -np 8 ./e_matmul
fn main() {
    let do_print_mxs = DO_PRINT_MXS;
    let do_debug_log = DO_DEBUG_LOG;

    let dim = MX_SIZE;

    // Init MPI
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    // Here we have hardcoded 8 CPUS
    if size != CPUS as i32 {
        eprintln!("\n[!] Size of MPI_COMM_WORLD must be {CPUS}, but is {size}.");
        return;
    }

    match rank {
        // == MPI Master ==
        0 => {
            let mx1 = SquareMx::new_random(dim, ITEM_VALUE_MIN, ITEM_VALUE_MAX);

            if do_print_mxs {
                print!("{mx1}");
            } else {
                println!("mx1 created, size {dim}x{dim}");
            }

            let mx2 = SquareMx::new_random(dim, ITEM_VALUE_MIN, ITEM_VALUE_MAX);

            if do_print_mxs {
                println!(" * {mx2}");
            } else {
                println!("mx2 created, size {dim}x{dim}");
            }

            println!("\n=== MXMUL Sequential ===");
            let now = time::Instant::now();
            let mxr1 = mxmul::sequential(&mx1, &mx2);
            println!("~ Elapsed: {:.2?}", now.elapsed());

            if do_print_mxs {
                println!("{mxr1}");
            }

            println!("\n=== MXMUL Concurrent ===");
            let now = time::Instant::now();
            let mxr2 = mxmul::concurrent(&mx1, &mx2, CPUS);
            println!("~ Elapsed: {:.2?}", now.elapsed());

            if do_print_mxs {
                println!("{mxr2}");
            }

            println!("\n=== MXMUL MPI ===");
            let now = time::Instant::now();

            let mx2t = mx2.get_transposed();
            let dim_chunk_len = dim.div_ceil(CPUS);

            let mut mpi_result_chunk_master = Vec::<u32>::new();

            let mut rank_n = 0;

            for dim_start in (0..dim).step_by(dim_chunk_len) {
                let dim_end = (dim_start + dim_chunk_len).min(dim);

                rank_n += 1;
                if rank_n != CPUS {
                    let process = world.process_at_rank(rank_n as i32);
                    process.send(&mx1.items[(dim_start * dim)..(dim_end * dim)]);
                    process.send(&mx2t.items[..]);
                } else {
                    if do_debug_log {
                        println!("Master will handle the rest ({} rows)", dim_end - dim_start);
                    }
                    mpi_result_chunk_master.reserve((dim_end - dim_start) * dim);
                    mxmul::fill_result_chunk(
                        &mx1,
                        &mx2t,
                        dim,
                        dim_start..dim_end,
                        &mut mpi_result_chunk_master,
                    );
                }
            }

            let mut mpi_result = Vec::<u32>::with_capacity(dim * dim);
            for i in 1..CPUS {
                let (mut mpi_result_chunk, status) =
                    world.process_at_rank(i as i32).receive_vec::<u32>();
                if do_debug_log {
                    println!("Master received sorted chunk {i}; {status:?}.");
                }
                mpi_result.append(&mut mpi_result_chunk);
            }
            mpi_result.append(&mut mpi_result_chunk_master);

            let mxr3 = SquareMx {
                items: mpi_result,
                dim,
            };

            println!("~ Elapsed: {:.2?}", now.elapsed());

            if do_print_mxs {
                println!("{mxr2}");
            }

            // == Check ==
            println!("\n=== Check ===");
            if mxr1 == mxr2 {
                println!("1 == 2 OK");
            } else {
                println!("1 != 2 NOK!");
            }
            if mxr2 == mxr3 {
                println!("2 == 3 OK");
            } else {
                println!("2 != 3 NOK!");
            }
        }
        // == MPI Worker ==
        _ => {
            if do_debug_log {
                println!("Worker {rank} online.");
            }
            let dim = MX_SIZE;
            let (mx1_chunk_items, status1) = world.process_at_rank(0).receive_vec::<u32>();
            let (mx2t_items, status2) = world.process_at_rank(0).receive_vec::<u32>();

            let chunk_size = mx1_chunk_items.len();
            let n_chunk_rows = chunk_size / dim;

            if do_debug_log {
                println!(
                    "Worker {rank} got mx1_chunk of size {chunk_size} ({n_chunk_rows} rows) and mx2t; {status1:?} {status2:?}"
                );
            }

            let a = SquareMx {
                items: mx1_chunk_items,
                dim,
            };
            let b = SquareMx {
                items: mx2t_items,
                dim,
            };

            let mut mpi_result_chunk_worker = Vec::<u32>::with_capacity(chunk_size);
            mxmul::fill_result_chunk(&a, &b, dim, 0..n_chunk_rows, &mut mpi_result_chunk_worker);

            if do_debug_log {
                println!(
                    "Worker {rank} sends back result of size {}",
                    mpi_result_chunk_worker.len()
                )
            }
            world.process_at_rank(0).send(&mpi_result_chunk_worker);
        }
    }
}
