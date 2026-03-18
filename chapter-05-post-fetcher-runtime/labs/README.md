# Chapter 5 Labs

These labs accompany the Chapter 5 runtime-layer example.

## Lab 1: Cloud golden path

Command:

```bash
./scripts/run_lab.sh lab1-cloud-golden-path
```

Goal:
- prove the validated cloud host path is deterministic and matches the checked-in golden fixture

What to pay attention to:
- the request URL is the hermetic fixture `uma-fixture://sample-post`, not a localhost server
- the event order should be `start`, `fetch_request`, `fetch_response`, `normalized`, `end`
- the lifecycle binding should show the chosen `network.fetch` implementation

## Lab 2: Header validation fail-fast

Command:

```bash
./scripts/run_lab.sh lab2-header-validation-fail-fast
```

Goal:
- show that runtime validation happens before the adapter call and can stop the request entirely

What to pay attention to:
- there should be an explicit `error` event for the unexpected header
- there should be no `fetch_request` event
- the lifecycle state should be `failed`

## Lab 3: Adapter binding and wrappers

Command:

```bash
./scripts/run_lab.sh lab3-adapter-binding-and-wrappers
```

Goal:
- show that the runtime can change adapter composition through environment-driven binding without changing the service code

What to pay attention to:
- the lifecycle binding should report `cache-retry-host-fetch`
- the normalized output should stay the same even though the runtime path changed

## Lab 4: Rust and TypeScript parity

Command:

```bash
./scripts/run_lab.sh lab4-rust-ts-parity
```

Goal:
- verify that the Rust runtime and the TypeScript reference runtime report the same validated behavior for the main Chapter 5 scenarios

What to pay attention to:
- parity should hold for the cloud golden path, the fail-fast validation path, and the wrapper-binding path
- the TypeScript implementation is a reference model of the runtime behavior, not the primary validated quick-start

## Reflection

You got value from these labs if you can explain:

- why the runtime layer owns validation, adapter selection, and lifecycle recording
- why a failed validation should stop before network fetch, not after it
- how the binding record explains which capability implementation actually ran
