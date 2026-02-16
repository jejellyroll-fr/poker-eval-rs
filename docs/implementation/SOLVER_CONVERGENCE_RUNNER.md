# Solver Convergence Runner

CLI: `solver_convergence`

## Usage

```bash
cargo run --bin solver_convergence -- \
  --variants holdem,omaha,shortdeck \
  --players 3,4 \
  --iterations 600 \
  --checkpoint-every 50 \
  --tree-profile tight \
  --cache-opponent-strategies true \
  --csv-out docs/reports/solver_convergence.csv \
  --md-out docs/reports/solver_convergence.md
```

Mode rapide (multiway):

```bash
cargo run --bin solver_convergence -- \
  --variants holdem,omaha,shortdeck \
  --players 3,4 \
  --fast \
  --csv-out docs/reports/solver_convergence_fast.csv \
  --md-out docs/reports/solver_convergence_fast.md
```

## Outputs

- CSV: per-checkpoint points with columns:
  - `variant,players,iteration,exploitability`
- Markdown:
  - one-line run config summary
  - per `(variant,players)` table with start/final exploitability and delta

## Notes

- Exploitability metric is the n-player proxy from `solver-core`:
  - `sum_i(BR_i - U_i)` using the current average policy profile.
- Tree profile (`tight|standard|wide`) is forwarded to the betting abstraction.
- `--cache-opponent-strategies` active le cache d'infosets non-updater et le cache d'actions de sous-arbres.
- `--fast` applies a lighter preset:
  - profile `tight`
  - reduced stack / raise cap
  - reduced chance scenarios
  - bounded iterations/checkpoint frequency
