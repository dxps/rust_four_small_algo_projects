pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        // Prime numbers are assumed to be positive.
        // If one of the parameters is negative, make it positive before starting the algorightm.
        if a < 0 {
            a = -a;
        }
        if b < 0 {
            b = -b;
        }
        // GCD(A,B) assumes that A is greater than B.
        if b > a {
            (a, b) = (b, a);
            gcd(a, b)
        // Finally, the main logic.
        } else {
            a = a % b;
            gcd(b, a)
        }
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test {

    use crate::{gcd, lcm};

    #[test]
    fn gcd_of_12_18_is_6() {
        let result = gcd(12, 18);
        let expected = 6;
        assert_eq!(result, expected, "testing the GCD of 12 and 18")
    }

    #[test]
    fn lcm_of_12_18_is_36() {
        let result = lcm(12, 18);
        let expected = 36;
        assert_eq!(result, expected, "testing the LCM of 12 and 18")
    }
}
