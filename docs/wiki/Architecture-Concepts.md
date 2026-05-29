# Architecture Concepts

UMA uses a small set of recurring architecture concepts. The code examples are organized to introduce these concepts gradually.

## Portable Capability

A portable capability owns deterministic business behavior behind an explicit contract. It avoids host-specific concerns in the core logic.

## Runtime Layer

The runtime hosts capabilities, validates inputs, applies trust and policy rules, approves or rejects execution, and records evidence.

## Capability Registry

A capability registry describes what a capability can do, what it accepts, and how it can be composed into workflows.

## Trust Boundary

A trust boundary keeps provenance, policy, and execution authority visible. UMA does not assume that portable code is automatically safe.

## Discoverable Decision

A discoverable decision is an approval, rejection, or routing choice represented as data that can be inspected after execution.

## Workflow

A workflow composes capabilities through runtime-approved steps rather than hidden glue code.

## Code References

- [Chapter 7: Metadata Orchestration](../../chapter-07-metadata-orchestration/README.md)
- [Chapter 9: Trust Boundaries](../../chapter-09-trust-boundaries/README.md)
- [Chapter 12: Discoverable Decisions](../../chapter-12-discoverable-decisions/README.md)
- [Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md)

