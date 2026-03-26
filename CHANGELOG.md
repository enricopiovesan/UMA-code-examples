# Changelog

This changelog tracks meaningful repo-level updates to the UMA examples, website, and reference application.

The project is still evolving quickly, so early entries summarize the architectural and reader-facing milestones instead of every low-level code change.

## Unreleased

- added reproducible benchmark-and-footprint reporting for selected UMA examples
- published a benchmark-and-footprint page on the website and linked it from the repo and site navigation

## 2026-03-25

### Repo and reader experience

- rewrote the root README to frame UMA as an execution model for distributed systems
- added a clearer learning path, stronger proof language, and links to the live reference app
- added a business-logic coverage gate in CI and documented the `100%` target for deterministic domain logic
- added root dual-license files (`MIT` or `Apache-2.0`)

### Website and book site

- expanded the public site with a full concept cluster:
  - what problem UMA solves
  - capability
  - workflow
  - UMA runtime
  - WASM MCP
  - agent vs runtime
  - portability, trust, orchestration, coherence, and system evolution
- reworked homepage and subpage positioning to emphasize:
  - UMA as an execution model
  - `Write once, run where it makes sense.`
- added stronger footer navigation, book CTA, FAQ growth, diagrams, sitemap, robots, and canonical improvements

### Chapter 13 reference application

- stabilized the portable MCP runtime reference app and aligned the graph, narrative, and workflow model
- published the reference app under:
  - `https://www.universalmicroservices.com/reference-application/`
- made `TranslatorFr` a real runtime-hosted AI capability path
- improved Chapter 13 reader smoke reliability and reduced CI fragility

### Coverage and quality

- raised business-logic coverage to `100%` for the tracked pure-logic crates:
  - Chapter 4 core
  - Chapter 5 service
  - Chapter 6 core service
  - Chapter 8 service graph
  - Chapter 9 trust boundaries
  - Chapter 10 tradeoffs
  - Chapter 11 evolution
  - Chapter 12 discoverable decisions
  - Chapter 13 portable MCP business logic surface

## 2026-03-24

### Public repo polish

- added `CODE_OF_CONDUCT.md`
- added `SECURITY.md`
- added `.github/CODEOWNERS`
- added `CITATION.cff`
- added repository presentation improvements for a public audience
