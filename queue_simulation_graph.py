#!/usr/bin/env python3
"""Plot simulated M/M/1 E[T] and E[N] against the arrival rate lambda (mu = 1).

Input: queue_simulation.csv with header row "lambda,E[T],E[N]".
Output: queue.png next to the script (override with --out).
"""

import argparse
import csv
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def read_csv(path):
    lam, et, en = [], [], []
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            lam.append(float(row["lambda"]))
            et.append(float(row["E[T]"]))
            en.append(float(row["E[N]"]))
    if not lam:
        raise SystemExit(f"{path}: no data rows")
    return lam, et, en


def main():
    here = pathlib.Path(__file__).resolve().parent
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("csv", nargs="?", default=here / "queue_simulation.csv", type=pathlib.Path)
    ap.add_argument("--out", default=here / "queue.png", type=pathlib.Path)
    args = ap.parse_args()

    lam, et, en = read_csv(args.csv)

    fig, ax = plt.subplots(figsize=(8, 5.5), dpi=150)
    for ys, label, color, offset in [
        (et, "$E[T]$", "#1f77b4", (-4, 8)),
        (en, "$E[N]$", "#2ca02c", (6, -14)),
    ]:
        ax.plot(lam, ys, marker="o", lw=2, color=color, label=label)
        for xi, yi in zip(lam, ys):
            ax.annotate(f"{yi:.2f}", (xi, yi), textcoords="offset points",
                        xytext=offset, ha="right" if offset[0] < 0 else "left",
                        fontsize=8, color="#333333")

    ax.set_xlabel(r"arrival rate $\lambda$")
    ax.set_xticks(lam)
    ax.grid(True, alpha=0.25)
    ax.legend(frameon=False, loc="upper left")
    fig.tight_layout()
    fig.savefig(args.out)
    print(f"wrote {args.out}")


if __name__ == "__main__":
    main()
