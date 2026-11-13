//! Multi-threaded implementation using `std::thread::scope`.

use std::thread;

/// Splits the input slice into chunks and processes them using `std::thread::scope`,
/// computing for each `n` the smallest `k >= 1` such that `2^k ≡ 1 (mod n)`.
///
/// # Preconditions
/// - `out_num.len() >= in_num.len()`
/// - Input values must be odd integers (`< 2^63`) to avoid infinite loops or `u64` overflow.
pub fn process(in_num: &[u64], out_num: &mut [u64]) {
    // Cap thread count (note: currently fixed to 1 thread for demo purposes).
    let num_threads = 1.min(in_num.len());

    if num_threads == 0 {
        return;
    }

    let chunk_size = in_num.len().div_ceil(num_threads);

    // Scoped threads allow safely borrowing input/output slices without static lifetimes.
    thread::scope(|s| {
        for (in_chunk, out_chunk) in in_num
            .chunks(chunk_size)
            .zip(out_num.chunks_mut(chunk_size))
        {
            s.spawn(move || {
                for i in 0..in_chunk.len() {
                    let mut tmp_sum = in_chunk[i];
                    let mut sht_sum: u64 = 0;

                    while (tmp_sum & (tmp_sum + 1)) != 0 {
                        let val = (tmp_sum + 1) & !tmp_sum;
                        let sht_r = (63 - val.leading_zeros()) as u64;
                        sht_sum = sht_sum + sht_r;

                        tmp_sum = tmp_sum >> sht_r;
                        tmp_sum = tmp_sum + in_chunk[i];
                    }

                    out_chunk[i] = sht_sum + (63 - (tmp_sum + 1).leading_zeros()) as u64;
                }
            });
        }
    });
}