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
  README.md
  contracts/
    policies/
    schemas/
  runtime/
    runner.mjs
    trust_lib.mjs
    validate.mjs
    trust.test.mjs
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
    policy_diff.sh
    smoke_trust_labs.sh
```

The runtime prints an allow or deny outcome and an audit log that explains the exact trust rule that fired.

---

## Prerequisites

- Node.js 20 or newer
- a checkout of this repository

No extra packages are required for this Chapter 9 scaffold.
The runtime, validator, and tests use only the built-in Node standard library.

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

Inspect the policy and metadata delta between two labs:

```bash
./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission
```

Run the full Chapter 9 reader path:

```bash
./scripts/smoke_trust_labs.sh
```

---

## Reader path

If you are following the chapter as a fresh reader, use this order:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_trust.sh lab1-trusted-service`
3. `./scripts/run_trust_demo.sh lab1-trusted-service`
4. `./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission`
5. `./scripts/run_trust_demo.sh lab2-undeclared-permission`
6. `./scripts/policy_diff.sh lab2-undeclared-permission lab3-untrusted-dependency`
7. `./scripts/run_trust_demo.sh lab3-untrusted-dependency`
8. `./scripts/policy_diff.sh lab3-untrusted-dependency lab4-forbidden-communication`
9. `./scripts/run_trust_demo.sh lab4-forbidden-communication`
10. `./scripts/policy_diff.sh lab4-forbidden-communication lab5-restored-compliance`
11. `./scripts/run_trust_demo.sh lab5-restored-compliance`

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

### "How do I know if the lab gave me value?"

You got value from the Chapter 9 lab if you can explain all three of these points after running it:

- a service can be syntactically valid and still be blocked by runtime trust policy
- dependency provenance is part of trust, not a separate afterthought
- inter-service communication is allowed only when both compatibility and trust policy agree

If those points are not obvious from the output, compare:

- `./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission`
- `./scripts/policy_diff.sh lab3-untrusted-dependency lab4-forbidden-communication`
- `./scripts/run_trust_demo.sh lab5-restored-compliance`

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
./scripts/policy_diff.sh lab1-trusted-service lab2-undeclared-permission
./scripts/run_trust_demo.sh lab2-undeclared-permission
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
./scripts/policy_diff.sh lab2-undeclared-permission lab3-untrusted-dependency
./scripts/run_trust_demo.sh lab3-untrusted-dependency
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
./scripts/policy_diff.sh lab3-untrusted-dependency lab4-forbidden-communication
./scripts/run_trust_demo.sh lab4-forbidden-communication
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
./scripts/policy_diff.sh lab4-forbidden-communication lab5-restored-compliance
./scripts/run_trust_demo.sh lab5-restored-compliance
```

Expected outcome:

```text
Outcome: allow
- [allow] communication upload-bridge->internal-audit-sink: communication.trusted
```

Architectural point:
Restoring compliant metadata restores trusted behavior without changing the runtime itself.
