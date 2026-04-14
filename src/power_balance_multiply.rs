use num_bigint::BigUint;
use num_traits::Zero;

pub struct PowerBalanceMultiply;

impl PowerBalanceMultiply {
    pub fn multiply(mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_a.is_zero() || num_b.is_zero() {
            return BigUint::zero();
        }

        if num_a < num_b {
            std::mem::swap(&mut num_a, &mut num_b);
        }

        let a_bits = num_a.bits();
        let b_bits = num_b.bits();

        if a_bits <= b_bits + 128 {
            return num_a * num_b;
        }

        let delta = (a_bits - b_bits) / 2;

        let mut q = num_a.clone();
        q >>= delta as usize;

        let mut r = num_a;
        let mask = (BigUint::from(1u32) << delta as usize) - 1u32;
        r &= mask;

        let mut b_shifted = num_b;
        b_shifted <<= delta as usize;

        let balanced_product = q * b_shifted;
        let remainder_product = Self::multiply(r, num_b);

        balanced_product + remainder_product
    }
}
