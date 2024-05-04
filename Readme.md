## PokerEval-RS
## Overview

PokerEval-RS is a Rust-based library for evaluating poker hands efficiently. This library is a Rust conversion of the original poker_eval library written in C. It provides functionalities to evaluate poker hands, compare them, and perform various card-related operations in the context of poker games.
## Features

- Evaluation of poker hands
- Comparison of different poker hands to determine winners
- Utility functions for card manipulation and deck management
- Rust-based implementation ensuring safety and performance

## Getting Started
### Prerequisites

- Rust programming language setup
- Cargo, Rust's package manager

### Installation

Clone the repository to your local machine:

bash
```
git clone https://github.com/yourusername/poker-eval-rs.git
```
Navigate to the cloned directory:

WARNING:
don't forget to setup your env PYO3_PYTHON to point to your python interpreter

bash
```
cd poker-eval-rs
```
Build the project using Cargo:

bash
```
cargo build
```

## examples
for python example to evaluate hand:
# Install maturin
bash
```
cargo install maturin
maturin build
```
# install the created lib
bash
```
pip install target/wheels/poker_eval_rs-0.1.0-cp311-cp311-manylinux_2_34_x86_64.whl 
```
# use python script as a cli
bash
```
python eval.py -h
python eval.py -hilo 2h4d5s6h7d
```
## Usage

To use PokerEval-RS in your Rust project, include it as a dependency in your Cargo.toml file.

Example of using PokerEval-RS to evaluate a poker hand:

## rust
```

```
## Contributing

Contributions to PokerEval-RS are welcome. Please ensure to follow the contribution guidelines outlined in CONTRIBUTING.md.
## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- Original poker_eval library authors
- Rust community for continuous support