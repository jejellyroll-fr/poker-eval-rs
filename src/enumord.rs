// Importations nécessaires
use crate::handval::HandVal;
use std::cmp::Ordering;

// Constantes pour la limite des joueurs
const ENUM_ORDERING_MAXPLAYERS: usize = 7;
const ENUM_ORDERING_MAXPLAYERS_HILO: usize = 5;

// Enum pour les modes d'ordre final des mains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumOrderingMode {
    None = 0,
    Hi,
    Lo,
    Hilo,
}

// Structure pour le suivi des ordres des mains
pub struct EnumOrdering {
    pub mode: EnumOrderingMode,
    nplayers: usize,
    nentries: usize,
    hist: Vec<u32>,
}

// Tableau de bits pour les rangs des joueurs
static ENUM_NBITS: [i32; ENUM_ORDERING_MAXPLAYERS + 1] = [0, 1, 2, 2, 3, 3, 3, 3];

// Structure pour aider à classer les mains
struct EnumRankelem {
    index: usize,
    handval: i32,
}

impl Ord for EnumRankelem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.handval.cmp(&other.handval)
    }
}

impl PartialOrd for EnumRankelem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EnumRankelem {
    fn eq(&self, other: &Self) -> bool {
        self.handval == other.handval
    }
}

impl Eq for EnumRankelem {}

// Fonction pour calculer le rang des mains
pub fn enum_ordering_rank(
    hands: &mut [HandVal],
    noqual: HandVal,
    nplayers: usize,
    ranks: &mut [i32],
    reverse: bool,
) {
    // Création d'une structure intermédiaire pour le tri
    let mut elems: Vec<(usize, HandVal)> = hands
        .iter()
        .enumerate()
        .map(|(index, handval)| (index, *handval))
        .collect();

    // Tri des mains en fonction de leur valeur, en ordre croissant ou décroissant
    if reverse {
        elems.sort_by(|a, b| b.1.value.cmp(&a.1.value)); // Tri en ordre décroissant si reverse est vrai
    } else {
        elems.sort_by(|a, b| a.1.value.cmp(&b.1.value)); // Tri en ordre croissant sinon
    }

    // Attribuer des rangs en fonction du tri
    let mut currank = 0;
    let mut lastval = elems[0].1.value;
    for &(index, ref handval) in &elems {
        if handval.value != lastval {
            currank += 1;
            lastval = handval.value;
        }
        if handval.value == noqual.value {
            ranks[index] = nplayers as i32; // Rang pour no qualifier
        } else {
            ranks[index] = currank;
        }
    }
}

// Fonction pour encoder les rangs en un seul entier
fn enum_ordering_encode(nplayers: usize, ranks: &[i32]) -> i32 {
    let mut encoding = 0;
    let nbits = ENUM_NBITS[nplayers];
    for &rank in ranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    encoding
}

// Fonction pour encoder les rangs high/low en un seul entier
fn enum_ordering_encode_hilo(nplayers: usize, hiranks: &[i32], loranks: &[i32]) -> i32 {
    let mut encoding = 0;
    let nbits = ENUM_NBITS[nplayers];
    for &rank in hiranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    for &rank in loranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    encoding
}

//
// Fonction pour décoder le rang d'un joueur à partir de l'encodage
fn enum_ordering_decode_k(encoding: i32, nplayers: usize, k: usize) -> i32 {
    let nbits = ENUM_NBITS[nplayers];
    let shift = (nplayers - k - 1) * (nbits as usize);
    (encoding >> shift) & ((1 << nbits) - 1)
}

// Fonction pour calculer le nombre d'entrées dans l'histogramme
fn enum_ordering_nentries(nplayers: usize) -> i32 {
    if nplayers > ENUM_ORDERING_MAXPLAYERS || ENUM_NBITS[nplayers] < 0 {
        -1
    } else {
        1 << (nplayers * (ENUM_NBITS[nplayers] as usize))
    }
}

// Fonction pour calculer le nombre d'entrées dans l'histogramme pour les jeux high/low
fn enum_ordering_nentries_hilo(nplayers: usize) -> i32 {
    if nplayers > ENUM_ORDERING_MAXPLAYERS_HILO || ENUM_NBITS[nplayers] < 0 {
        -1
    } else {
        1 << (2 * nplayers * (ENUM_NBITS[nplayers] as usize))
    }
}

// Fonction pour incrémenter la valeur d'une entrée spécifique de l'histogramme
fn enum_ordering_increment(ordering: &mut EnumOrdering, ranks: &[i32]) {
    let encoding = enum_ordering_encode(ordering.nplayers, ranks);
    ordering.hist[encoding as usize] += 1;
}

// Fonction pour incrémenter la valeur d'une entrée spécifique de l'histogramme pour les jeux high/low
fn enum_ordering_increment_hilo(ordering: &mut EnumOrdering, hiranks: &[i32], loranks: &[i32]) {
    let encoding = enum_ordering_encode_hilo(ordering.nplayers, hiranks, loranks);
    ordering.hist[encoding as usize] += 1;
}

// Implémentation de EnumOrdering
impl EnumOrdering {
    pub fn new(mode: EnumOrderingMode, nplayers: usize) -> Self {
        let nentries = match mode {
            EnumOrderingMode::Hilo => enum_ordering_nentries_hilo(nplayers) as usize,
            _ => enum_ordering_nentries(nplayers) as usize,
        };
        EnumOrdering {
            mode,
            nplayers,
            nentries,
            hist: vec![0; nentries],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test pour vérifier la création d'un objet EnumOrdering
    #[test]
    fn test_enum_ordering_new() {
        let ordering = EnumOrdering::new(EnumOrderingMode::Hi, 5);
        assert_eq!(ordering.mode, EnumOrderingMode::Hi);
        assert_eq!(ordering.nplayers, 5);
    }

    // Test pour la fonction enum_ordering_rank
    #[test]
    fn test_enum_ordering_rank() {
        let mut hands = vec![
            HandVal { value: 3 }, // Main 1
            HandVal { value: 5 }, // Main 2
            HandVal { value: 2 }, // Main 3
        ];
        let noqual = HandVal { value: 0 };
        let nplayers = 3;
        let mut ranks = vec![0; nplayers];

        enum_ordering_rank(&mut hands, noqual, nplayers, &mut ranks, false);

        assert_eq!(ranks, vec![1, 2, 0]); // Les rangs attendus après le tri
    }

    // Test pour enum_ordering_encode
    #[test]
    fn test_enum_ordering_encode() {
        let ranks = vec![1, 2, 0];
        let nplayers = 3;
        let encoded = enum_ordering_encode(nplayers, &ranks);

        assert_eq!(encoded, 24); // La valeur encodée attendue
    }

    // Test pour enum_ordering_decode_k
    #[test]
    fn test_enum_ordering_decode_k() {
        let encoded = 9; // Encodage de [1, 2, 0]
        let nplayers = 3;
        let rank = enum_ordering_decode_k(encoded, nplayers, 1); // Décodage du 2e rang

        assert_eq!(rank, 2); // Rang attendu
    }
}
