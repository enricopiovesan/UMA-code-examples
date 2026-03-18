# Chapter 9: Trust Boundaries and Runtime Enforcement

This example turns Chapter 9 of the UMA book into a reader-facing hands-on sequence.
The focus is not generic application security. The focus is how UMA makes trust explicit through metadata, runtime policy, dependency verification, and communication rules.

Chapter 9 explains that portable systems cannot inherit trust from infrastructure alone.
Instead, a runtime must evaluate:

- who published a service
- which trust tier it belongs to
- which permissions it declares
- which dependencies it carries
- whether communication across services is allowed

The labs below let a reader watch those trust decisions become visible through deterministic allow and deny outcomes.

---

## Learning goals

By the end of this hands-on section, the reader should be able to:

- run a trusted service whose metadata satisfies runtime policy
- see the runtime deny execution when a service asks for undeclared permissions
- see supply-chain trust fail when dependency provenance is not acceptable
- understand that valid message shape alone is not enough for trusted communication
- restore compliance and explain why the runtime allows the system again

---

## Folder layout

```text
chapter-9-trust-boundaries/
  Cargo.toml
  README.md
  contracts/
    policies/
    schemas/
  src/
    lib.rs
    main.rs
  ts/
    package.json
    src/
  scenarios/
    lab1-trusted-service/
    lab2-undeclared-permission/
    lab3-untrusted-dependency/
    lab4-forbidden-communication/
    lab5-restored-compliance/
  scripts/
    list_labs.sh
    run_trust_demo.sh
    validate_trust.sh
    trust_diff.sh
    policy_diff.sh
    compare_impls.sh
    run_trust_demo_ts.sh
    smoke_trust_labs.sh
```

The validated reader path is the Rust CLI in `src/`.
The secondary implementation is the TypeScript parity path under `ts/`.

---

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- a checkout of this repository

The Rust CLI is the validated default path.
Node.js is still required for the full smoke run and the TypeScript parity checks.
The TypeScript implementation is retained as a secondary implementation for comparison.

## Validation status

- Validated path: `./scripts/smoke_trust_labs.sh`
- Main implementation: Rust CLI in `src/`
- Secondary implementation: TypeScript implementation in `ts/`
- Implementation parity check: `./scripts/compare_impls.sh <lab-name>`

---

## Quick start

Start by listing the labs:

```bash
cd chapter-9-trust-boundaries
./scripts/list_labs.sh
```

Run a single trust scenario:

```bash
./scripts/run_trust_demo.sh lab1-trusted-service
```

Validate all Chapter 9 contracts and runtime plans:

```bash
./scripts/validate_trust.sh
```

Inspect the trust-decision delta between two labs:

```bash
./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission
```

Verify Rust and TypeScript stay aligned for one lab:

```bash
./scripts/compare_impls.sh lab1-trusted-service
```

Inspect the raw scenario metadata and policy delta when you want the exact file-level change:

```bash
./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission
```

Run the full Chapter 9 reader path:

```bash
./scripts/smoke_trust_labs.sh
```

If you want to inspect the TypeScript implementation directly:

```bash
./scripts/run_trust_demo_ts.sh lab1-trusted-service
```

## Troubleshooting

- If `cargo` reports dependency resolution failures, run the commands with network access at least once so Cargo can fetch the locked dependencies.
- If `npm test` fails in `ts/`, ensure you are using Node.js 20 or newer.
- If you are unsure which scenarios exist, run `./scripts/list_labs.sh`.
- Start with `./scripts/trust_diff.sh` for behavioral changes and only use `./scripts/policy_diff.sh` when you want the raw file diff.
- If you want to confirm Rust and TypeScript still agree for a scenario, use `./scripts/compare_impls.sh <lab>`.

---

## Reader path

If you are following the chapter as a fresh reader, use this order:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_trust.sh lab1-trusted-service`
3. `./scripts/run_trust_demo.sh lab1-trusted-service`
4. `./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission`
5. `./scripts/compare_impls.sh lab2-undeclared-permission`
6. `./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission`
7. `./scripts/run_trust_demo.sh lab2-undeclared-permission`
8. `./scripts/trust_diff.sh lab2-undeclared-permission lab3-untrusted-dependency`
9. `./scripts/compare_impls.sh lab3-untrusted-dependency`
10. `./scripts/policy_diff.sh lab2-undeclared-permission lab3-untrusted-dependency`
11. `./scripts/run_trust_demo.sh lab3-untrusted-dependency`
12. `./scripts/trust_diff.sh lab3-untrusted-dependency lab4-forbidden-communication`
13. `./scripts/compare_impls.sh lab4-forbidden-communication`
14. `./scripts/policy_diff.sh lab3-untrusted-dependency lab4-forbidden-communication`
15. `./scripts/run_trust_demo.sh lab4-forbidden-communication`
16. `./scripts/trust_diff.sh lab4-forbidden-communication lab5-restored-compliance`
17. `./scripts/compare_impls.sh lab5-restored-compliance`
18. `./scripts/policy_diff.sh lab4-forbidden-communication lab5-restored-compliance`
19. `./scripts/run_trust_demo.sh lab5-restored-compliance`

That flow mirrors the chapter idea:

- trusted execution must be earned through metadata
- portability does not grant undeclared permissions
- dependency trust is part of runtime trust
- communication is governed by trust policy, not shape alone
- restoring compliant metadata restores the system

---

## Questions A Reader Might Ask

### "What exactly is the runtime checking?"

This Chapter 9 runtime evaluates:

- publisher trust
- service trust tier
- placement compatibility
- requested permissions versus declared permissions
- dependency provenance and checksum presence
- event communication rules across trust tiers

### "What should I pay attention to in the output?"

The most important lines are:

- `Outcome: allow|deny`
- `publisher: ...`
- `trust tier: ...`
- `permissions: ...`
- `[deny] ...: permission.undeclared`
- `[deny] ...: dependency.provenance.untrusted`
- `[deny] ...: communication.forbidden`
- `Outcome: deny -> allow` in `./scripts/trust_diff.sh`

### "When should I use `policy_diff.sh` versus `trust_diff.sh`?"

Use:

- `./scripts/trust_diff.sh` when you want a short explanation of what changed in trust terms
- `./scripts/policy_diff.sh` when you want the raw Git diff of the scenario files and metadata

The first answers "what changed in runtime behavior?"
The second answers "which exact metadata fields changed?"

Most readers should start with `trust_diff.sh` and only use `policy_diff.sh` when they want the underlying file diff.

### "Which implementation should I treat as the main one?"

Use the Rust CLI through the `scripts/` entry points.
Those scripts call `cargo run --locked` and `cargo test --locked`, so the Chapter 9 quick-start path is Rust-first and reproducible on a clean machine.

TypeScript lives under `ts/` and is kept in parity through `./scripts/compare_impls.sh`.

### "How do I know if the lab gave me value?"

You got value from the Chapter 9 lab if you can explain all three of these points after running it:

- a service can be syntactically valid and still be blocked by runtime trust policy
- dependency provenance is part of trust, not a separate afterthought
- inter-service communication is allowed only when both compatibility and trust policy agree

If those points are not obvious from the output, compare:

- `./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission`
- `./scripts/trust_diff.sh lab3-untrusted-dependency lab4-forbidden-communication`
- `./scripts/run_trust_demo.sh lab5-restored-compliance`
- `./scripts/compare_impls.sh lab5-restored-compliance`

---

## Hands-on flow

### Lab 9.1: Run a Trusted Service

Start with a service whose metadata, permissions, publisher, and dependencies all satisfy policy.

Suggested commands:

```bash
./scripts/validate_trust.sh lab1-trusted-service
./scripts/run_trust_demo.sh lab1-trusted-service
```

Expected outcome:

```text
Outcome: allow
- [allow] execution case-redactor: execution.trusted
```

Architectural point:
Trust is attached to the service metadata and validated at runtime.

### Lab 9.2: Block an Undeclared Permission

The service now requests a permission that was never declared in its contract.

Suggested commands:

```bash
./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission
./scripts/run_trust_demo.sh lab2-undeclared-permission
```

Optional deeper inspection:

```bash
./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission
```

Expected outcome:

```text
Outcome: deny
- [deny] execution case-redactor: permission.undeclared
```

Architectural point:
Portable code is not trusted to access capabilities it did not declare.

### Lab 9.3: Reject an Untrusted Dependency

The service now carries a dependency whose provenance is not verified.

Suggested commands:

```bash
./scripts/trust_diff.sh lab2-undeclared-permission lab3-untrusted-dependency
./scripts/run_trust_demo.sh lab3-untrusted-dependency
```

Optional deeper inspection:

```bash
./scripts/policy_diff.sh lab2-undeclared-permission lab3-untrusted-dependency
```

Expected outcome:

```text
Outcome: deny
- [deny] execution case-redactor: dependency.provenance.untrusted
```

Architectural point:
Supply-chain trust is part of the architectural trust boundary.

### Lab 9.4: Forbid Cross-Boundary Communication

Two services are individually trusted enough to execute, but the consumer trust tier is not permitted to receive the event.

Suggested commands:

```bash
./scripts/trust_diff.sh lab3-untrusted-dependency lab4-forbidden-communication
./scripts/run_trust_demo.sh lab4-forbidden-communication
```

Optional deeper inspection:

```bash
./scripts/policy_diff.sh lab3-untrusted-dependency lab4-forbidden-communication
```

Expected outcome:

```text
Outcome: deny
- [deny] communication upload-bridge->partner-audit-sink: communication.forbidden
```

Architectural point:
Valid contracts alone do not authorize communication across trust boundaries.

### Lab 9.5: Restore Compliance

Fix the consumer identity so the communication rule is satisfied again.

Suggested commands:

```bash
./scripts/trust_diff.sh lab4-forbidden-communication lab5-restored-compliance
./scripts/run_trust_demo.sh lab5-restored-compliance
```

Optional deeper inspection:

```bash
./scripts/policy_diff.sh lab4-forbidden-communication lab5-restored-compliance
```

Expected outcome:

```text
Outcome: allow
- [allow] communication upload-bridge->internal-audit-sink: communication.trusted
```

Architectural point:
Restoring compliant metadata restores trusted behavior without changing the runtime itself.
