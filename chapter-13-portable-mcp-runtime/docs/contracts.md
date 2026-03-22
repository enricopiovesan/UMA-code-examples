# Contracts, Goals, Context, and Events

## Contract model

Each capability is defined through a contract.

A contract must describe:

- capability identity
- version
- intent
- inputs
- outputs
- constraints
- emitted events
- metadata

## Compatibility rule

A capability is compatible when:

- all required inputs are available in the current context
- all declared constraints are satisfied
- the capability contributes to an unmet part of the goal or next required state

## Goal model

A goal defines what the system must produce.

For the MVP, the primary goal is a structured report in French.

## Execution context model

The runtime maintains a shared execution context containing:

- source fragments
- discovered structured facts
- summary
- translated summary
- final report
- output language
- runtime constraints

For the Chapter 13 AI path, two AI-facing capabilities are modeled explicitly:

- `PlannerAI`
- `SummarizerAI`

`PlannerAI` proposes the next capability from the visible contract surface, goal, and current context.
`SummarizerAI` generates a richer report summary when constraints allow it.

`SummarizerAI` is defined as a runtime-hosted AI capability.
That means the contract is stable even if the current implementation path changes underneath it.
In the validated Chapter 13 path today:

- the runtime still resolves the `SummarizerAI` contract
- execution remains visible and inspectable
- if the runtime-hosted AI provider is not bound, the runtime falls back automatically
- the fallback must be reported explicitly in both the machine-readable report and the UI step that executed it

The runtime remains authoritative in both cases:

- planner output is a proposal, not execution authority
- summarization output is still consumed through validated capability execution

## Event model

The event model stays small and explicit.

Recommended event types:

- CapabilityDiscovered
- CapabilityRejected
- CapabilityProposed
- CapabilityValidated
- CapabilityExecuted
- ContextUpdated
- GoalSatisfied

## Runtime rule

The runtime loop must keep these phases explicit:

1. discovery
2. compatibility check
3. proposal
4. validation
5. execution
6. context update
7. event emission

## Important modeling rule

Contracts describe capabilities, not implementations.

A capability may have more than one valid implementation. Runtime resolution chooses a suitable implementation under current conditions.
