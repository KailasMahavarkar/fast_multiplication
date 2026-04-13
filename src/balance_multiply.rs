use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Zero;

pub struct BalanceMultiply;

impl BalanceMultiply {
    #[inline]
    pub const fn new() -> Self {
        BalanceMultiply
    }

    pub fn balance_pair(&self, mut num_a: BigUint, mut num_b: BigUint) -> (BigUint, BigUint, BigUint) {
        let mut accumulator = BigUint::zero();
        let mut a_bits = num_a.bits();
        let mut b_bits = num_b.bits();

        while a_bits > b_bits + 64 {
            let a_odd = num_a.is_odd();
            let b_odd = num_b.is_odd();

            if a_odd && b_odd {
                let a_minus_1 = &num_a - 1u32;
                accumulator += &a_minus_1 + &num_b;
                num_a = a_minus_1 >> 1;
                num_b = (&num_b - 1u32) << 1;
                a_bits = a_bits.saturating_sub(1);
                b_bits = b_bits.saturating_add(1);
            } else if !a_odd && !b_odd {
                num_a >>= 1;
                num_b <<= 1;
                a_bits = a_bits.saturating_sub(1);
                b_bits = b_bits.saturating_add(1);
            } else if a_odd {
                accumulator += &num_b;
                num_a = (&num_a - 1u32) >> 1;
                num_b <<= 1;
                a_bits = a_bits.saturating_sub(1);
                b_bits = b_bits.saturating_add(1);
            } else {
                accumulator += &num_a;
                num_a >>= 1;
                num_b = (&num_b - 1u32) << 1;
                a_bits = a_bits.saturating_sub(1);
                b_bits = b_bits.saturating_add(1);
            }
        }

        (num_a, num_b, accumulator)
    }

    #[inline]
    pub fn square(&self, num: &BigUint) -> BigUint {
        num * num
    }

    /// Euclidean product decomposition using single-pass division.
    pub fn solve(&self, mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_b.is_zero() {
            return BigUint::zero();
        }
        if num_a.bits() < 4096 {
            return num_a * num_b;
        }

        let mut result = BigUint::zero();
        while !num_b.is_zero() {
            let (quotient, remainder) = num_a.div_rem(&num_b);
            result += &quotient * (&num_b * &num_b);
            num_a = num_b;
            num_b = remainder;
        }
        result
    }

    #[inline]
    pub fn multiply(&self, mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_a.is_zero() || num_b.is_zero() {
            return BigUint::zero();
        }

        if num_a < num_b {
            std::mem::swap(&mut num_a, &mut num_b);
        }

        let a_bits = num_a.bits();
        let b_bits = num_b.bits();

        if a_bits < 300 {
            return num_a * num_b;
        }

        // Power-of-two fast path (only worth checking for large inputs)
        if Self::is_power_of_two(&num_a) {
            return num_b << num_a.trailing_zeros().unwrap_or(0);
        }
        if Self::is_power_of_two(&num_b) {
            return num_a << num_b.trailing_zeros().unwrap_or(0);
        }

        let bit_ratio = if b_bits > 0 { a_bits / b_bits } else { u64::MAX };

        // Tier 2: Highly imbalanced
        if bit_ratio > 8 {
            let (balanced_a, balanced_b, accumulator) = self.balance_pair(num_a, num_b);
            self.solve(balanced_a, balanced_b) + accumulator
        }
        // Tier 3: Moderately imbalanced
        else if bit_ratio > 2 {
            let (balanced_a, balanced_b, accumulator) = self.balance_pair(num_a, num_b);
            balanced_a * balanced_b + accumulator
        }
        // Tier 4: Near-equal
        else {
            num_a * num_b
        }
    }

    #[inline]
    fn is_power_of_two(num: &BigUint) -> bool {
        if num.is_zero() {
            return false;
        }
        let digits = num.to_u32_digits();
        // BigUint stores digits little-endian, trims high zeros.
        // For a power of two: the last (MSB) digit is a power of two,
        // and all preceding digits are zero.
        let msb = *digits.last().unwrap();
        if (msb & msb.wrapping_sub(1)) != 0 {
            return false; // MSB has >1 bits set
        }
        // Fast path: if len == 1 and msb is power of two, we're done
        if digits.len() == 1 {
            return true;
        }
        // Verify all lower digits are zero
        digits.iter().take(digits.len() - 1).all(|&d| d == 0)
    }
}

impl Default for BalanceMultiply {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
