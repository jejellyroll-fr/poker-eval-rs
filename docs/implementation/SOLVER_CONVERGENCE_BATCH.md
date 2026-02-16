# Solver Convergence Batch Pipeline

CLI: `solver_convergence_batch`

## Usage

```bash
cargo run --bin solver_convergence_batch -- \
  --variants holdem,omaha,shortdeck \
  --players 3,4 \
  --profiles tight,standard,wide \
  --repeats 2 \
  --iterations 200 \
  --checkpoint-every 20 \
  --stack 8 \
  --raise-cap 1 \
  --chance-scenarios 2 \
  --chance-stride 7 \
  --cache-opponent-strategies true \
  --runs-csv docs/reports/solver_batch_runs.csv \
  --agg-csv docs/reports/solver_batch_agg.csv \
  --md-out docs/reports/solver_batch_report.md
```

## Outputs

- `runs_csv`: per-run checkpoint rows
- `agg_csv`: aggregated final-point stats per `(variant, players, profile)`
- `md_out`: comparative table with the best profile per `(variant, players)`

## Auto Plots

Use the helper script to produce PNG charts in `docs/reports/`:

```bash
uv run --no-project python scripts/plot_solver_batch.py \
  --agg docs/reports/solver_batch_agg.csv \
  --runs docs/reports/solver_batch_runs.csv \
  --out-dir docs/reports \
  --curve holdem:3
```

Prerequisite:

```bash
uv sync --extra plots --no-install-project
```

Outputs:

- `docs/reports/solver_batch_profiles.png`
- `docs/reports/solver_batch_curve_<variant>_<players>p.png`

## Cost Controls

- `--chance-scenarios`: limits chance branching width at root
- `--chance-stride`: spreads deterministic sampled deals
- `--cache-opponent-strategies`: active le cache de stratégies adverses + cache d'actions de sous-arbres stables
- `--stack`, `--raise-cap`: reduce betting tree size
- build with default `parallel` feature to run jobs concurrently

## Long Campaigns (Frozen Reports)

Use the orchestrator to run chunked campaigns and freeze final artifacts:

```bash
python scripts/run_solver_long_campaigns.py \
  --variants holdem,omaha,shortdeck \
  --players 3,4,5,6 \
  --profiles tight,standard,wide \
  --iterations 30 --repeats 2 \
  --cache-iterations 20 --cache-repeats 2 \
  --report-dir docs/reports
```

Outputs:

- `docs/reports/solver_batch_runs_final.csv`
- `docs/reports/solver_batch_agg_final.csv`
- `docs/reports/solver_traversal_cache_runs_final.csv`
- `docs/reports/solver_traversal_cache_report_final.md`
- `docs/reports/solver_final_report.md`
