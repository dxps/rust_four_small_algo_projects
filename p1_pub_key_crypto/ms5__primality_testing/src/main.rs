use ms5_primality_testing::{prime::find_prime, prng::Prng, prompt::get_i64};

const NUM_TESTS: i64 = 20;

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime,
    // if it passes all NUM_TESTS tests.
    let probability = (1.0 - (0.5 as f64).powf(NUM_TESTS as f64)) * 100.0;
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 {
            break;
        }

        // Calculate minimum and maximum values.
        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
    }
}
