pub struct Combination {
    pub nelem: usize,            // Nombre d'éléments dans chaque combinaison
    pub ncombo: usize,           // Nombre total de combinaisons
    pub combos: Vec<Vec<usize>>, // Vecteur contenant toutes les combinaisons possibles
}

impl Combination {
    // Constructeur pour initialiser une nouvelle instance de `Combination`.
    pub fn new(nuniv: usize, nelem: usize) -> Option<Combination> {
        if nelem > nuniv {
            return None;
        }

        let ncombo = (0..nelem).fold(1, |acc, i| acc * (nuniv - i) / (i + 1));
        let mut combos = vec![vec![0; ncombo]; nelem];

        for i in 0..nelem {
            combos[i][0] = i;
        }

        for j in 1..ncombo {
            let mut first_incr = None;

            for i in (0..nelem).rev() {
                if combos[i][j - 1] + 1 <= nuniv - (nelem - i) {
                    combos[i][j] = combos[i][j - 1] + 1;
                    first_incr = Some(i);
                    break;
                }
            }

            let first_incr = first_incr?;
            for i in 0..first_incr {
                combos[i][j] = combos[i][j - 1];
            }
            for i in first_incr + 1..nelem {
                combos[i][j] = combos[i - 1][j] + 1;
            }
        }

        Some(Combination {
            nelem,
            ncombo,
            combos,
        })
    }
    // Retourne le nombre total de combinaisons.
    pub fn num_combinations(&self) -> usize {
        self.ncombo
    }
    // Retourne une combinaison spécifique basée sur son index.
    pub fn get_combination(&self, cnum: usize) -> Option<Vec<usize>> {
        if cnum >= self.ncombo {
            return None;
        }
        Some(self.combos.iter().map(|combo| combo[cnum]).collect())
    }
}
