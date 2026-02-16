# Solver Traversal Cache Bench

Benchmark Criterion dédié pour mesurer l'impact du cache de traversal CFR+.

## Bench

- Nom: `solver_core_traversal_cache_bench`
- Matrice:
  - Variantes: `holdem`, `omaha`, `shortdeck`
  - Joueurs: `3..6`
  - Modes: `cache_on` vs `cache_off`

## Lancer

```bash
cargo bench --bench solver_core_traversal_cache_bench
```

Filtrer un cas:

```bash
cargo bench --bench solver_core_traversal_cache_bench -- "holdem_3p/cache_on"
```

## Résultats

- Snapshot CSV auto-écrit:
  - `benches/results/solver_core_traversal_cache_snapshot.csv`
  - Colonnes: `variant,players,iters,cache_on,time_sec`

## Runner Report (recommande)

Pour un run batch complet plus robuste (CSV + Markdown auto):

```bash
cargo run --bin solver_traversal_cache_report -- \
  --variants holdem,omaha,shortdeck \
  --players 3,4,5,6 \
  --repeats 2 \
  --iterations 20 \
  --runs-csv docs/reports/solver_traversal_cache_runs.csv \
  --md-out docs/reports/solver_traversal_cache_report.md
```

## Note Windows (linker lock)

Si `LNK1104` bloque `target/release/...exe`, relancer avec un target dir isolé:

```bash
set CARGO_TARGET_DIR=target_bench_iso
cargo bench --bench solver_core_traversal_cache_bench
```
