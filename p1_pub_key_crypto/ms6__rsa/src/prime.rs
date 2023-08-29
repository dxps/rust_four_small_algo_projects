use crate::{exponentiation::fast_exp_mod, prng::Prng};

pub fn is_probably_prime(p: i64, num_tests: i64, prng: &mut Prng) -> bool {
    let mut n: i64;
    let mut np: i64;
    for _ in 0..num_tests {
        n = prng.next_i64(2, p);
        np = fast_exp_mod(n as i64, p - 1, p);
        if np != 1 {
            return false;
        }
    }
    true
}

pub fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    let mut p: i64;
    loop {
        p = prng.next_i64(min, max);
        p |= 1; // If p is even, increment it by 1.
        if is_probably_prime(p, num_tests, prng) {
            return p;
        }
    }
}
