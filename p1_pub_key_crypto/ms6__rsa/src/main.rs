use ms6_rsa::{
    exponentiation::fast_exp_mod,
    get_input::get_i64,
    ms_additions::{inverse_mod, random_exponent, totient},
    prime::find_prime,
    prng::Prng,
};

const NUM_TESTS: i64 = 100;

fn main() {
    let mut prng = Prng::new();

    // Pick two random primes p and q.
    let p = find_prime(&mut prng, 1_000, 10_000, NUM_TESTS);
    let q = find_prime(&mut prng, 1_000, 10_000, NUM_TESTS);

    // Calculate the public key modulus n.
    let n = p * q;

    // Calculate Carmichael's totient function λ(n).
    let λ_n = totient(p, q);

    // Pick a random public key exponent e in the range [3, λ_n)
    // where gcd(e, λ_n) = 1.
    let e = random_exponent(&mut prng, λ_n);

    // Find the inverse of e mod λ_n.
    let d = inverse_mod(e, λ_n);

    // Print out the important values.
    println!();
    println!("*** Public ***");
    println!("Public key modulus:    {}", n);
    println!("Public key exponent e: {}", e);
    println!();
    println!("*** Private ***");
    println!("Primes:     {}, {}", p, q);
    println!("λ(n):       {}", λ_n);
    println!("d:          {}", d);

    loop {
        // Messages can be up to around 10_000_000 depending on the
        // key values. Stick to 7 digits and you'll probably be okay.
        println!();
        let m = get_i64(format!("Message (a number in the range 2..{}):    ", n - 1).as_str());

        if m == 1 {
            return;
        }

        let ciphertext = fast_exp_mod(m, e, n);
        println!("Ciphertext: {}", ciphertext);

        let plaintext = fast_exp_mod(ciphertext, d, n);
        println!("Plaintext:  {}", plaintext);
    }
}
