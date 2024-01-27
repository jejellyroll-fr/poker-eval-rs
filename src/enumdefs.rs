use crate::enumord::EnumOrdering;
use std::ptr::NonNull;

// Constante définissant le nombre maximum de joueurs
pub const ENUM_MAXPLAYERS: usize = 12;

// Énumération des différents variantes de poker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    Holdem,
    Holdem8,
    Omaha,
    Omaha5,
    Omaha6,
    Omaha8,
    Omaha85,
    Stud7,
    Stud78,
    Stud7nsq,
    Razz,
    Draw5,
    Draw58,
    Draw5nsq,
    Lowball,
    Lowball27,
    NumGames,
}

// Structure définissant les paramètres d'un jeu spécifique
pub struct GameParams {
    pub game: Game,     // Type de jeu.
    pub minpocket: i32, // Nombre minimum de cartes en main.
    pub maxpocket: i32, // Nombre maximum de cartes en main.
    pub maxboard: i32,  // Nombre maximum de cartes sur le board.
    pub haslopot: i32,  // Indique si le jeu a un low
    pub hashipot: i32,  // Indique si le jeu a un high
    pub name: String,   // Nom de la variante.
}
// Énumération définissant les types d'échantillonnage pour l'évaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleType {
    Exhaustive, // Évaluation exhaustive de toutes les mains possibles
    Sample,     // Évaluation basée sur un échantillon de mains
}
// Structure stockant les résultats de l'évaluation d'un jeu
pub struct EnumResult {
    pub game: Game,              // Type de jeu évalué
    pub sample_type: SampleType, // Type d'échantillonnage utilisé
    pub nsamples: u32,           // Nombre d'échantillons considérés dans l'évaluation
    pub nplayers: u32,           // Nombre de joueurs dans le jeu
    // Statistiques pour le high (victoires, égalités, défaites)
    pub nwinhi: [u32; ENUM_MAXPLAYERS],
    pub ntiehi: [u32; ENUM_MAXPLAYERS],
    pub nlosehi: [u32; ENUM_MAXPLAYERS],
    // Statistiques pour le low (victoires, égalités, défaites)
    pub nwinlo: [u32; ENUM_MAXPLAYERS],
    pub ntielo: [u32; ENUM_MAXPLAYERS],
    pub nloselo: [u32; ENUM_MAXPLAYERS],
    pub nscoop: [u32; ENUM_MAXPLAYERS], // Nombre de fois où un joueur remporte l'ensemble du pot
    // Répartition des parts pour le high et le low
    pub nsharehi: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
    pub nsharelo: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
    pub nshare: [[[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
    pub ev: [f64; ENUM_MAXPLAYERS], // Équité moyenne de chaque joueur

    // Pointeur nullable sûr vers une structure d'ordre d'énumération
    pub ordering: Option<NonNull<EnumOrdering>>,
}
