# UMA Examples

This repository is the runnable companion for Universal Microservices Architecture. Each chapter introduces one architectural concern and validates it with code.

## Minimal Capability

[Chapter 4: Feature Flag Evaluator](../../chapter-04-feature-flag-evaluator/README.md) shows the smallest portable service boundary: one contract, deterministic behavior, and a clear test path.

## Runtime Layer

[Chapter 5: Post Fetcher Runtime](../../chapter-05-post-fetcher-runtime/README.md) separates runtime concerns from the portable core.

## Portability Proof

[Chapter 6: Portability Lab](../../chapter-06-portability-lab/README.md) compares native and WebAssembly execution paths.

## Trust And Decisions

[Chapter 9: Trust Boundaries](../../chapter-09-trust-boundaries/README.md) and [Chapter 12: Discoverable Decisions](../../chapter-12-discoverable-decisions/README.md) show how runtime decisions become visible artifacts.

## Full Reference Application

[Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md) combines capability discovery, runtime validation, agent proposals, event-driven execution, and structured output.

## Validation Command

```bash
./scripts/smoke_reader_paths.sh
```

