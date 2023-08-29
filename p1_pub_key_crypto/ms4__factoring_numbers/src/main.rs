use std::time::Instant;

use ms4_factoring_numbers::{
    find_factors, find_factors_sieve, get_i64, multiply_vector, print_numbers,
    sieve_of_eratosthenes, sieve_to_primes,
};

fn main() {
    // Build a sieve holding values up to 100 million.
    let sieve = sieve_of_eratosthenes(100_000_000);

    let primes = sieve_to_primes(sieve);

    loop {
        let num = get_i64("Num: ");
        if num <= 0 {
            break;
        }

        // Finding the factors using the slow approach.
        let begin1 = Instant::now();
        let factors1 = find_factors(num);
        let duration1 = begin1.elapsed();
        println!("find_factors: {:?} seconds", duration1);
        print_numbers(&factors1);
        println!("Product: {}\n", multiply_vector(&factors1));

        // Finding the factors using the fast approach.
        let begin2 = Instant::now();
        let factors2 = find_factors_sieve(&primes, num);
        let duration2 = begin2.elapsed();
        println!("find_factors_sieve: {:?} seconds", duration2);
        print_numbers(&factors2);
        println!("Product: {}\n", multiply_vector(&factors2));
    }
}
