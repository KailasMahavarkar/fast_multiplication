extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::One;
use std::cmp::max;

pub struct MultiplyKaratsuba;

impl MultiplyKaratsuba {
    pub fn multiply(a: &BigUint, b: &BigUint) -> BigUint {
        // Base case: if either number is small enough, use normal multiplication
        if a.bits() < 64 || b.bits() < 64 {
            return a * b;
        }

        let n = max(a.bits(), b.bits()) / 2;
        let half_n = n / 2;

        let (high_a, low_a) = a.split_at(half_n.try_into().unwrap());
        let (high_b, low_b) = b.split_at(half_n.try_into().unwrap());

        let z0 = Self::multiply(&low_a, &low_b);
        let z2 = Self::multiply(&high_a, &high_b);
        let z1 = Self::multiply(&(low_a + &high_a), &(low_b + &high_b)) - &z2 - &z0;

        // Combining the results using Karatsuba's formula
        return (z2 << (2 * half_n)) + (z1 << half_n) + z0;
    }
}

trait Split {
    fn split_at(&self, at: usize) -> (BigUint, BigUint);
}

impl Split for BigUint {
    fn split_at(&self, at: usize) -> (BigUint, BigUint) {
        let mask = (BigUint::one() << at) - BigUint::one();
        let high = self >> at;
        let low = self & mask;
        (high, low)
    }
}
