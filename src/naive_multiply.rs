use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct NaiveMultiplication;

impl NaiveMultiplication {
    pub fn multiply(&self, a: &BigUint, b: &BigUint) -> BigUint {
        // Base case: if either number is smaller than 2, just return the multiplication
        if a.is_zero() || b.is_zero() {
            return BigUint::zero();
        }
        if a.bits() < 2 || b.bits() < 2 {
            return a * b;
        }

        let size = a.bits().max(b.bits()) / 2;
        let (a_high, a_low) = a.split_at(size as usize);
        let (b_high, b_low) = b.split_at(size as usize);

        // Recursively multiply the halves
        let z0 = self.multiply(&a_low, &b_low); // a_low * b_low
        let z1 = self.multiply(&(a_high.clone() + &a_low), &(b_high.clone() + &b_low)); // (a_high + a_low) * (b_high + b_low)
        let z2 = self.multiply(&a_high, &b_high); // a_high * b_high

        // Combine the results
        let middle_term = z1 - &z2 - &z0; // (a_high + a_low) * (b_high + b_low) - a_high * b_high - a_low * b_low
        let result = z2 * &BigUint::from(2u64).pow(2 * size as u32)
            + middle_term * &BigUint::from(2u64).pow(size as u32)
            + z0;

        result
    }
}

trait BigUintSplit {
    fn split_at(&self, size: usize) -> (BigUint, BigUint);
}

impl BigUintSplit for BigUint {
    fn split_at(&self, size: usize) -> (BigUint, BigUint) {
        let high = self >> size; // Shift right to get the high part
        let low = self & ((BigUint::one() << size) - BigUint::one()); // Mask to get the low part
        (high, low)
    }
}
