extern crate num_bigint;
extern crate rand;

use num_bigint::BigUint;
use rand::Rng;
use std::time::Instant;

use crate::karatsuba_multiply::MultiplyKaratsuba;
use crate::naive_multiply::NaiveMultiplication;
use crate::power_multiply::MultiplyPowerTwo;

pub fn benchmark() -> std::collections::HashMap<String, Vec<u128>> {
    let mult = MultiplyPowerTwo::new();

    let methods: Vec<(&str, Box<dyn Fn(&BigUint, &BigUint) -> BigUint>)> = vec![
        ("Naive", Box::new(|a, b| NaiveMultiplication.multiply(a, b))),
        (
            "Karatsuba",
            Box::new(|a, b| MultiplyKaratsuba::multiply(a, b)),
        ),
        ("PowerTwo", Box::new(|a, b| mult.multiply(a, b))),
    ];

    let input_sizes = 1..=10;
    let runs = 50;
    let mut results: std::collections::HashMap<String, Vec<u128>> =
        std::collections::HashMap::new();

    for size in input_sizes {
        let num_a = BigUint::from(1024u64);
        let num_b = BigUint::from(rand::thread_rng().gen_range(0..10u64.pow(size) - 1));

        println!("\nTesting with input size: {} digits", size);

        for (name, func) in methods.iter() {
            let mut total_time = 0;
            for _ in 0..runs {
                let start_time = Instant::now();
                func(&num_a, &num_b);
                total_time += start_time.elapsed().as_nanos();
            }

            let avg_time = total_time / runs as u128;
            results
                .entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(avg_time);
            println!("{} Avg Time: {}ns", name, avg_time);
        }
    }

    results
}
