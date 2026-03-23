# Use Cases

## Primary scenario

Generate a structured report in French from distributed source fragments.

All other use cases should feel like a variation or stress test of that main flow.

## Use Case 1: Basic report generation

### Goal

Generate a structured report.

### Initial context

- local source fragments are available
- translation is not required
- deterministic local execution is preferred

### Expected path

- DataProviderLocal
- InsightEnricher
- SummarizerBasic
- Formatter

### What it proves

- contract-driven composition
- end-to-end flow without a hardwired workflow

## Use Case 2: Same goal, different path through changed constraints

### Goal

Generate a structured report in French.

### Initial context

- distributed source fragments are available
- AI summarization is available
- richer narrative quality is preferred over strict local-only execution

### Expected path

- DataProviderLocal
- InsightEnricher
- SummarizerAI
- TranslatorFr
- Formatter

### What it proves

- the same goal can resolve through a different valid path
- the runtime swaps capabilities based on current constraints instead of application rewrites
- `PlannerAI` and `SummarizerAI` can participate as capabilities without becoming the runtime authority
- if a runtime-hosted AI provider is unavailable, the fallback must remain explicit and visible

## Use Case 3: French output with translation

### Goal

Generate a structured report in French.

### Initial context

- local source fragments are available
- translation capability is available

### Expected path

- DataProviderLocal
- InsightEnricher
- SummarizerBasic or SummarizerAI
- TranslatorFr
- Formatter

### What it proves

- the runtime can insert a new compatible step dynamically
- composition responds to goal requirements instead of static wiring

## Use Case 4: Capability unavailable, runtime adapts

### Goal

Generate a structured report in French.

### Initial context

- translation capability is unavailable
- degraded mode is allowed

### Expected behavior

The runtime either:
- chooses a degraded but valid path, or
- reports why the full French requirement cannot be satisfied

### What it proves

- adaptation under changing capability availability
- explicit degraded execution instead of hidden failure

## Use Case 5: Agent proposes, runtime validates

### Goal

Generate a structured report in French.

### Initial context

- local-only mode enabled
- both summarizers are visible
- translation capability is available

### Agent proposal

- DataProviderLocal
- InsightEnricher
- SummarizerAI
- TranslatorFr
- Formatter

### Expected runtime outcome

- SummarizerAI rejected because it violates local-only constraints
- SummarizerBasic selected instead
- the remaining valid steps continue

### What it proves

- the agent participates in planning
- `PlannerAI` is visible as an explicit planning capability
- the runtime remains contract-authoritative
- rejected candidates are visible and explainable
