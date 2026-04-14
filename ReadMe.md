# fast_multiplication

Experimental BigInteger multiplication algorithms in Rust.

## Algorithms

- **Naive** — basic schoolbook multiplication
- **Karatsuba** — O(n^1.585) divide-and-conquer
- **Power of Two** — precomputed power-of-2 optimization
- **BalanceMultiply** — iterative parity-based balancing with Euclidean decomposition
- **PowerBalanceMultiply** — O(1)-step recursive power-of-two splitting

## Benchmark

```sh
cargo bench --bench multiply
```

Tests equal-size and imbalanced inputs against native `num-bigint` `*`.

## License

BSD 3-Clause — see [LICENSE.txt](LICENSE.txt)
