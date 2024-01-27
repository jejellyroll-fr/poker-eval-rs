use crate::enumord::EnumOrdering;
use std::ptr::NonNull;

pub const ENUM_MAXPLAYERS: usize = 12;

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

pub struct GameParams {
    pub game: Game,
    pub minpocket: i32,
    pub maxpocket: i32,
    pub maxboard: i32,
    pub haslopot: i32,
    pub hashipot: i32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleType {
    Exhaustive,
    Sample,
}

pub struct EnumResult {
    pub game: Game,
    pub sample_type: SampleType,
    pub nsamples: u32,
    pub nplayers: u32,
    pub nwinhi: [u32; ENUM_MAXPLAYERS], // qualifies for high and wins (no tie)
    pub ntiehi: [u32; ENUM_MAXPLAYERS], // qualifies for high and ties
    pub nlosehi: [u32; ENUM_MAXPLAYERS], // qualifies for high and loses
    pub nwinlo: [u32; ENUM_MAXPLAYERS], // qualifies for low and wins (no tie)
    pub ntielo: [u32; ENUM_MAXPLAYERS], // qualifies for low and ties
    pub nloselo: [u32; ENUM_MAXPLAYERS], // qualifies for low and loses
    pub nscoop: [u32; ENUM_MAXPLAYERS], // wins entire pot
    pub nsharehi: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS], // shares for high hand
    pub nsharelo: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS], // shares for low hand
    pub nshare: [[[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS], // shares for both hands
    pub ev: [f64; ENUM_MAXPLAYERS], // pot equity of player i averaged over all outcomes

    // A safer nullable pointer using NonNull
    pub ordering: Option<NonNull<EnumOrdering>>,
}
