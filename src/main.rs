pub mod balance_multiply;
pub mod benchmark;
pub mod karatsuba_multiply;
pub mod naive_multiply;
pub mod power_multiply;

fn main() {
    benchmark::benchmark();
}
