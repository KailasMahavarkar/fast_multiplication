extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Zero;

pub struct BalanceMultiply;

impl BalanceMultiply {
    /// Division-free balancing transformation.
    /// Maintains invariant: `original_a * original_b = a' * b' + accumulator`
    pub fn balance_pair(
        &self,
        mut num_a: BigUint,
        mut num_b: BigUint,
    ) -> (BigUint, BigUint, BigUint) {
        let mut accumulator = BigUint::zero();

        // Stop when operands are within ~64 bits. Further balancing yields diminishing returns
        // and increases shift/allocate overhead.
        while num_a.bits() > num_b.bits() + 64 {
            let a_odd = num_a.is_odd();
            let b_odd = num_b.is_odd();

            // All branches use only shifts and additions. No division.
            if a_odd && b_odd {
                let a_minus_1 = &num_a - 1u32;
                accumulator += &a_minus_1 + &num_b;
                num_a = a_minus_1 >> 1;
                num_b = (&num_b - 1u32) << 1;
            } else if !a_odd && !b_odd {
                num_a >>= 1;
                num_b <<= 1;
            } else if a_odd {
                // a odd, b even
                accumulator += &num_b;
                num_a = (&num_a - 1u32) >> 1;
                num_b <<= 1;
            } else {
                // a even, b odd
                accumulator += &num_a;
                num_a >>= 1;
                num_b = (&num_b - 1u32) << 1;
            }
        }

        (num_a, num_b, accumulator)
    }

    /// Fast squaring: delegate to `num-bigint`'s internally optimized Karatsuba/Toom-Cook.
    #[inline]
    pub fn square(&self, num: &BigUint) -> BigUint {
        num * num
    }

    /// Euclidean product decomposition.
    /// Kept for mathematical fidelity, but division is explicitly the bottleneck.
    pub fn solve(&self, mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_b.is_zero() {
            return BigUint::zero();
        }

        // Early fallback: native multiplication beats Euclidean decomposition for small operands
        if num_a.bits() < 256 {
            return num_a * num_b;
        }

        let mut result = BigUint::zero();
        while !num_b.is_zero() {
            let quotient = &num_a / &num_b;
            result += &quotient * self.square(&num_b);
            let remainder = &num_a % &num_b;
            num_a = num_b;
            num_b = remainder;
        }
        result
    }

    pub fn multiply(&self, mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_a.is_zero() || num_b.is_zero() {
            return BigUint::zero();
        }

        // Ensure consistent operand ordering for balancing
        if num_a < num_b {
            std::mem::swap(&mut num_a, &mut num_b);
        }

        // Base case: `num-bigint` switches to Karatsuba around 100-300 bits.
        // Below that, native multiplication is heavily optimized and allocation-light.
        if num_a.bits() < 300 {
            return num_a * num_b;
        }

        let (balanced_a, balanced_b, accumulator) = self.balance_pair(num_a, num_b);
        self.solve(balanced_a, balanced_b) + accumulator
    }
}
