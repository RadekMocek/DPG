use crate::matrix::SquareMx;
use std::thread;

fn check_and_get_eq_dim(mx1: &SquareMx, mx2: &SquareMx) -> Option<usize> {
    if mx1.dim == mx2.dim {
        return Some(mx1.dim);
    }
    None
}

pub(super) fn sequential(mx1: &SquareMx, mx2: &SquareMx) -> SquareMx {
    let Some(dim) = check_and_get_eq_dim(mx1, mx2) else {
        eprintln!("Aborting squaremx_mul, non compatible matrices were provided.");
        return SquareMx::zeroes_square(0);
    };

    let mx2t = mx2.get_transposed();
    let mut result = SquareMx::zeroes_square(dim);

    for row_n in 0..dim {
        for col_n in 0..dim {
            result[(row_n, col_n)] = mx1
                .get_row_slice(row_n)
                .iter()
                .zip(mx2t.get_row_slice(col_n))
                .map(|(a, b)| a * b)
                .sum();
        }
    }

    result
}

pub(super) fn concurrent(mx1: &SquareMx, mx2: &SquareMx, n_threads: usize) -> SquareMx {
    let Some(dim) = check_and_get_eq_dim(mx1, mx2) else {
        eprintln!("Aborting squaremx_mul, non compatible matrices were provided.");
        return SquareMx::zeroes_square(0);
    };

    let dim_chunk_len = dim.div_ceil(n_threads);
    let mx2t = mx2.get_transposed();
    let mut result = SquareMx::zeroes_square(dim);

    thread::scope(|s| {
        let a = mx1;
        let b = &mx2t;
        let mut handles = Vec::new();
        for dim_start in (0..dim).step_by(dim_chunk_len) {
            let dim_end = (dim_start + dim_chunk_len).min(dim);

            handles.push(s.spawn(move || {
                let mut chunk = Vec::<u32>::with_capacity((dim_end - dim_start) * dim);
                fill_result_chunk(a, b, dim, dim_start..dim_end, &mut chunk);
                (dim_start, chunk)
            }));
        }
        for handle in handles {
            let (dim_start, chunk) = handle.join().unwrap();
            let item_start = dim_start * dim;
            result.items[item_start..(item_start + chunk.len())].copy_from_slice(&chunk);
        }
    });

    result
}

pub(super) fn fill_result_chunk(
    a: &SquareMx,
    b: &SquareMx,
    dim: usize,
    dim_range: core::ops::Range<usize>,
    result_chunk: &mut Vec<u32>,
) {
    for row_n in dim_range {
        for col_n in 0..dim {
            result_chunk.push(
                a.get_row_slice(row_n)
                    .iter()
                    .zip(b.get_row_slice(col_n))
                    .map(|(a, b)| a * b)
                    .sum(),
            );
        }
    }
}
