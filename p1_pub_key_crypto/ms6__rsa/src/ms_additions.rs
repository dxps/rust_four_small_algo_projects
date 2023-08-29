use crate::gcd_lcm::lcm;
use crate::{gcd_lcm::gcd, prng::Prng};

/// Calculate Carmichael's totient function λ(n)
/// where n = p * q and p and q are prime numbers.
pub fn totient(p: i64, q: i64) -> i64 {
    lcm(p - 1, q - 1)
}

///! The additional functions that this milestones brings
///! on top of what has been implemented (and just added here)
///! in the previous milestones.

/// Pick a random exponent e in the range [3, λ_n) such that gcd(e, tn) = 1.
/// Provided `tn` means `λ(n)` that is Carmichael's totient function.
pub fn random_exponent(prng: &mut Prng, tn: i64) -> i64 {
    loop {
        let e = prng.next_i64(3, tn);
        if gcd(e, tn) == 1 {
            return e;
        }
    }
}

/// Calculate the inverse of a in the modulus.<br/>
/// For further details, check out the following sections in [this](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers) wikipedia page:
/// - Section "Computing multiplicative inverses in modular structures"
/// - Subsection "Modular integers"
pub fn inverse_mod(a: i64, modulus: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = modulus;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }

    if r > 1 {
        return -1;
    }

    if t < 0 {
        t = t + modulus;
    }
    return t;
}
