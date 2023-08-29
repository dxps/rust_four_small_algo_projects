use std::time::{SystemTime, UNIX_EPOCH};

/// `Prng` is a simple *pseudorandom number generator* implementation.
pub struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    pub fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    /// Get a pseudorandom value in the range [0, 2147483647].
    pub fn next_u32(&mut self) -> u32 {
        // Notes:
        // 1. The two values are taken from gcc reference [here](https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use).
        // 2. `1 << 31` means `2 power 32`.
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    /// Get a pseudorandom value in the range [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    /// Get a pseudorandom value in the range [min, max).
    pub fn next_i64(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i64;
    }
}
