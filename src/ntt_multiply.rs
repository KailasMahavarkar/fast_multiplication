use num_bigint::BigUint;
use rayon::prelude::*;

// Goldilocks prime: 2^64 - 2^32 + 1. Two-adicity 32, primitive root 7.
const P: u64 = 0xFFFF_FFFF_0000_0001;
const EPSILON: u64 = 0xFFFF_FFFF;
const GENERATOR: u64 = 7;
const THRESHOLD_BITS: u64 = 1 << 15;
const PAR_MIN_POINTS: usize = 1 << 14;
const INNER_CHUNK: usize = 1 << 12;

pub struct NttMultiply;

#[inline]
fn add_mod(a: u64, b: u64) -> u64 {
    let (mut s, carry) = a.overflowing_add(b);
    if carry {
        s = s.wrapping_add(EPSILON);
    }
    if s >= P {
        s -= P;
    }
    s
}

#[inline]
fn sub_mod(a: u64, b: u64) -> u64 {
    let (d, borrow) = a.overflowing_sub(b);
    if borrow {
        d.wrapping_sub(EPSILON)
    } else {
        d
    }
}

#[inline]
fn mul_mod(a: u64, b: u64) -> u64 {
    // 2^64 = 2^32 - 1 and 2^96 = -1 (mod P)
    let x = u128::from(a) * u128::from(b);
    let lo = x as u64;
    let hi = (x >> 64) as u64;
    let hi_hi = hi >> 32;
    let hi_lo = hi & EPSILON;
    let (mut t, borrow) = lo.overflowing_sub(hi_hi);
    if borrow {
        t = t.wrapping_sub(EPSILON);
    }
    add_mod(t, hi_lo * EPSILON)
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut acc = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul_mod(acc, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    acc
}

fn build_twiddles(n: usize, omega: u64) -> Vec<u64> {
    let mut tw = Vec::with_capacity(n / 2);
    let mut w = 1u64;
    for _ in 0..n / 2 {
        tw.push(w);
        w = mul_mod(w, omega);
    }
    tw
}

fn bit_reverse(a: &mut [u64]) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            a.swap(i, j);
        }
    }
}

#[inline]
fn butterfly_span(lo: &mut [u64], hi: &mut [u64], tw: &[u64], base: usize, stride: usize) {
    for j in 0..lo.len() {
        let t = mul_mod(tw[(base + j) * stride], hi[j]);
        let u = lo[j];
        lo[j] = add_mod(u, t);
        hi[j] = sub_mod(u, t);
    }
}

fn ntt(a: &mut [u64], tw: &[u64]) {
    let n = a.len();
    bit_reverse(a);
    let parallel = n >= PAR_MIN_POINTS;
    let mut m = 1usize;
    while m < n {
        let stride = n / (2 * m);
        if parallel {
            a.par_chunks_mut(2 * m).for_each(|chunk| {
                let (lo, hi) = chunk.split_at_mut(m);
                if m >= 2 * INNER_CHUNK {
                    lo.par_chunks_mut(INNER_CHUNK)
                        .zip(hi.par_chunks_mut(INNER_CHUNK))
                        .enumerate()
                        .for_each(|(ci, (lc, hc))| {
                            butterfly_span(lc, hc, tw, ci * INNER_CHUNK, stride);
                        });
                } else {
                    butterfly_span(lo, hi, tw, 0, stride);
                }
            });
        } else {
            for chunk in a.chunks_mut(2 * m) {
                let (lo, hi) = chunk.split_at_mut(m);
                butterfly_span(lo, hi, tw, 0, stride);
            }
        }
        m *= 2;
    }
}

fn to_u16_digits(num: &BigUint) -> Vec<u64> {
    let digits = num.to_u32_digits();
    let mut out = Vec::with_capacity(digits.len() * 2);
    for d in digits {
        out.push(u64::from(d & 0xFFFF));
        out.push(u64::from(d >> 16));
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}

fn from_u16_carries(coeffs: &[u64]) -> BigUint {
    let mut bytes = Vec::with_capacity(coeffs.len() * 2 + 8);
    let mut carry = 0u64;
    for &c in coeffs {
        let acc = carry + c;
        bytes.push((acc & 0xFF) as u8);
        bytes.push(((acc >> 8) & 0xFF) as u8);
        carry = acc >> 16;
    }
    while carry > 0 {
        bytes.push((carry & 0xFF) as u8);
        carry >>= 8;
    }
    BigUint::from_bytes_le(&bytes)
}

impl NttMultiply {
    pub fn multiply(a: &BigUint, b: &BigUint) -> BigUint {
        if a.bits().min(b.bits()) < THRESHOLD_BITS {
            return a * b;
        }
        let da = to_u16_digits(a);
        let db = to_u16_digits(b);
        let n = (da.len() + db.len()).next_power_of_two();

        let omega = pow_mod(GENERATOR, (P - 1) / n as u64);
        let tw = build_twiddles(n, omega);
        let mut fa = da;
        fa.resize(n, 0);
        let mut fb = db;
        fb.resize(n, 0);
        rayon::join(|| ntt(&mut fa, &tw), || ntt(&mut fb, &tw));
        let n_inv = pow_mod(n as u64, P - 2);
        if n >= PAR_MIN_POINTS {
            fa.par_iter_mut().zip(fb.par_iter()).for_each(|(x, y)| {
                *x = mul_mod(*x, *y);
            });
        } else {
            for i in 0..n {
                fa[i] = mul_mod(fa[i], fb[i]);
            }
        }
        let tw_inv = build_twiddles(n, pow_mod(omega, P - 2));
        ntt(&mut fa, &tw_inv);
        if n >= PAR_MIN_POINTS {
            fa.par_iter_mut().for_each(|v| *v = mul_mod(*v, n_inv));
        } else {
            for v in fa.iter_mut() {
                *v = mul_mod(*v, n_inv);
            }
        }
        from_u16_carries(&fa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Zero;
    use rand::Rng;

    fn rand_biguint(rng: &mut impl Rng, bits: u64) -> BigUint {
        let bytes = (bits as usize + 7) / 8;
        let v: Vec<u8> = (0..bytes).map(|_| rng.gen()).collect();
        BigUint::from_bytes_le(&v)
    }

    #[test]
    fn mul_mod_matches_u128_reference() {
        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let a = rng.gen::<u64>() % P;
            let b = rng.gen::<u64>() % P;
            let expect = ((u128::from(a) * u128::from(b)) % u128::from(P)) as u64;
            assert_eq!(mul_mod(a, b), expect, "a={a} b={b}");
        }
    }

    #[test]
    fn add_sub_mod_match_u128_reference() {
        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let a = rng.gen::<u64>() % P;
            let b = rng.gen::<u64>() % P;
            assert_eq!(
                add_mod(a, b),
                ((u128::from(a) + u128::from(b)) % u128::from(P)) as u64
            );
            assert_eq!(
                sub_mod(a, b),
                ((u128::from(P) + u128::from(a) - u128::from(b)) % u128::from(P)) as u64
            );
        }
    }

    #[test]
    fn ntt_roundtrip_is_identity() {
        let mut rng = rand::thread_rng();
        let n = 1024;
        let original: Vec<u64> = (0..n).map(|_| rng.gen::<u64>() % P).collect();
        let mut a = original.clone();
        let omega = pow_mod(GENERATOR, (P - 1) / n as u64);
        ntt(&mut a, &build_twiddles(n, omega));
        ntt(&mut a, &build_twiddles(n, pow_mod(omega, P - 2)));
        let n_inv = pow_mod(n as u64, P - 2);
        for v in a.iter_mut() {
            *v = mul_mod(*v, n_inv);
        }
        assert_eq!(a, original);
    }

    #[test]
    fn multiply_matches_native_above_threshold() {
        let mut rng = rand::thread_rng();
        for &(abits, bbits) in &[(40_000, 40_000), (100_000, 90_000), (200_000, 40_000)] {
            let a = rand_biguint(&mut rng, abits);
            let b = rand_biguint(&mut rng, bbits);
            assert_eq!(NttMultiply::multiply(&a, &b), &a * &b, "{abits}x{bbits}");
        }
    }

    #[test]
    fn multiply_matches_native_below_threshold() {
        let mut rng = rand::thread_rng();
        let a = rand_biguint(&mut rng, 1000);
        let b = rand_biguint(&mut rng, 900);
        assert_eq!(NttMultiply::multiply(&a, &b), &a * &b);
    }

    #[test]
    fn multiply_handles_zero_and_powers_of_two() {
        let mut rng = rand::thread_rng();
        let z = BigUint::zero();
        let x = rand_biguint(&mut rng, 50_000);
        assert_eq!(NttMultiply::multiply(&x, &z), z);
        let p2 = BigUint::from(1u32) << 60_000u32;
        assert_eq!(NttMultiply::multiply(&x, &p2), &x * &p2);
    }
}
