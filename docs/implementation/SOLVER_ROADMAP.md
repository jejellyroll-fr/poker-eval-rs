# Solver Roadmap (Rust)

## Product Scope

- Variants (core): Holdem, Holdem8, ShortDeck, Omaha4/5/6, Omaha8/85, Stud7/78/nsq, Razz, Draw5/58/5nsq, Lowball A-5, Lowball 2-7
- Variants (advanced): Double Flop Holdem, Pineapple, Courchevel, Irish, Fusion, Drawmaha, Badugi, Badeucy/Badacey, OFC
- Deck profiles: Standard52, Joker53, Short36, Manila32
- Pro tools: range-vs-range, weighted multiway equity, ICM, advanced range parser, dead-card handling, OFC analyzers
- Solver: CFR+/MCCFR multi-player, multi-street, multi-variant

## Architecture Targets

1. `solver-core` (single source of truth)
- GameTree abstraction, infosets, regrets, policies, exploitability

2. `betting-engine` (public)
- Legal action generation by variant/street
- State transitions, side pots, raise caps, betting structures

3. `variant-adapters`
- holdem, omaha, shortdeck, stud, draw, hybrid families

4. `deck-core`
- Universal deck model + converters (52/53/36/32)

5. `analysis-core`
- Exhaustive/MC/QMC/stratified for equity and distributions

## Decision Tree Optimizations

- Tree profiles: `tight | standard | wide | none`
- Action abstraction by street (`bet-sizes`, `raise-cap`)
- Public-state canonicalization (board/suit isomorphisms)
- Transposition cache
- Lazy expansion
- Reach/regret pruning thresholds
- Depth-limited solving + leaf evaluator (CPU/GPU batch)

## Convergence and Quality Metrics

- NashConv
- Exploitability proxy
- Regret L1/L2 and max regret
- Policy drift (L2/KL between checkpoints)
- EV stability (rolling-window delta)

Solver variants:
- CFR+
- DCFR
- ECFR
- MCCFVFP (flow-weighted updates)

Operational:
- Checkpoint/resume/final snapshot
- Live monitor (hero EV mean/std, progress)
- JSON/CSV reports + plotting scripts

## Performance Program

- SIMD batch paths (SSE2/AVX2)
- Rayon work-stealing parallelism
- Monte Carlo batching
- GPU dispatch-resident evaluation loop
- Release profiles with LTO

## API and Tooling

- CLI: `pokenum`, `multiway_equity`, `solve`, `ofcalc`, `range_tools`
- Solver flags: checkpoint/resume/deals/monitor/tree-profile
- Python bindings: solver + equity APIs
- WASM bindings: equity APIs + wasm-bindgen tests
- Versioned policy/checkpoint formats

## CI and Validation

- Rule invariants by variant/deck
- Canonical regression packs
- Convergence tests per family (holdem/stud/draw/omaha/shortdeck)
- Performance benchmarks separated from PR critical path
- Manual workflow inputs for heavy jobs

## Roadmap Milestones

### M1 - Public Betting Engine
- `GameConfig`, `HandState`, `Action`
- Legal actions per turn + transitions
- Side pot construction
- Multi-player tests and invariants

### M2 - Real Holdem Multi-Street Solver
- Chance nodes + realistic action trees
- CFR+ end-to-end on depth-limited trees
- Baseline exploitability tracking

### M3 - Omaha and ShortDeck
- Full adapters on shared betting/core infra
- Terminal evaluators wired and validated
- Convergence + regression tests

### M4 - Tree Optimization Layer
- Profiles, abstraction, canonicalization, transposition, pruning

### M5 - Solver Variant Suite
- DCFR/ECFR/MCCFVFP + robust checkpointing

### M6 - Advanced Variants
- Courchevel, Irish, Fusion, Drawmaha, OFC ecosystem

### M7 - Regional Deck/Variant Expansion
- Manila deck + regional rules

### M8 - Final Hardening
- Full perf tuning, docs, long-run convergence reporting

