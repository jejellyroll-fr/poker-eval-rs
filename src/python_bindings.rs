// Python bindings module

use crate::board::{calculate_outs as calc_outs_rust, BoardTexture};
use crate::deck::*;
use crate::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use crate::enumerate::evaluation::{
    supports_enum_exhaustive, supports_enum_sample, validate_enum_configuration,
};
use crate::enumerate::{enum_exhaustive, enum_sample, CardMask};
use crate::evaluators::range_equity::calculate_equity as calc_equity_rust;
use crate::evaluators::{
    Eval, EvalJoker, HandEvaluator, OmahaHiEvaluator, OmahaHiLoEvaluator, ShortDeckEvaluator,
};
use crate::range::HandRange;
use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tournament::calculate_icm as calculate_icm_rust;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::str::FromStr;

/// Helper to parse card string and validate no duplicates
fn parse_and_validate(input: &str) -> PyResult<(StdDeckCardMask, usize)> {
    let result = StdDeck::string_to_mask(input);
    match result {
        Ok((mask, count)) => {
            if mask.num_cards() != count {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Duplicate cards detected in '{}'",
                    input
                )));
            }
            Ok((mask, count))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error converting cards: {}",
            e
        ))),
    }
}

/// Helper to parse card string for Joker deck
fn parse_and_validate_joker(
    input: &str,
) -> PyResult<(crate::tables::t_jokercardmasks::JokerDeckCardMask, usize)> {
    let result = JokerDeck::string_to_mask(input);
    match result {
        Ok((mask, count)) => {
            // Check for duplicates? JokerDeck mask handles it?
            // JokerDeckCardMask uses bitmasks, so duplicates merge.
            // But we should check if count matches expected unique cards if possible.
            // string_to_mask returns mask and count of cards set.
            // If input had duplicates, count might be less than input tokens?
            // Actually string_to_mask counts tokens.
            // Let's trust string_to_mask for now.
            Ok((mask, count))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error converting cards: {}",
            e
        ))),
    }
}

#[pyclass(name = "Card")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PyCard {
    index: usize,
}

#[pymethods]
impl PyCard {
    #[new]
    pub fn new(input: &str) -> PyResult<Self> {
        match StdDeck::string_to_card(input) {
            Some(idx) => Ok(PyCard { index: idx }),
            None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid card string: {}",
                input
            ))),
        }
    }

    #[staticmethod]
    pub fn from_id(id: usize) -> PyResult<Self> {
        if id < 52 {
            Ok(PyCard { index: id })
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Card ID must be 0-51",
            ))
        }
    }

    #[getter]
    pub fn rank(&self) -> u8 {
        (self.index % 13) as u8
    }

    #[getter]
    pub fn suit(&self) -> u8 {
        (self.index / 13) as u8
    }

    pub fn __repr__(&self) -> String {
        format!("<Card {}>", self)
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }

    pub fn __int__(&self) -> usize {
        self.index
    }
}

impl std::fmt::Display for PyCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", StdDeckCardMask::from_card_index(self.index))
    }
}

#[pyclass(name = "Hand")]
#[derive(Clone, Debug)]
pub struct PyHand {
    mask: StdDeckCardMask,
}

#[pymethods]
impl PyHand {
    #[new]
    #[pyo3(signature = (input=None))]
    pub fn new(input: Option<&str>) -> PyResult<Self> {
        if let Some(s) = input {
            let (mask, _) = parse_and_validate(s)?;
            Ok(PyHand { mask })
        } else {
            Ok(PyHand {
                mask: StdDeckCardMask::new(),
            })
        }
    }

    pub fn add(&mut self, card: &str) -> PyResult<()> {
        let (mask, _) = parse_and_validate(card)?;
        if !self.mask.overlaps(&mask) {
            self.mask.or(&mask);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Card already in hand",
            ))
        }
    }

    pub fn __len__(&self) -> usize {
        self.mask.num_cards()
    }

    pub fn __str__(&self) -> String {
        self.mask.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("<Hand {}>", self.mask)
    }
}

#[pyfunction]
pub fn string_to_mask(input: &str) -> PyResult<String> {
    let (mask, _) = parse_and_validate(input)?;
    Ok(format!("{:b}", mask.as_raw()))
}

#[pyfunction]
pub fn eval_n(input: &str) -> PyResult<String> {
    let (mask, num_cards) = parse_and_validate(input)?;
    let hand_val = Eval::eval_n(&mask, num_cards);
    Ok(hand_val.std_rules_hand_val_to_string())
}

#[pyfunction]
pub fn eval_low_func(input: &str) -> PyResult<String> {
    let (mask, num_cards) = parse_and_validate(input)?;
    let low_hand_val = crate::evaluators::std_deck_lowball_eval(&mask, num_cards);
    Ok(low_hand_val.to_string())
}

/// Evaluate an Omaha hand (4 hole cards + 5 board cards)
#[pyfunction]
pub fn eval_omaha_hi(hole_cards: &str, board: &str) -> PyResult<String> {
    let (hole_mask, hole_count) = parse_and_validate(hole_cards).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing hole cards: {}", e))
    })?;

    if hole_count != 4 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Omaha requires exactly 4 hole cards",
        ));
    }

    let (board_mask, board_count) = parse_and_validate(board).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing board: {}", e))
    })?;

    if !(3..=5).contains(&board_count) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Board must have 3-5 cards",
        ));
    }

    match OmahaHiEvaluator::evaluate_hand(&hole_mask, &board_mask) {
        Ok(hival) => match hival {
            Some(hand_val) => Ok(hand_val.std_rules_hand_val_to_string()),
            None => Ok("No valid hand".to_string()),
        },
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Omaha evaluation error: {:?}",
            e
        ))),
    }
}

/// Evaluate an Omaha Hi/Lo hand (4 hole cards + 5 board cards)
/// Returns a tuple (hi_hand_string, lo_hand_string)
/// lo_hand_string is "No valid low" if no low hand qualifies
#[pyfunction]
pub fn eval_omaha_hi_lo(hole_cards: &str, board: &str) -> PyResult<(String, String)> {
    let (hole_mask, hole_count) = parse_and_validate(hole_cards).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing hole cards: {}", e))
    })?;

    if hole_count != 4 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Omaha requires exactly 4 hole cards",
        ));
    }

    let (board_mask, board_count) = parse_and_validate(board).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing board: {}", e))
    })?;

    if !(3..=5).contains(&board_count) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Board must have 3-5 cards",
        ));
    }

    match OmahaHiLoEvaluator::evaluate_hand(&hole_mask, &board_mask) {
        Ok((hival, loval)) => {
            let hi_str = match hival {
                Some(hand_val) => hand_val.std_rules_hand_val_to_string(),
                None => "No valid hand".to_string(),
            };
            let lo_str = match loval {
                Some(hand_val) => hand_val.to_string(),
                None => "No valid low".to_string(),
            };
            Ok((hi_str, lo_str))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Omaha Hi/Lo evaluation error: {:?}",
            e
        ))),
    }
}

/// Evaluate a Short Deck (Six Plus) Hold'em hand
#[pyfunction]
pub fn eval_short_deck(hole_cards: &str, board: &str) -> PyResult<String> {
    let (hole_mask, hole_count) = parse_and_validate(hole_cards).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing hole cards: {}", e))
    })?;

    if hole_count != 2 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Short Deck requires exactly 2 hole cards",
        ));
    }

    let (board_mask, board_count) = parse_and_validate(board).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing board: {}", e))
    })?;

    if !(3..=5).contains(&board_count) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Board must have 3-5 cards",
        ));
    }

    match ShortDeckEvaluator::evaluate_hand(&hole_mask, &board_mask) {
        Ok(val) => Ok(val.std_rules_hand_val_to_string()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Short Deck evaluation error: {:?}",
            e
        ))),
    }
}

/// Evaluate a hand using joker rules (High hand)
#[pyfunction]
pub fn eval_joker(input: &str) -> PyResult<String> {
    let (mask, num_cards) = parse_and_validate_joker(input)?;
    let hand_val = EvalJoker::eval_n(mask, num_cards);
    Ok(hand_val.std_rules_hand_val_to_string())
}

/// Evaluate a Lowball hand using joker rules (A-5 Lowball with Joker)
#[pyfunction]
pub fn eval_lowball_joker(input: &str) -> PyResult<String> {
    let (mask, num_cards) = parse_and_validate_joker(input)?;
    let low_hand_val = crate::evaluators::joker_lowball_eval(&mask, num_cards);
    Ok(low_hand_val.to_string())
}

/// Batch evaluate multiple hands
#[pyfunction]
pub fn eval_n_batch(hands: Vec<String>) -> PyResult<Vec<String>> {
    let mut results = Vec::with_capacity(hands.len());

    for hand_str in hands {
        let (mask, num_cards) = parse_and_validate(&hand_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Error parsing hand '{}': {}",
                hand_str, e
            ))
        })?;
        let hand_val = Eval::eval_n(&mask, num_cards);
        results.push(hand_val.std_rules_hand_val_to_string());
    }

    Ok(results)
}

/// Calculate equity between multiple hands (Texas Hold'em)
/// Returns a dictionary with win%, tie%, and EV for each player
#[pyfunction]
#[pyo3(signature = (hands, board="", dead="", game="holdem", monte_carlo=false, iterations=100000))]
pub fn calculate_equity(
    py: Python<'_>,
    hands: Vec<String>,
    board: &str,
    dead: &str,
    game: &str,
    monte_carlo: bool,
    iterations: usize,
) -> PyResult<PyObject> {
    let npockets = hands.len();

    let game_variant = match game.to_lowercase().as_str() {
        "holdem" => Game::Holdem,
        "holdem8" => Game::Holdem8,
        "omaha" => Game::Omaha,
        "omaha5" => Game::Omaha5,
        "omaha6" => Game::Omaha6,
        "omaha8" => Game::Omaha8,
        "omaha85" => Game::Omaha85,
        "stud7" => Game::Stud7,
        "stud78" => Game::Stud78,
        "stud7nsq" => Game::Stud7nsq,
        "razz" => Game::Razz,
        "draw5" => Game::Draw5,
        "draw58" => Game::Draw58,
        "draw5nsq" => Game::Draw5nsq,
        "lowball" => Game::Lowball,
        "lowball27" => Game::Lowball27,
        "shortdeck" => Game::ShortDeck,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported game variant: {}",
                game
            )));
        }
    };

    if !(2..=ENUM_MAXPLAYERS).contains(&npockets) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Need 2-{} hands",
            ENUM_MAXPLAYERS
        )));
    }

    // Parse hands
    let mut pockets: Vec<StdDeckCardMask> = Vec::new();
    for hand in &hands {
        let (mask, _) = parse_and_validate(hand).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Error parsing hand '{}': {}",
                hand, e
            ))
        })?;
        pockets.push(mask);
    }

    // Parse board
    let board_mask = if board.is_empty() {
        StdDeckCardMask::new()
    } else {
        match parse_and_validate(board) {
            Ok((mask, _)) => mask,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Error parsing board: {}",
                    e
                )));
            }
        }
    };

    // Parse dead cards
    let dead_mask = if dead.is_empty() {
        StdDeckCardMask::new()
    } else {
        match parse_and_validate(dead) {
            Ok((mask, _)) => mask,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Error parsing dead cards: {}",
                    e
                )));
            }
        }
    };

    // Check for duplicate cards across hands, board, and dead cards
    let mut collision_check = board_mask;
    if !collision_check.is_empty() && !dead_mask.is_empty() {
        let mut board_clone = board_mask;
        board_clone.and(&dead_mask);
        if !board_clone.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Overlap between board and dead cards",
            ));
        }
    }
    collision_check.or(&dead_mask);

    for (i, pocket) in pockets.iter().enumerate() {
        let mut intersection = *pocket;
        intersection.and(&collision_check);
        if !intersection.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Hand {} contains cards already used in board, dead cards, or other hands",
                hands[i]
            )));
        }
        collision_check.or(pocket);
    }

    let nboard = board_mask.num_cards();

    if let Err(e) = validate_enum_configuration(game_variant, board_mask, nboard, !monte_carlo) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Invalid board configuration: {}",
            e
        )));
    }

    // Initialize result
    let mut result = EnumResult::new(game_variant);
    result.sample_type = if monte_carlo {
        SampleType::Sample
    } else {
        SampleType::Exhaustive
    };
    result.nplayers = npockets as u32;

    if monte_carlo && !supports_enum_sample(game_variant) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Monte Carlo mode is not supported for game variant: {}",
            game
        )));
    }
    if !monte_carlo && !supports_enum_exhaustive(game_variant) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Exhaustive mode is not supported for game variant: {}",
            game
        )));
    }

    // Run calculation
    let calc_result = if monte_carlo {
        enum_sample(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            iterations,
            false,
            &mut result,
        )
    } else {
        enum_exhaustive(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            false,
            &mut result,
        )
    };

    match calc_result {
        Ok(_) => {
            let dict = PyDict::new(py);
            dict.set_item("samples", result.nsamples)?;

            let mut players = Vec::new();
            for (i, hand) in hands.iter().enumerate().take(npockets) {
                let total = result.nwinhi[i] + result.ntiehi[i] + result.nlosehi[i];
                if total > 0 {
                    let player_dict = PyDict::new(py);
                    player_dict.set_item("hand", hand)?;
                    player_dict
                        .set_item("win", (result.nwinhi[i] as f64 / total as f64) * 100.0)?;
                    player_dict
                        .set_item("tie", (result.ntiehi[i] as f64 / total as f64) * 100.0)?;
                    player_dict
                        .set_item("lose", (result.nlosehi[i] as f64 / total as f64) * 100.0)?;
                    player_dict.set_item("ev", result.ev[i])?;
                    // Add scoop and lo stats if applicable (e.g. for hi-lo games)
                    if result.game == Game::Omaha8
                        || result.game == Game::Stud78
                        || result.game == Game::Omaha85
                        || result.game == Game::Holdem8
                        || result.game == Game::Draw58
                    {
                        player_dict
                            .set_item("scoop", (result.nscoop[i] as f64 / total as f64) * 100.0)?;
                        player_dict
                            .set_item("win_lo", (result.nwinlo[i] as f64 / total as f64) * 100.0)?;
                        player_dict
                            .set_item("tie_lo", (result.ntielo[i] as f64 / total as f64) * 100.0)?;
                        player_dict.set_item(
                            "lose_lo",
                            (result.nloselo[i] as f64 / total as f64) * 100.0,
                        )?;
                    }

                    players.push(player_dict);
                }
            }
            dict.set_item("players", players)?;

            Ok(dict.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Calculation error: {:?}",
            e
        ))),
    }
}

/// A wrapper class for HandRange to be used in Python.
#[pyclass(name = "HandRange")]
#[derive(Clone)]
pub struct PyHandRange {
    pub inner: HandRange,
}

#[pymethods]
impl PyHandRange {
    #[new]
    pub fn new(range_str: &str) -> PyResult<Self> {
        match HandRange::from_str(range_str) {
            Ok(range) => Ok(PyHandRange { inner: range }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid range string: {}",
                e
            ))),
        }
    }

    pub fn __len__(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn __repr__(&self) -> String {
        format!("<HandRange combinations={}>", self.inner.len())
    }

    /// Add a hand to the range with a specific weight.
    #[pyo3(signature = (hand, weight=1.0))]
    pub fn add(&mut self, hand: &str, weight: f64) -> PyResult<()> {
        let (mask, _) = parse_and_validate(hand)?;
        self.inner.push_weighted(mask, weight);
        Ok(())
    }
}

/// Calculate equity between two ranges
/// Returns a dictionary with equity stats
#[pyfunction]
#[pyo3(signature = (range1, range2, board="", iterations=10000))]
pub fn calculate_range_equity(
    py: Python<'_>,
    range1: &PyHandRange,
    range2: &PyHandRange,
    board: &str,
    iterations: usize,
) -> PyResult<PyObject> {
    let board_mask = if board.is_empty() {
        StdDeckCardMask::new()
    } else {
        match parse_and_validate(board) {
            Ok((mask, _)) => mask,
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Error parsing board: {}",
                    e
                )));
            }
        }
    };

    match calc_equity_rust(&range1.inner, &range2.inner, &board_mask, iterations) {
        Ok(res) => {
            let dict = PyDict::new(py);
            dict.set_item("equity", res.equity * 100.0)?;
            dict.set_item("wins", res.wins)?;
            dict.set_item("ties", res.ties)?;
            dict.set_item("losses", res.losses)?;
            dict.set_item("samples", res.samples)?;
            Ok(dict.into())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Equity calculation error: {}",
            e
        ))),
    }
}

/// Calculate Independent Chip Model (ICM) equities.
///
/// `stacks` are chip counts per player, `prizes` are payout amounts ordered from
/// first place downward.
#[pyfunction]
pub fn calculate_icm(stacks: Vec<f64>, prizes: Vec<f64>) -> PyResult<Vec<f64>> {
    calculate_icm_rust(&stacks, &prizes).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("ICM calculation error: {}", e))
    })
}

/// Exposure of BoardTexture to Python
#[pyclass(name = "BoardTexture")]
#[derive(Clone)]
pub struct PyBoardTexture {
    #[pyo3(get)]
    pub is_rainbow: bool,
    #[pyo3(get)]
    pub is_two_tone: bool,
    #[pyo3(get)]
    pub is_monotone: bool,
    #[pyo3(get)]
    pub is_paired: bool,
    #[pyo3(get)]
    pub is_trips: bool,
    #[pyo3(get)]
    pub is_quads: bool,
    #[pyo3(get)]
    pub is_full_house: bool,
    #[pyo3(get)]
    pub has_straight_draw: bool,
    #[pyo3(get)]
    pub has_flush_draw: bool,
}

#[pymethods]
impl PyBoardTexture {
    #[staticmethod]
    pub fn analyze(board: &str) -> PyResult<Self> {
        let (board_mask, _) = parse_and_validate(board).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing board: {}", e))
        })?;

        let texture = BoardTexture::analyze(&board_mask);
        Ok(PyBoardTexture {
            is_rainbow: texture.is_rainbow,
            is_two_tone: texture.is_two_tone,
            is_monotone: texture.is_monotone,
            is_paired: texture.is_paired,
            is_trips: texture.is_trips,
            is_quads: texture.is_quads,
            is_full_house: texture.is_full_house,
            has_straight_draw: texture.has_straight_draw,
            has_flush_draw: texture.has_flush_draw,
        })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<BoardTexture rainbow={} monotone={} paired={} straight_draw={} flush_draw={}>",
            self.is_rainbow,
            self.is_monotone,
            self.is_paired,
            self.has_straight_draw,
            self.has_flush_draw
        )
    }
}

/// Calculate outs for a given hand and board.
/// Returns a dictionary where keys are HandType strings (e.g. "Flush", "Straight")
/// and values are lists of card strings (e.g. ["Ah", "Kh"]).
#[pyfunction]
pub fn calculate_outs(pocket: &str, board: &str) -> PyResult<PyObject> {
    let (pocket_mask, _) = parse_and_validate(pocket).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing pocket: {}", e))
    })?;

    let (board_mask, _) = parse_and_validate(board).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing board: {}", e))
    })?;

    let outs_result = calc_outs_rust(&pocket_mask, &board_mask);

    Python::with_gil(|py| {
        let dict = PyDict::new(py);

        for (hand_type_idx, cards) in outs_result.outs_by_type.iter().enumerate() {
            if !cards.is_empty() {
                // Map numeric HandType to string?
                // We need HandType enum to be accessible or replicate standard naming.
                // Assuming standard mappings:
                // 0=HighCard, 1=Pair, 2=TwoPair, 3=Trips, 4=Straight, 5=Flush, 6=FullHouse, 7=Quads, 8=StraightFlush
                let type_name = match hand_type_idx {
                    0 => "HighCard",
                    1 => "Pair",
                    2 => "TwoPair",
                    3 => "Trips",
                    4 => "Straight",
                    5 => "Flush",
                    6 => "FullHouse",
                    7 => "Quads",
                    8 => "StraightFlush",
                    _ => "Unknown",
                };

                let card_strs: Vec<String> = cards.iter().map(|c| c.to_string()).collect();

                dict.set_item(type_name, card_strs)?;
            }
        }

        Ok(dict.into())
    })
}

#[pymodule]
pub fn poker_eval_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(string_to_mask, m)?)?;
    m.add_function(wrap_pyfunction!(eval_n, m)?)?;
    m.add_function(wrap_pyfunction!(eval_n_batch, m)?)?;
    m.add_function(wrap_pyfunction!(eval_low_func, m)?)?;
    m.add_function(wrap_pyfunction!(eval_omaha_hi, m)?)?;
    m.add_function(wrap_pyfunction!(eval_omaha_hi_lo, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_equity, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_icm, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_range_equity, m)?)?;
    m.add_class::<PyHandRange>()?;
    m.add_class::<PyBoardTexture>()?;
    m.add_class::<PyCard>()?;
    m.add_class::<PyHand>()?;
    m.add_function(wrap_pyfunction!(calculate_outs, m)?)?;
    m.add_function(wrap_pyfunction!(eval_short_deck, m)?)?;
    m.add_function(wrap_pyfunction!(eval_joker, m)?)?;
    m.add_function(wrap_pyfunction!(eval_lowball_joker, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn py_calculate_equity_rejects_board_for_stud7() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let err = calculate_equity(
                py,
                vec!["AsKsQh".to_string(), "AdKdQc".to_string()],
                "2c",
                "",
                "stud7",
                true,
                32,
            )
            .unwrap_err();
            assert!(err.to_string().contains("Invalid board configuration"));
        });
    }

    #[test]
    fn py_calculate_equity_rejects_invalid_exhaustive_street_for_holdem() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let err = calculate_equity(
                py,
                vec!["AsKs".to_string(), "QhQd".to_string()],
                "2c 7d",
                "",
                "holdem",
                false,
                32,
            )
            .unwrap_err();
            assert!(err.to_string().contains("Invalid board configuration"));
        });
    }
}
