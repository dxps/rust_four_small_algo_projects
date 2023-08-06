use std::io::{self, Write};

use m1_pub_key_crypto::{gcd, lcm};

fn main() {
    println!("\nEnter two positive integers (named below as A and B) to calculate the GCD and LCM of them.");
    let a = get_i64("Enter A: ");
    let b = get_i64("Enter B: ");
    print_gcd_and_lcm_result(a, b)
}

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>().expect("Error parsing integer");
}

fn print_gcd_and_lcm_result(a: i64, b: i64) {
    println!("-----------------------------------------");
    println!("{:>8}{:>8}{:>12}{:>12}", "A", "B", "GCD(A,B)", "LCM(A,B)");
    println!("{:>8}{:>8}{:>12}{:>12}", a, b, gcd(a, b), lcm(a, b));
    println!("-----------------------------------------\n");
}
