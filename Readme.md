# PokerEval-RS

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build Status](https://github.com/yourusername/poker-eval-rs/workflows/Rust%20CI/badge.svg)

**PokerEval-RS** is a high-performance, safety-first poker hand evaluation library written in Rust. It is a modern, idiomatic port of the legendary C `poker_eval` library, designed for speed and correctness.

It supports **Texas Hold'em**, **Omaha**, **Omaha Hi/Lo**, **Stud**, **Razz**, **Lowball (2-7 & A-5)**, and **Short Deck**.

## 🚀 Features

- **Blazing Fast**: Stack-allocated memory, bitwise operations, and optimized lookups.
- **Verification**: Extensive test suite including property-based testing and regression checks against golden data.
- **Multi-Game Support**: Evaluate hands for virtually any poker variant.
- **Python Bindings**: First-class Python support via `PyO3`.
- **Equity Calculation**: Monte Carlo and exhaustive enumeration engines.

## 📦 Installation

### Rust

Add this to your `Cargo.toml`:

```toml
[dependencies]
poker-eval-rs = "0.1.0"
```

### Python

You can build and install the bindings using `maturin`:

```bash
# Requires Rust toolchain installed
pip install maturin
maturin develop --release --features python
```

## 🛠 Usage

### Rust Evaluator

```rust
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::evaluators::{HoldemEvaluator, OmahaHiEvaluator, HandEvaluator};

fn main() {
    // Texas Hold'em (Royal Flush)
    let (hole, _) = StdDeck::string_to_mask("AsKs").unwrap();
    let (board, _) = StdDeck::string_to_mask("QsJsTs").unwrap();
    let val = HoldemEvaluator::evaluate_hand(&hole, &board).unwrap();
    println!("Hold'em Val: {}", val); // Full display string

    // Omaha Hi (Quads)
    let (omaha_hole, _) = StdDeck::string_to_mask("AsKs2h2d").unwrap();
    let (omaha_board, _) = StdDeck::string_to_mask("AcAhAd").unwrap();
    let omaha_val = OmahaHiEvaluator::evaluate_hand(&omaha_hole, &omaha_board).unwrap().unwrap();
    println!("Omaha Val: {}", omaha_val);
}
```

### Python Bindings

```python
import poker_eval_rs

# Evaluate a single hand
hand_rank = poker_eval_rs.eval_n("As Ks Qs Js Ts")
print(f"Hand Rank: {hand_rank}")

# Omaha Hi/Lo
hi, lo = poker_eval_rs.eval_omaha_hi_lo("As 2s 3d 4d", "5s 6s 7s 8d 9d")
print(f"Hi: {hi}, Lo: {lo}")

# Equity Calculator
result = poker_eval_rs.calculate_equity(
    hands=["AsKs", "2h2d"],
    board="",
    game="holdem",
    monte_carlo=True,
    iterations=100000
)

print(f"Win Probability: {result['players'][0]['win']}%")
```

## ⚡ Performance

PokerEval-RS utilizes pre-computed lookup tables and efficient bitmask operations.
- **Zero Allocation** during evaluation hot paths.
- **Rayon** support for parallel equity calculation (in progress).

## 🤝 Contributing

Contributions are welcome! Please run tests and formatting before submitting a PR:

```bash
cargo test
cargo fmt
cargo clippy
```

## 📜 License

This project is licensed under the MIT License.