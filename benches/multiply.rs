// benches/multiply.rs
//! Quick sanity: BalanceMultiply vs Native

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use num_bigint::BigUint;
use learn_rust::PowerBalanceMultiply;

// ============================================================================
// Size configurations — switch between them
// ============================================================================

const SIZES_BASE: &[usize] = &[
    64, 256, 1024, 4096, 16384, 65536, 262144, 1048576,
];

// const SIZES_EXTENDED: &[usize] = &[
//     64, 256, 1024, 4096, 16384, 65536, 262144, 1048576,
//     2097152, 4194304, 8388608, 16777216, 33554432, 67108864,
// ];

// 🔧 Change this to SIZES_EXTENDED for the full run
const SIZES: &[usize] = SIZES_BASE;

// ============================================================================
// Helpers
// ============================================================================

fn gen_random_bits(bits: usize) -> BigUint {
    (BigUint::from(2u64).pow(bits as u32) - 1u64) / 3u64
}

fn gen_power_of_two_minus_one(bits: usize) -> BigUint {
    BigUint::from(2u64).pow(bits as u32) - 1u64
}

fn bench_equal(c: &mut Criterion) {
    let mut group = c.benchmark_group("equal");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(5));

    for &bits in SIZES {
        let a = gen_random_bits(bits);
        let b = &a * 7u64 + 13u64;

        group.bench_with_input(BenchmarkId::new("balance", bits), &bits, |bencher, _| {
            let a = a.clone();
            let b = b.clone();
            bencher.iter(|| PowerBalanceMultiply::multiply(black_box(a.clone()), black_box(b.clone())))
        });

        group.bench_with_input(BenchmarkId::new("native", bits), &bits, |bencher, _| {
            let a = a.clone();
            let b = b.clone();
            bencher.iter(|| black_box(a.clone()) * black_box(b.clone()))
        });
    }
    group.finish();
}

fn bench_imbalanced(c: &mut Criterion) {
    let mut group = c.benchmark_group("imbalanced");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(5));

    for &(big, small) in &[(512, 64), (2048, 64), (8192, 64), (32768, 64), (131072, 64)] {
        let a = gen_power_of_two_minus_one(big);
        let b = gen_power_of_two_minus_one(small);
        let label = format!("{}b_{}b", big, small);

        group.bench_with_input(BenchmarkId::new("balance", &label), &label, |bencher, _| {
            let a = a.clone();
            let b = b.clone();
            bencher.iter(|| PowerBalanceMultiply::multiply(black_box(a.clone()), black_box(b.clone())))
        });

        group.bench_with_input(BenchmarkId::new("native", &label), &label, |bencher, _| {
            let a = a.clone();
            let b = b.clone();
            bencher.iter(|| black_box(a.clone()) * black_box(b.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_equal, bench_imbalanced);
criterion_main!(benches);
