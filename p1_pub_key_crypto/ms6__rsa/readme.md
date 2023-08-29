## Milestone 6. RSA

This is the last milestone of the project and its goal is to implement an RSA encryption and decryption mechanism. This is for learning and demostration purposes, and it must not be considered a cryptographically secure implementation.

It is based on previous milestones outcomes, these being:

-   `gcd` - that finds the greatest common divisor of two integers.
-   `Prng` - a pseudorandom numbers generator.
-   `is_probably_prime` - that tells if a number is probably a prime or not.
-   `fast_exp_mod` - to perform fast exponentiation in a modulus.
-   `find_prime` - to find a prime number within a range.

The RSA implementation needs some additional functions:

-   `totient` - to calculate the totient for primes `p`and `q` numbers.
-   `random_exponent` - to pick a random public exponent.
-   `inverse_mod` - to find the inverse of a number in a modulus.

<br/>

### Usage

Example:

```shell
❯ cargo run
   ...

*** Public ***
Public key modulus:    46634239
Public key exponent e: 11153003

*** Private ***
Primes:     8039, 5801
λ(n):       23310200
d:          8848667

Message (a number in the range 2..46634238):    178945
Ciphertext: 28356683
Plaintext:  178945

Message (a number in the range 2..46634238):    46634238
Ciphertext: 46634238
Plaintext:  46634238

Message (a number in the range 2..46634238):    2
Ciphertext: 41890366
Plaintext:  2

Message (a number in the range 2..46634238):    1
❯
```
