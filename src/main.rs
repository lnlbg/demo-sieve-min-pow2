//! Demo benchmark comparing execution performance for finding the smallest
//! power of two `k >= 1` such that `2^k ≡ 1 (mod n)` across sequential,
//! Rayon data-parallel, and standard scoped-thread implementations.

mod parallel;
mod sequential;
mod threads;

use std::time::Instant;

// Note: No thread restrictions applied; Rayon automatically utilizes all available CPU cores.
fn main() {
    println!("--- >>> [ demo-sieve-min-pow2 ] <<< ---");

    // Input dataset: odd prime numbers in the range [3, 1021].
    let mut in_num = vec![
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509,
        521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631,
        641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751,
        757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877,
        881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009,
        1013, 1019, 1021,
    ];

    let mut out_seq = vec![0u64; in_num.len()];
    let mut out_par = vec![0u64; in_num.len()];
    let mut out_thr = vec![0u64; in_num.len()];

    // Normalize numbers of the form 2^k - 1 (Mersenne numbers) to 1.
    for i in 0..in_num.len() {
        if (in_num[i] & (in_num[i] + 1)) == 0 {
            in_num[i] = 1;
        }
    }

    println!("\n--- [1] Sequential Module Output ---");
    let start_seq = Instant::now();
    sequential::process(&in_num, &mut out_seq);
    let duration_seq = start_seq.elapsed();
    println!("⏱️ Sequential time: {:?}", duration_seq);

    println!("\n--- [2] Parallel (Rayon) Module Output ---");
    let start_par = Instant::now();
    parallel::process(&in_num, &mut out_par);
    let duration_par = start_par.elapsed();
    println!("⏱️ Parallel time: {:?}", duration_par);

    println!("\n--- [3] Standard Threads Module Output ---");
    let start_thr = Instant::now();
    threads::process(&in_num, &mut out_thr);
    let duration_thr = start_thr.elapsed();
    println!("⏱️ Std Threads time: {:?}", duration_thr);

    // Ensure all implementations produce identical outputs.
    assert_eq!(out_seq, out_par, "\n--- >>> ERROR <<< ---");
    assert_eq!(out_seq, out_thr, "\n--- >>> ERROR <<< ---");

    //    println!("\n");
    //    for i in 0..in_num.len() {
    //        println!("{} , {}", in_num[i], out_seq[i]);
    //    }

    println!("\n--- >>> OK <<< ---");
}