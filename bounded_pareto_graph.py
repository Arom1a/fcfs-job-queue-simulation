#!/usr/bin/env python3
"""Plot sample averages of BoundedPareto(332.067, 1e10, 1.1) against sample size.

Input: graph.csv with header row -- columns "instances" and "sample average".
Output: graph.png next to the script (override with --out).
"""

import argparse
import csv
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker

LO, HI, ALPHA = 332.067, 1e10, 1.1


def read_csv(path):
    n, avg = [], []
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            n.append(int(row["instances"]))
            avg.append(float(row["sample average"]))
    if not n:
        raise SystemExit(f"{path}: no data rows")
    return n, avg


def main():
    here = pathlib.Path(__file__).resolve().parent
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("csv", nargs="?", default=here / "graph.csv", type=pathlib.Path)
    ap.add_argument("--out", default=here / "graph.png", type=pathlib.Path)
    args = ap.parse_args()

    n, avg = read_csv(args.csv)

    # Analytic mean of the bounded Pareto on [LO, HI].
    theo = (ALPHA / (ALPHA - 1.0)) * (
        LO ** (1.0 - ALPHA) - HI ** (1.0 - ALPHA)
    ) / (LO**-ALPHA - HI**-ALPHA)

    fig, ax = plt.subplots(figsize=(9, 5.5), dpi=150)
    ax.plot(n, avg, marker="o", lw=2, color="#1f77b4", label="sample average")
    ax.axhline(theo, ls="--", lw=1.2, color="#d62728",
               label=f"theoretical mean $\\approx$ {theo:,.0f}")

    for xi, yi in zip(n, avg):
        ax.annotate(f"{yi:,.1f}", (xi, yi), textcoords="offset points",
                    xytext=(0, 10), ha="center", fontsize=7.5, color="#333333")

    ax.set_xscale("log")
    ax.set_xlabel("number of instances $N$ (log scale)")
    ax.set_ylabel("sample average")
    ax.set_title("BoundedPareto(332.067, $10^{10}$, 1.1): sample average vs. sample size")
    ax.xaxis.set_major_formatter(mticker.FuncFormatter(
        lambda v, _: f"{v:.0e}" if v >= 1e4 else f"{int(v):,}"))
    ax.grid(True, which="both", alpha=0.25)
    ax.set_ylim(0, 4200)
    ax.legend(frameon=False)
    fig.tight_layout()
    fig.savefig(args.out)
    print(f"wrote {args.out}")


if __name__ == "__main__":
    main()
