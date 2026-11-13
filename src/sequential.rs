//! Sequential implementation of the computation.

/// Computes for each element `n` in `in_num` the smallest `k >= 1`
/// such that `2^k ≡ 1 (mod n)`, writing the result into `out_num`.
///
/// # Preconditions
/// - `out_num.len() >= in_num.len()`
/// - Input values must be odd integers (`< 2^63`) to avoid infinite loops or `u64` overflow.
pub fn process(in_num: &[u64], out_num: &mut [u64]) {
    let size = in_num.len();
    for i in 0..size {
        let mut tmp_sum = in_num[i];
        let mut sht_sum: u64 = 0;

        // Loop until `tmp_sum` is of the form 2^m - 1 (all trailing bits are 1).
        while (tmp_sum & (tmp_sum + 1)) != 0 {
            // Isolate the lowest zero bit and count trailing 1-bits.
            let val = (tmp_sum + 1) & !tmp_sum;
            let sht_r = (63 - val.leading_zeros()) as u64;
            sht_sum = sht_sum + sht_r;

            tmp_sum = tmp_sum >> sht_r;
            tmp_sum = tmp_sum + in_num[i];
        }

        // Add the bit-length of the final all-ones value.
        out_num[i] = sht_sum + (63 - (tmp_sum + 1).leading_zeros()) as u64;
    }
}