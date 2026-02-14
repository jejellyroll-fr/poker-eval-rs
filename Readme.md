# PokerEval-RS

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build Status](https://github.com/jejellyroll-fr/poker-eval-rs/workflows/Rust%20CI/badge.svg)

**PokerEval-RS** is a high-performance, safety-first poker hand evaluation library written in Rust. It is a modern, idiomatic port of the concepts found in the `poker-eval` and `OMPEval` libraries, designed for extreme speed and portability.

It supports **Texas Hold'em**, **Omaha (4, 5, 6 cards)**, **Stud**, **Razz**, **Lowball (2-7 & A-5)**, and **Short Deck**.

---

## 🚀 Features

- **Blazing Fast**: Evaluates Hold'em hands in ~1-3 nanoseconds depending on the chosen lookup table.
- **Multi-Platform**: First-class support for **Rust**, **Python** (via PyO3), and **WebAssembly**.
- **Flexible Lookups**: Choose between a 32KB cache-friendly table or a 65MB high-throughput table.
- **SIMD Optimized**: Batch evaluation using AVX2 for massive throughput.
- **Parallelism**: Thread-safe equity calculations powered by `rayon`.

---

## 🛠 Usage

### 🦀 Rust

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
poker-eval-rs = { version = "0.1.0", features = ["parallel", "compact-table"] }
```

#### Basic Hand Evaluation
```rust
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::evaluators::{Eval, HandEvaluator};

fn main() {
    // Evaluation returns a HandVal which can be compared or displayed
    let (mask, count) = StdDeck::string_to_mask("As Ks Qs Js Ts").unwrap();
    let val = Eval::eval_n(&mask, count);
    
    println!("Type: {}", val.hand_type()); // 8 (Straight Flush)
    println!("Description: {}", val);      // "Straight Flush, Ace-high"
}
```

#### Feature Flags
- `compact-table` (Default): Uses a 32KB table with a perfect hash. Fits in L1/L2 cache.
- `large-table`: Use a ~65MB sparse table for raw indexing speed (style OMPEval). (~1.8x faster single-thread).
- `simd`: Enables `eval_8_hands` (AVX2) for batch processing.
- `parallel`: Enables multi-threaded equity calculations using `rayon`.

---

### 🐍 Python

Build and install the bindings using [maturin](https://github.com/PyO3/maturin):

```bash
pip install maturin
maturin develop --release --features python
```

#### Example
```python
import poker_eval_rs

# Standard High evaluation
rank = poker_eval_rs.eval_n("As Ks Qs Js Ts")
print(f"Hand: {rank}") # "Straight Flush"

# Equity: AA vs KK preflop
res = poker_eval_rs.calculate_equity(
    hands=["AsAd", "KsKd"],
    board="",
    game="holdem",
    monte_carlo=False
)
print(f"AA Win Probability: {res['players'][0]['win_hi']}%")
```

---

### 🕸 WebAssembly

Compile for the web or Node.js using `wasm-pack`:

```bash
wasm-pack build -- --features wasm
```

#### Javascript Usage
```javascript
import * as poker from "poker-eval-rs";

const result = poker.eval_omaha_hi("AsKs2d3d", "4s5s6s");
console.log(`Omaha Hand: ${result}`);

const equity = poker.calculate_equity("AsKs QdJd", "", "", "holdem", true, 10000);
console.log(`P1 Equity: ${equity.players[0].win_pct}%`);
```

---

## ⚡ Technical Choices & Inspirations

### The OMPEval Legacy
This library's high-speed evaluation is inspired by **OMPEval**. When the `large-table` feature is enabled, the library generates a 65MB lookup table using the exact same multiplier constants and indexing logic as OMPEval, allowing for O(1) evaluation with minimal CPU cycles.

### Perfect Hashing (Compact Mode)
To support memory-constrained environments (like mobile or WebAssembly), we implemented a **Compact Mode**. It uses a tiny 32KB table combined with a custom perfect hash function. This fits entirely in the CPU's L1 or L2 cache, offering a great balance between memory footprint and speed (~3.7ns vs ~2.0ns for the large table).

### Batch SIMD Evaluation
For scenarios involving millions of evaluations (simulations, solvers), `poker-eval-rs` provides AVX2-accelerated functions that evaluate 8 hands in parallel, drastically reducing the overhead for non-flush hands.

---

## 🤝 Contributing

Contributions are welcome! Please run tests and clippy before submitting a PR:

```bash
cargo test --all-features
cargo clippy --all-targets --all-features
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
