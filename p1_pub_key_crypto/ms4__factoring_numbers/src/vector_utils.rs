pub fn multiply_vector(factors: &Vec<i64>) -> i64 {
    let mut product = 1;
    for num in factors.iter() {
        product *= num;
    }
    product
}

// Print the vector of numbers.
pub fn print_numbers(primes: &Vec<i64>) {
    for prime in primes {
        print!("{} ", prime);
    }
    println!();
}
