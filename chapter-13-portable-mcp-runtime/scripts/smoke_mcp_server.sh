#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVER_CMD=(cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- mcp-serve)

python3 - "$ROOT_DIR" <<'PY'
import json
import subprocess
import sys

root = sys.argv[1]
proc = subprocess.Popen(
    ["cargo", "run", "--locked", "--quiet", "--manifest-path", f"{root}/rust/Cargo.toml", "--", "mcp-serve"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
)

def send(message):
    body = json.dumps(message).encode("utf-8")
    payload = f"Content-Length: {len(body)}\r\n\r\n".encode("utf-8") + body
    proc.stdin.write(payload)
    proc.stdin.flush()

def recv():
    headers = {}
    while True:
        line = proc.stdout.readline()
        if not line:
            raise RuntimeError("server closed stdout")
        if line in (b"\r\n", b"\n"):
            break
        key, value = line.decode("utf-8").split(":", 1)
        headers[key.strip().lower()] = value.strip()
    length = int(headers["content-length"])
    body = proc.stdout.read(length)
    return json.loads(body.decode("utf-8"))

try:
    send({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}})
    init = recv()
    assert init["result"]["serverInfo"]["name"] == "chapter13-portable-mcp-runtime"

    send({"jsonrpc": "2.0", "method": "notifications/initialized", "params": {}})

    send({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}})
    tools = recv()
    tool_names = {tool["name"] for tool in tools["result"]["tools"]}
    assert "run_scenario" in tool_names
    assert "list_capabilities" in tool_names

    send({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "run_scenario",
            "arguments": {
                "scenario": "use-case-1-basic-report"
            }
        }
    })
    report = recv()
    structured = report["result"]["structuredContent"]
    assert structured["scenario"] == "use-case-1-basic-report"
    assert structured["final_language"] == "en"
finally:
    if proc.stdin:
        proc.stdin.close()
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=5)
PY

echo "Chapter 13 MCP server passed."
