---
name: sdk-ci-runner
description: Run the Claude Agent SDK against a chapter or the full reader path to produce a structured diagnosis report. Use when asked for machine-readable validation or AI-assisted CI triage.
---

## What this skill does

Invokes the Claude Agent SDK CLI with structured JSON output so failures can be summarized programmatically.

## Requirements

- `@anthropic-ai/claude-agent-sdk` installed
- `ANTHROPIC_API_KEY` available in the environment

## Steps

1. Identify the target scope:
   - one chapter for focused diagnosis
   - repo root for full reader-path diagnosis
2. Run a structured SDK prompt similar to:
```bash
claude -p "Run the relevant smoke path for <target>. Use the repo scripts and return JSON with fields: target, passed_steps[], failed_steps[], errors{}, suggested_fixes{}." \
  --allowedTools "Bash,Read,Glob" \
  --output-format json \
  --bare
```
3. Parse the structured output and extract:
   - target name
   - pass/fail summary
   - exact failure explanations
   - suggested fixes
4. Present the result as a diagnosis artifact, not a replacement for local validation.

## When to use this vs validate-chapter

- Use `validate-chapter` for fast local debugging.
- Use `sdk-ci-runner` when you need machine-readable output, CI artifacts, or a second-pass diagnosis for a confusing failure.

## Never do

- Do not treat SDK output as authoritative if local smoke still fails.
- Do not run it without a clear target or acceptance script.
