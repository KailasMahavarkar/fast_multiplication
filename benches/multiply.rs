use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;

use learn_rust::balance_multiply::BalanceMultiply;

fn bench_multiply(c: &mut Criterion) {
    let bm = BalanceMultiply;
    let sizes = [64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576];

    for bits in sizes {
        let num_a = (BigUint::from(2u64).pow(bits as u32) - 1u64) / 3u64;
        let num_b = &num_a * 7u64 + 13u64;

        c.bench_function(&format!("balance_{}b", bits), |b| {
            b.iter(|| bm.multiply(num_a.clone(), num_b.clone()))
        });
        c.bench_function(&format!("native_{}b", bits), |b| {
            b.iter(|| num_a.clone() * num_b.clone())
        });
    }
}

criterion_group!(benches, bench_multiply);
criterion_main!(benches);
