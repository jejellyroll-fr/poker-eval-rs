#!/usr/bin/env python3
import argparse
import csv
import math
import os
from collections import defaultdict


def read_agg(path):
    rows = []
    with open(path, newline="", encoding="utf-8") as f:
        for r in csv.DictReader(f):
            rows.append(
                {
                    "variant": r["variant"],
                    "players": int(r["players"]),
                    "profile": r["profile"],
                    "exp_mean": float(r["exp_mean"]),
                    "time_mean": float(r["time_mean"]),
                }
            )
    return rows


def read_runs(path):
    rows = []
    with open(path, newline="", encoding="utf-8") as f:
        for r in csv.DictReader(f):
            rows.append(
                {
                    "variant": r["variant"],
                    "players": int(r["players"]),
                    "profile": r["profile"],
                    "repeat": int(r["repeat"]),
                    "iteration": int(r["iteration"]),
                    "exploitability": float(r["exploitability"]),
                }
            )
    return rows


def plot_agg(agg_rows, out_png):
    try:
        import matplotlib.pyplot as plt
    except ModuleNotFoundError as exc:
        raise RuntimeError(
            "matplotlib is required. Install with: uv sync --extra plots --no-install-project"
        ) from exc

    groups = defaultdict(list)
    for r in agg_rows:
        key = f"{r['variant']}-{r['players']}p"
        groups[key].append(r)

    labels = []
    vals = []
    for g, rows in sorted(groups.items()):
        rows = sorted(rows, key=lambda x: x["profile"])
        for r in rows:
            if not math.isfinite(r["exp_mean"]):
                continue
            labels.append(f"{g}\n{r['profile']}")
            vals.append(r["exp_mean"])

    plt.figure(figsize=(max(8, len(labels) * 0.5), 5))
    plt.bar(range(len(vals)), vals)
    plt.xticks(range(len(vals)), labels, rotation=45, ha="right")
    plt.ylabel("Mean Final Exploitability")
    plt.title("Solver Batch: Profile Comparison")
    plt.tight_layout()
    plt.savefig(out_png, dpi=160)
    plt.close()


def plot_runs(runs_rows, out_png, variant, players):
    try:
        import matplotlib.pyplot as plt
    except ModuleNotFoundError as exc:
        raise RuntimeError(
            "matplotlib is required. Install with: uv sync --extra plots --no-install-project"
        ) from exc

    sel = [r for r in runs_rows if r["variant"] == variant and r["players"] == players]
    if not sel:
        return False

    grouped = defaultdict(list)
    for r in sel:
        grouped[r["profile"]].append((r["iteration"], r["exploitability"]))

    plt.figure(figsize=(7, 4))
    plotted = False
    for profile, pts in sorted(grouped.items()):
        pts = sorted(pts, key=lambda x: x[0])
        pts = [(x, y) for (x, y) in pts if math.isfinite(y)]
        if not pts:
            continue
        xs = [x for x, _ in pts]
        ys = [y for _, y in pts]
        plt.plot(xs, ys, marker="o", label=profile)
        plotted = True
    plt.xlabel("Iteration")
    plt.ylabel("Exploitability")
    plt.title(f"Convergence Curves: {variant} {players}p")
    if plotted:
        plt.legend()
    plt.tight_layout()
    plt.savefig(out_png, dpi=160)
    plt.close()
    return True


def main():
    p = argparse.ArgumentParser(description="Plot solver batch convergence outputs")
    p.add_argument("--agg", required=True, help="Aggregated CSV path")
    p.add_argument("--runs", required=True, help="Runs CSV path")
    p.add_argument(
        "--out-dir",
        default="docs/reports",
        help="Output directory for generated PNG files",
    )
    p.add_argument(
        "--curve",
        default="holdem:3",
        help="Curve target as variant:players, e.g. holdem:3",
    )
    args = p.parse_args()

    os.makedirs(args.out_dir, exist_ok=True)
    agg_rows = read_agg(args.agg)
    runs_rows = read_runs(args.runs)

    agg_png = os.path.join(args.out_dir, "solver_batch_profiles.png")
    try:
        plot_agg(agg_rows, agg_png)
    except RuntimeError as e:
        print(str(e))
        return

    curve_variant, curve_players = args.curve.split(":")
    curve_png = os.path.join(
        args.out_dir, f"solver_batch_curve_{curve_variant}_{curve_players}p.png"
    )
    ok = plot_runs(runs_rows, curve_png, curve_variant, int(curve_players))

    print(f"Wrote {agg_png}")
    if ok:
        print(f"Wrote {curve_png}")
    else:
        print(f"No run rows matched curve selector {args.curve}")


if __name__ == "__main__":
    main()
