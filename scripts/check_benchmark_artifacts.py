#!/usr/bin/env python3
"""Validate that published benchmark proof artifacts still exist and keep the expected shape."""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
JSON_PATH = ROOT / "benchmarks" / "benchmark-proof.json"
MD_PATH = ROOT / "benchmarks" / "benchmark-proof.md"
PAGE_PATH = ROOT / "book-site" / "benchmark-and-footprint" / "index.html"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def main() -> None:
    require(JSON_PATH.exists(), "Missing benchmarks/benchmark-proof.json")
    require(MD_PATH.exists(), "Missing benchmarks/benchmark-proof.md")
    require(PAGE_PATH.exists(), "Missing book-site/benchmark-and-footprint/index.html")

    data = json.loads(JSON_PATH.read_text())
    require("generated_at" in data, "benchmark-proof.json missing generated_at")
    require("environment" in data, "benchmark-proof.json missing environment")
    require("benchmarks" in data, "benchmark-proof.json missing benchmarks")

    benchmarks = data["benchmarks"]
    for key in ("chapter4", "chapter6", "chapter13"):
        require(key in benchmarks, f"benchmark-proof.json missing {key}")
        require("artifacts" in benchmarks[key], f"{key} missing artifacts")
        require("timings" in benchmarks[key], f"{key} missing timings")

    markdown = MD_PATH.read_text()
    for heading in (
        "## Chapter 4: Feature Flag Evaluator",
        "## Chapter 6: Portability Lab",
        "## Chapter 13: Reference Runtime CLI",
        "## Interpretation",
    ):
        require(heading in markdown, f"benchmark-proof.md missing section: {heading}")

    page = PAGE_PATH.read_text()
    for needle in (
        "Benchmark report",
        "Benchmark script",
        "Rust WASI via Wasmtime",
        "WASI runner via Wasmtime",
        "reference runtime CLI",
    ):
        require(needle in page, f"benchmark page missing content: {needle}")

    print("Benchmark proof artifact checks passed.")


if __name__ == "__main__":
    main()
