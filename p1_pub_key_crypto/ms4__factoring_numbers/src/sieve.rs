// Build a sieve of Eratosthenes.
pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = Vec::with_capacity(max + 1);
    is_prime.resize(max + 1, false);
    if max >= 2 {
        is_prime[2] = true;
        for i in (3..=max).step_by(2) {
            is_prime[i] = true;
        }
        for i in (3..(max as f64).sqrt().ceil() as usize).step_by(2) {
            if is_prime[i] == true {
                for j in ((i * i)..=max).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
    }
    is_prime
}

// Print out the primes in the sieve.
pub fn print_sieve(sieve: &mut Vec<bool>) {
    if sieve.len() > 2 {
        print!("2");
    }
    for i in (3..sieve.len()).step_by(2) {
        if sieve[i] {
            print!(" {i}");
        }
    }
    println!()
}

// Convert the sieve into a vector holding prime numbers.
pub fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    let mut primes = vec![];
    if sieve.len() > 2 {
        primes.push(2);
        for i in (3..sieve.len()).step_by(2) {
            if sieve[i] {
                primes.push(i as i64);
            }
        }
    }
    primes
}
