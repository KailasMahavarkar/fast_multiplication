use num_bigint::BigUint;
use num_traits::Zero;

#[derive(Clone)]
pub struct MultiplyPowerTwo {
    powers_of_2: Vec<BigUint>,
}

impl MultiplyPowerTwo {
    pub fn new() -> Self {
        let powers_of_2: Vec<BigUint> = (0..128).map(|i| BigUint::from(1u64) << i).collect();
        MultiplyPowerTwo { powers_of_2 }
    }

    // Find the highest set bit position in a BigUint
    #[inline]
    fn find_closest_power_of_two(num: &BigUint) -> u64 {
        let bits = num.bits();
        if bits == 0 {
            0
        } else {
            bits - 1
        }
    }

    #[inline]
    fn find_power(&self, n: u64) -> &BigUint {
        &self.powers_of_2[n as usize]
    }

    pub fn multiply(&self, num_a: &BigUint, num_b: &BigUint) -> BigUint {
        // Early return for zero multiplication
        if num_a.is_zero() || num_b.is_zero() {
            return BigUint::zero();
        }

        let a = Self::find_closest_power_of_two(num_a);
        let c = Self::find_closest_power_of_two(num_b);

        let b = num_a - self.find_power(a);
        let d = num_b - self.find_power(c);

        self.find_power(a) * self.find_power(c) + (b.clone() << c) + (d.clone() << a) + (b * d)
    }
}
