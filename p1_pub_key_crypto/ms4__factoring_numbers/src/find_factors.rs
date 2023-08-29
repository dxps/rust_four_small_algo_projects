// Find the number's prime factors.
pub fn find_factors(mut num: i64) -> Vec<i64> {
    let mut factors = vec![];
    // Take out the 2s.
    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }
    // Take out other primes.
    let mut factor = 3;
    while factor * factor <= num {
        if num % factor == 0 {
            // This is a factor.
            factors.push(factor);
            num /= factor;
        } else {
            factor += 2;
        }
    }
    // If `num` is not 1, then whatever is left is prime.
    if num > 1 {
        factors.push(num);
    }
    factors
}

// This is the faster way if finding prime factors,
// since it loops through the values in the primes vector.
pub fn find_factors_sieve(primes: &Vec<i64>, mut num: i64) -> Vec<i64> {
    let mut factors = vec![];
    let sqrt = (num as f64).sqrt().ceil() as i64;
    for prime in primes.iter() {
        while num % prime == 0 {
            // This is a factor.
            factors.push(*prime);
            num /= prime;
        }
        if *prime > sqrt || num < *prime {
            break;
        }
    }
    if num > 1 {
        factors.push(num);
    }
    factors
}
