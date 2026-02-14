pub struct Combination {
    /// Number of elements in each combination (k)
    pub nelem: usize,
    /// Total number of combinations (n choose k)
    pub ncombo: usize,
    /// Precomputed vector containing all possible combinations (row-major)
    pub combos: Vec<Vec<usize>>,
}

impl Combination {
    /// Creates a new `Combination` instance for choosing `nelem` items from `nuniv` items.
    ///
    /// Returns `None` if `nelem > nuniv`.
    pub fn new(nuniv: usize, nelem: usize) -> Option<Combination> {
        if nelem > nuniv {
            return None;
        }

        if nelem == 0 {
            return Some(Combination {
                nelem,
                ncombo: 1,
                combos: vec![vec![]],
            });
        }

        let ncombo = (0..nelem).fold(1, |acc, i| acc * (nuniv - i) / (i + 1));
        let mut combos = Vec::with_capacity(ncombo);

        crate::combinations::for_each_combination(nuniv, nelem, |indices| {
            combos.push(indices.to_vec());
        });

        Some(Combination {
            nelem,
            ncombo,
            combos,
        })
    }
    /// Returns the total number of combinations.
    pub fn num_combinations(&self) -> usize {
        self.ncombo
    }
    /// Returns the `cnum`-th combination.
    pub fn get_combination(&self, cnum: usize) -> Option<Vec<usize>> {
        self.combos.get(cnum).cloned()
    }

    /// Returns a reference to the `cnum`-th combination.
    ///
    /// This avoids allocation if the caller only needs to inspect the values.
    pub fn get_combination_ref(&self, cnum: usize) -> Option<&[usize]> {
        self.combos.get(cnum).map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_c52_choose_2() {
        // C(52,2) = 1326 - number of possible starting hands in Hold'em
        let combo = Combination::new(52, 2).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 1326);
    }

    #[test]
    fn test_combination_c52_choose_5() {
        // C(52,5) = 2,598,960 - number of possible 5-card hands
        let combo = Combination::new(52, 5).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 2598960);
    }

    #[test]
    fn test_combination_c5_choose_2() {
        // C(5,2) = 10
        let combo = Combination::new(5, 2).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 10);
    }

    #[test]
    fn test_combination_c4_choose_2() {
        // C(4,2) = 6 - used in Omaha for selecting 2 hole cards
        let combo = Combination::new(4, 2).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 6);
    }

    #[test]
    fn test_combination_c5_choose_3() {
        // C(5,3) = 10 - used in Omaha for selecting 3 board cards
        let combo = Combination::new(5, 3).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 10);
    }

    #[test]
    fn test_combination_get_first() {
        let combo = Combination::new(5, 2).expect("Should create combination");
        let first = combo.get_combination(0).expect("Should get first combo");
        assert_eq!(first, vec![0, 1]);
    }

    #[test]
    fn test_combination_get_last() {
        let combo = Combination::new(5, 2).expect("Should create combination");
        let last = combo.get_combination(9).expect("Should get last combo");
        assert_eq!(last, vec![3, 4]);
    }

    #[test]
    fn test_combination_invalid_nelem_greater_than_nuniv() {
        // nelem > nuniv should return None
        let result = Combination::new(3, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_combination_get_out_of_bounds() {
        let combo = Combination::new(5, 2).expect("Should create combination");
        let result = combo.get_combination(100);
        assert!(result.is_none());
    }

    #[test]
    fn test_combination_edge_case_c1_choose_1() {
        let combo = Combination::new(1, 1).expect("Should create combination");
        assert_eq!(combo.num_combinations(), 1);
        assert_eq!(combo.get_combination(0), Some(vec![0]));
    }
    #[test]
    fn test_combination_sorted() {
        // Combinations should be generated in lexicographical order.
        // For C(5, 2):
        // [0, 1]
        // [0, 2]
        // [0, 3]
        // [0, 4]
        // [1, 2]
        // [1, 3]
        // [1, 4]
        // [2, 3]
        // [2, 4]
        // [3, 4]
        let combo = Combination::new(5, 2).expect("Should create combination");
        let mut prev = combo.get_combination(0).expect("First");

        for i in 1..combo.num_combinations() {
            let next = combo.get_combination(i).expect("Next");
            assert!(
                next > prev,
                "Combinations not sorted: {:?} -> {:?}",
                prev,
                next
            );
            prev = next;
        }
    }
}
