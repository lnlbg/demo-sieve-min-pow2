# demo-sieve-min-pow2

A Rust demonstration project that computes, for a list of odd integers $n$, the smallest positive power of two $k \ge 1$ such that:

$$2^k \equiv 1 \pmod n$$

(i.e. the multiplicative order of $2$ modulo $n$, $\text{ord}_n(2)$, computed via a division-free bitwise shift-and-add algorithm) and benchmarks three CPU execution models as a reference baseline prior to GPU/CUDA scaling:

1. **Sequential** (`src/sequential.rs`) — single-threaded baseline.
2. **Parallel / Rayon** (`src/parallel.rs`) — data-parallel execution using `rayon`.
3. **Standard Threads** (`src/threads.rs`) — manual chunk partitioning via `std::thread::scope`.

---

## How the Algorithm Works

For any odd integer $n$, the goal is to find the minimal $k \ge 1$ such that $n \mid (2^k - 1)$:
1. On each iteration, the kernel isolates the lowest zero bit of `tmp_sum` via `(tmp_sum + 1) & !tmp_sum` to count trailing `1`-bits (`sht_r`).
2. It right-shifts those bits out (`>> sht_r`), accumulates the shift count in `sht_sum`, and adds `n` back.
3. Once `tmp_sum` reaches the form $2^m - 1$ (checked via `(x & (x + 1)) == 0`), the accumulated shifts plus the remaining bit-width equal the exact minimal $k$.

---

## Project Structure

```text
.
├── Cargo.toml
└── src/
    ├── main.rs        # Dataset setup, timing harness, and result verification
    ├── sequential.rs  # Single-threaded implementation
    ├── parallel.rs    # Rayon data-parallel implementation
    └── threads.rs     # Standard scoped-threads implementation
```

---

## Requirements

- Crate dependency: `rayon`

```toml
[dependencies]
rayon = "1"
```

---

## Usage

Always run benchmarks in `--release` mode for accurate timings:

```bash
cargo run --release
```

Build and view local API documentation:

```bash
cargo doc --open
```

---

## Notes & Limitations

- **Input Domain:** The algorithm expects odd integers $n < 2^{63}$. Even numbers do not have a valid $k \ge 1$ for $2^k \equiv 1 \pmod n$.
- **Mersenne Numbers:** In `main.rs`, values of the form $2^k - 1$ (e.g., `3`, `7`, `31`, `127`) are normalized to `1` prior to execution.
- **Thread Count in `threads.rs`:** `num_threads` is currently capped at `1` in this demo; replace `1` with `std::thread::available_parallelism()` to enable multi-core chunking.
- **Benchmark Scale:** With the default 170-element dataset, thread-spawning and pool scheduling overhead dominate runtime. Scale up the input vector to observe parallel speedup on multi-core CPUs.