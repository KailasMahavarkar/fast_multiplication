extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct BalanceMultiply;

impl BalanceMultiply {
    pub fn balance_pair(
        &self,
        mut num_a: BigUint,
        mut num_b: BigUint,
    ) -> (BigUint, BigUint, BigUint) {
        let mut accumulator = BigUint::zero();

        while !num_a.is_zero() && !num_b.is_zero() {
            if num_a < num_b {
                std::mem::swap(&mut num_a, &mut num_b);
            }

            if num_a.is_zero() || num_b.is_zero() {
                return (num_a, num_b, accumulator);
            }

            let num_a_is_odd = &num_a & BigUint::one() != BigUint::zero();
            let num_b_is_odd = &num_b & BigUint::one() != BigUint::zero();

            if num_a_is_odd && num_b_is_odd {
                if &num_a / &num_b > BigUint::one() {
                    accumulator += (&num_a - BigUint::one()) + &num_b;
                    num_a = (&num_a - BigUint::one()) >> 1;
                    num_b = (&num_b - BigUint::one()) << 1;
                    continue;
                }
            }

            if !num_a_is_odd && !num_b_is_odd {
                if &num_a / &num_b > BigUint::one() {
                    num_a >>= 1;
                    num_b <<= 1;
                    continue;
                }
            }

            if num_a_is_odd && !num_b_is_odd {
                if &num_a / &num_b > BigUint::one() {
                    accumulator += &num_b;
                    num_a = (&num_a - BigUint::one()) >> 1;
                    num_b <<= 1;
                    continue;
                }
            }

            if &num_a / &num_b > BigUint::one() {
                accumulator += &num_a;
                num_a >>= 1;
                num_b = (&num_b - BigUint::one()) << 1;
                continue;
            }

            break;
        }

        (num_a, num_b, accumulator)
    }

    pub fn square(&self, num: &BigUint) -> BigUint {
        let mut pow = 0u64;
        let mut res = BigUint::zero();
        let mut tmp = num.clone();

        while !tmp.is_zero() {
            if &tmp & BigUint::one() != BigUint::zero() {
                res += num << pow;
            }
            pow += 1;
            tmp >>= 1;
        }

        res
    }

    pub fn solve(&self, mut num_a: BigUint, mut num_b: BigUint) -> BigUint {
        if num_a.is_zero() || num_b.is_zero() {
            return BigUint::zero();
        }

        let mut result = BigUint::zero();

        if num_a < num_b {
            std::mem::swap(&mut num_a, &mut num_b);
        }

        while num_b > BigUint::zero() {
            let quotient = &num_a / &num_b;
            result += &quotient * self.square(&num_b);
            let remainder = &num_a % &num_b;
            num_a = num_b;
            num_b = remainder;
        }

        result
    }

    pub fn multiply(&self, num_a: BigUint, num_b: BigUint) -> BigUint {
        let (balanced_a, balanced_b, accumulator) = self.balance_pair(num_a, num_b);
        self.solve(balanced_a, balanced_b) + accumulator
    }
}
