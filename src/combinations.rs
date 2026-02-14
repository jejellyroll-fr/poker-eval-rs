/// Iterator for combinations.
///
/// # Deprecated
/// Use `for_each_combination` to avoid allocations on every iteration.
#[deprecated(
    since = "0.2.0",
    note = "Use for_each_combination to avoid allocations"
)]
pub struct Combinations {
    n: usize,
    k: usize,
    indices: Vec<usize>,
    first: bool,
}

#[allow(deprecated)]
impl Combinations {
    pub fn new(n: usize, k: usize) -> Self {
        let indices: Vec<usize> = (0..k).collect();
        Self {
            n,
            k,
            indices,
            first: true,
        }
    }
}

#[allow(deprecated)]
impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.n {
            return None;
        }

        if self.first {
            self.first = false;
            return Some(self.indices.clone());
        }

        let mut i = self.k;
        while i > 0 {
            i -= 1;
            if self.indices[i] < self.n - self.k + i {
                self.indices[i] += 1;
                for j in i + 1..self.k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(self.indices.clone());
            }
        }

        None
    }
}

#[deprecated(
    since = "0.2.0",
    note = "Use for_each_combination to avoid allocations"
)]
#[allow(deprecated)]
pub fn combinations(n: usize, k: usize) -> Combinations {
    Combinations::new(n, k)
}

/// Generates combinations of k elements from n, calling `f` with a reference to the current combination.
/// This avoids allocations for the combination vector on every step.
pub fn for_each_combination<F>(n: usize, k: usize, mut f: F)
where
    F: FnMut(&[usize]),
{
    if k > n {
        return;
    }
    if k == 0 {
        f(&[]);
        return;
    }

    let mut indices: Vec<usize> = (0..k).collect();
    // First combination
    f(&indices);

    loop {
        let mut i = k;
        let mut found = false;
        while i > 0 {
            i -= 1;
            if indices[i] < n - k + i {
                indices[i] += 1;
                for j in i + 1..k {
                    indices[j] = indices[j - 1] + 1;
                }
                f(&indices);
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_for_each_combination_3_2() {
        let mut count = 0;
        let mut combinations = Vec::new();
        for_each_combination(3, 2, |c| {
            count += 1;
            combinations.push(c.to_vec());
        });
        assert_eq!(count, 3);
        assert_eq!(combinations, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
    }

    #[test]
    fn test_for_each_combination_4_2() {
        let mut count = 0;
        let mut combinations = Vec::new();
        for_each_combination(4, 2, |c| {
            count += 1;
            combinations.push(c.to_vec());
        });
        assert_eq!(count, 6);
        assert_eq!(
            combinations,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3]
            ]
        );
    }

    #[test]
    fn test_combinations_3_2() {
        let mut combs = combinations(3, 2);
        assert_eq!(combs.next(), Some(vec![0, 1]));
        assert_eq!(combs.next(), Some(vec![0, 2]));
        assert_eq!(combs.next(), Some(vec![1, 2]));
        assert_eq!(combs.next(), None);
    }

    #[test]
    fn test_combinations_4_2() {
        let mut combs = combinations(4, 2);
        assert_eq!(combs.next(), Some(vec![0, 1]));
        assert_eq!(combs.next(), Some(vec![0, 2]));
        assert_eq!(combs.next(), Some(vec![0, 3]));
        assert_eq!(combs.next(), Some(vec![1, 2]));
        assert_eq!(combs.next(), Some(vec![1, 3]));
        assert_eq!(combs.next(), Some(vec![2, 3]));
        assert_eq!(combs.next(), None);
    }
}
