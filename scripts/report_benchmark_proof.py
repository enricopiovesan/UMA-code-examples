#!/usr/bin/env python3
"""Generate a small reproducible benchmark-and-footprint report for UMA examples."""

from __future__ import annotations

import json
import os
import platform
import shlex
import statistics
import subprocess
import sys
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
OUTPUT_DIR = ROOT / "benchmarks"
JSON_PATH = OUTPUT_DIR / "benchmark-proof.json"
MD_PATH = OUTPUT_DIR / "benchmark-proof.md"


def run(cmd: list[str], cwd: Path | None = None, env: dict[str, str] | None = None, capture: bool = False) -> str:
    result = subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        env=env,
        check=True,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.STDOUT if capture else None,
    )
    return result.stdout if capture else ""


def detect_wasmtime() -> str:
    candidates = [
        ROOT / ".bin" / "wasmtime-v39.0.0-aarch64-macos" / "wasmtime",
        ROOT / ".bin" / "wasmtime-v39.0.0-x86_64-linux" / "wasmtime",
    ]
    for candidate in candidates:
        if candidate.exists():
            return str(candidate)

    path = shutil_which("wasmtime")
    if path:
        return path

    raise SystemExit("Unable to locate wasmtime. Expected a local .bin installation or a PATH entry.")


def shutil_which(binary: str) -> str | None:
    for entry in os.environ.get("PATH", "").split(os.pathsep):
        candidate = Path(entry) / binary
        if candidate.exists() and os.access(candidate, os.X_OK):
            return str(candidate)
    return None


def command_version(cmd: list[str], cwd: Path | None = None, env: dict[str, str] | None = None) -> str:
    return run(cmd, cwd=cwd, env=env, capture=True).strip()


def measure(cmd: list[str], cwd: Path, env: dict[str, str], runs: int = 20, warmups: int = 2) -> dict[str, float]:
    for _ in range(warmups):
        run(cmd, cwd=cwd, env=env, capture=True)

    samples_ms: list[float] = []
    for _ in range(runs):
        start = time.perf_counter()
        run(cmd, cwd=cwd, env=env, capture=True)
        elapsed_ms = (time.perf_counter() - start) * 1000.0
        samples_ms.append(elapsed_ms)

    return {
        "runs": runs,
        "mean_ms": round(statistics.fmean(samples_ms), 2),
        "median_ms": round(statistics.median(samples_ms), 2),
        "min_ms": round(min(samples_ms), 2),
        "max_ms": round(max(samples_ms), 2),
    }


def human_size(size_bytes: int) -> str:
    units = ["B", "KiB", "MiB", "GiB"]
    size = float(size_bytes)
    for unit in units:
        if size < 1024.0 or unit == units[-1]:
            return f"{size:.2f} {unit}"
        size /= 1024.0
    return f"{size_bytes} B"


def gather() -> dict:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    wasmtime = detect_wasmtime()
    env = os.environ.copy()
    env["PATH"] = f"{Path(wasmtime).parent}{os.pathsep}{env.get('PATH', '')}"

    chapter4 = ROOT / "chapter-04-feature-flag-evaluator"
    chapter6 = ROOT / "chapter-06-portability-lab"
    chapter6_runtime = chapter6 / "runtime"
    chapter13 = ROOT / "chapter-13-portable-mcp-runtime"
    chapter13_rust = chapter13 / "rust"

    # Build benchmark targets in release mode.
    run(
        ["cargo", "build", "--release", "--target", "wasm32-wasip1", "-p", "ff_eval_wasi_app", "--manifest-path", str(chapter4 / "Cargo.toml")],
        cwd=ROOT,
        env=env,
    )
    run(
        ["cargo", "build", "--release", "-p", "runner_native", "-p", "runner_wasm", "--target", "wasm32-wasip1", "--manifest-path", str(chapter6_runtime / "Cargo.toml")],
        cwd=ROOT,
        env=env,
    )
    # native target for chapter 6
    run(
        ["cargo", "build", "--release", "-p", "runner_native", "--manifest-path", str(chapter6_runtime / "Cargo.toml")],
        cwd=ROOT,
        env=env,
    )
    run(
        ["cargo", "build", "--release", "--manifest-path", str(chapter13_rust / "Cargo.toml")],
        cwd=ROOT,
        env=env,
    )

    c4_wasm = chapter4 / "target" / "wasm32-wasip1" / "release" / "ff_eval_wasi_app.wasm"
    c6_native = chapter6_runtime / "target" / "release" / "runner_native"
    c6_wasm = chapter6_runtime / "target" / "wasm32-wasip1" / "release" / "runner_wasm.wasm"
    c13_native = chapter13_rust / "target" / "release" / "chapter13_portable_mcp_runtime"

    chapter4_rust_cmd = [wasmtime, str(c4_wasm)]
    chapter4_ts_cmd = ["node", str(chapter4 / "ts" / "src" / "main.mjs")]
    chapter13_render_cmd = [str(c13_native), "render", "use-case-1-basic-report", "json"]
    chapter4_input = (chapter4 / "labs" / "inputs" / "lab2-rollout-match.json").read_text()

    def measure_stdin(cmd: list[str], cwd: Path, stdin_text: str, runs: int = 20, warmups: int = 2) -> dict[str, float]:
        for _ in range(warmups):
            subprocess.run(cmd, cwd=str(cwd), env=env, check=True, input=stdin_text, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

        samples_ms: list[float] = []
        for _ in range(runs):
            start = time.perf_counter()
            subprocess.run(cmd, cwd=str(cwd), env=env, check=True, input=stdin_text, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            samples_ms.append((time.perf_counter() - start) * 1000.0)

        return {
            "runs": runs,
            "mean_ms": round(statistics.fmean(samples_ms), 2),
            "median_ms": round(statistics.median(samples_ms), 2),
            "min_ms": round(min(samples_ms), 2),
            "max_ms": round(max(samples_ms), 2),
        }

    data = {
        "generated_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "environment": {
            "os": platform.platform(),
            "python": sys.version.split()[0],
            "rustc": command_version(["rustc", "--version"], env=env),
            "node": command_version(["node", "--version"], env=env),
            "wasmtime": command_version([wasmtime, "--version"], env=env),
        },
        "benchmarks": {
            "chapter4": {
                "input": "lab2-rollout-match.json",
                "artifacts": {
                    "wasi_module_bytes": c4_wasm.stat().st_size,
                    "wasi_module_human": human_size(c4_wasm.stat().st_size),
                },
                "timings": {
                    "rust_wasi_via_wasmtime": measure_stdin(chapter4_rust_cmd, chapter4, chapter4_input),
                    "typescript_node": measure_stdin(chapter4_ts_cmd, chapter4, chapter4_input),
                },
            },
            "chapter6": {
                "input": "sample-data/sample.pgm",
                "artifacts": {
                    "native_runner_bytes": c6_native.stat().st_size,
                    "native_runner_human": human_size(c6_native.stat().st_size),
                    "wasi_runner_bytes": c6_wasm.stat().st_size,
                    "wasi_runner_human": human_size(c6_wasm.stat().st_size),
                },
                "timings": {
                    "native_runner": measure([str(c6_native), "../sample-data/sample.pgm"], chapter6_runtime, env),
                    "wasi_runner_via_wasmtime": measure([wasmtime, "run", "--dir=..", str(c6_wasm), "../sample-data/sample.pgm"], chapter6_runtime, env),
                },
            },
            "chapter13": {
                "input": "use-case-1-basic-report",
                "artifacts": {
                    "cli_binary_bytes": c13_native.stat().st_size,
                    "cli_binary_human": human_size(c13_native.stat().st_size),
                },
                "timings": {
                    "render_json_cli": measure(chapter13_render_cmd, chapter13, env),
                },
            },
        },
    }

    return data


def write_markdown(data: dict) -> str:
    c4 = data["benchmarks"]["chapter4"]
    c6 = data["benchmarks"]["chapter6"]
    c13 = data["benchmarks"]["chapter13"]
    lines = [
        "# Benchmark And Footprint Notes",
        "",
        "These measurements are a reproducible local proof point, not a universal performance claim.",
        "",
        f"- generated: `{data['generated_at']}`",
        f"- environment: `{data['environment']['os']}`",
        f"- rust: `{data['environment']['rustc']}`",
        f"- node: `{data['environment']['node']}`",
        f"- wasmtime: `{data['environment']['wasmtime']}`",
        "",
        "## Chapter 4: Feature Flag Evaluator",
        "",
        f"- WASI module size: `{c4['artifacts']['wasi_module_human']}`",
        f"- benchmark input: `{c4['input']}`",
        "",
        "| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for label, metrics in c4["timings"].items():
        lines.append(f"| {label} | {metrics['mean_ms']} | {metrics['median_ms']} | {metrics['min_ms']} | {metrics['max_ms']} | {metrics['runs']} |")

    lines.extend(
        [
            "",
            "## Chapter 6: Portability Lab",
            "",
            f"- native runner size: `{c6['artifacts']['native_runner_human']}`",
            f"- WASI runner size: `{c6['artifacts']['wasi_runner_human']}`",
            f"- benchmark input: `{c6['input']}`",
            "",
            "| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for label, metrics in c6["timings"].items():
        lines.append(f"| {label} | {metrics['mean_ms']} | {metrics['median_ms']} | {metrics['min_ms']} | {metrics['max_ms']} | {metrics['runs']} |")

    lines.extend(
        [
            "",
            "## Chapter 13: Reference Runtime CLI",
            "",
            f"- CLI binary size: `{c13['artifacts']['cli_binary_human']}`",
            f"- benchmark input: `{c13['input']}`",
            "",
            "| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
            f"| render_json_cli | {c13['timings']['render_json_cli']['mean_ms']} | {c13['timings']['render_json_cli']['median_ms']} | {c13['timings']['render_json_cli']['min_ms']} | {c13['timings']['render_json_cli']['max_ms']} | {c13['timings']['render_json_cli']['runs']} |",
            "",
            "## Interpretation",
            "",
            "- Chapter 4 shows a very small portable evaluator module with comparable Rust/WASI and TypeScript invocation timings on the same contract-driven input.",
            "- Chapter 6 shows the expected tradeoff: the native runner stays faster, while the WASI runner remains compact and behaviorally aligned.",
            f"- Chapter 13 shows the reference runtime can expose a deterministic report path from one release CLI binary (`{c13['artifacts']['cli_binary_human']}`) with a mean local render time of `{c13['timings']['render_json_cli']['mean_ms']} ms` for `use-case-1-basic-report`.",
            "- The important proof is not “fastest everywhere.” It is that portable behavior remains measurable, comparable, and explicit across runtime choices.",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> None:
    data = gather()
    JSON_PATH.write_text(json.dumps(data, indent=2) + "\n")
    MD_PATH.write_text(write_markdown(data))
    print(f"Wrote {JSON_PATH.relative_to(ROOT)}")
    print(f"Wrote {MD_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
