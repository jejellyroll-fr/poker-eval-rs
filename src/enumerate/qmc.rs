//! Quasi-Monte Carlo (QMC) sequence generator.
//!
//! This keeps the historical `SobolGenerator` API name for compatibility,
//! but uses a Halton low-discrepancy sequence internally. The previous
//! implementation was an ad-hoc Sobol placeholder and did not provide
//! mathematically sound multi-dimensional direction vectors.

/// A Sobol sequence generator supporting multiple dimensions.
pub struct SobolGenerator {
    index: u64,
    dimensions: usize,
    bases: Vec<u32>,
}

impl SobolGenerator {
    /// Creates a new Sobol generator with the specified number of dimensions.
    pub fn new(dimensions: usize) -> Self {
        // First primes as Halton bases. This supports up to 16 dimensions which is
        // above our current card draw dimensionality needs.
        const PRIMES: [u32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];
        let capped_dims = dimensions.min(PRIMES.len());
        let mut bases = Vec::with_capacity(capped_dims);
        for base in PRIMES.iter().take(capped_dims) {
            bases.push(*base);
        }

        Self {
            index: 0,
            dimensions: capped_dims,
            bases,
        }
    }

    /// Returns the next D-dimensional point as a vector of floats in [0, 1).
    pub fn next_point(&mut self) -> Vec<f64> {
        self.index = self.index.saturating_add(1);
        let mut point = vec![0.0; self.dimensions];
        for (d, point_item) in point.iter_mut().enumerate().take(self.dimensions) {
            *point_item = radical_inverse(self.index, self.bases[d]);
        }
        point
    }
}

fn radical_inverse(mut n: u64, base: u32) -> f64 {
    let b = base as f64;
    let mut inv = 1.0 / b;
    let mut value = 0.0;

    while n > 0 {
        let digit = (n % base as u64) as f64;
        value += digit * inv;
        n /= base as u64;
        inv /= b;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qmc_distribution_range() {
        let mut gen = SobolGenerator::new(1);
        let mut values = Vec::new();
        for _ in 0..100 {
            values.push(gen.next_point()[0]);
        }

        // Verify values are in [0, 1)
        for &v in &values {
            assert!((0.0..1.0).contains(&v));
        }

        // Roughly check if it's more distributed than random
        // (Just check we didn't get all zeros)
        let sum: f64 = values.iter().sum();
        assert!(sum > 10.0 && sum < 90.0);
    }

    #[test]
    fn test_first_points_dim1() {
        let mut gen = SobolGenerator::new(1);
        let p1 = gen.next_point()[0];
        let p2 = gen.next_point()[0];
        let p3 = gen.next_point()[0];

        assert!((p1 - 0.5).abs() < 1e-12);
        assert!((p2 - 0.25).abs() < 1e-12);
        assert!((p3 - 0.75).abs() < 1e-12);
    }

    #[test]
    fn test_deterministic_sequence() {
        let mut g1 = SobolGenerator::new(3);
        let mut g2 = SobolGenerator::new(3);

        for _ in 0..64 {
            let p1 = g1.next_point();
            let p2 = g2.next_point();
            assert_eq!(p1, p2);
        }
    }
}
