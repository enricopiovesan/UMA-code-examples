# Universal Microservices vs Traditional Microservices

Traditional microservices focus on independently deployable services. Universal Microservices focus on portable capabilities whose behavior can remain consistent across runtime surfaces.

Both models can coexist. UMA does not reject microservices; it narrows attention to the parts of a system that need portability, runtime governance, and explicit execution evidence.

## Comparison

| Concern | Traditional microservices | Universal Microservices Architecture |
| --- | --- | --- |
| Primary boundary | Deployable service | Portable capability plus runtime boundary |
| Main unit of ownership | Service team, API, infrastructure | Capability contract, runtime policy, execution trace |
| Portability | Usually achieved by reimplementation or shared libraries | Designed into the capability boundary |
| Runtime decision | Often implicit in deployment topology | Explicit and visible to the system |
| Governance | API gateways, service mesh, platform controls | Runtime validation, trust rules, approval, and trace |
| Drift risk | Logic can diverge across frontend, backend, edge, and workflows | Drift is reduced by keeping behavior in one portable core |

## Where Traditional Microservices Still Fit

A conventional microservice is often the right answer when:

- a single backend service owns the behavior cleanly
- the behavior does not need to move between runtimes
- platform-specific behavior is more important than portability
- runtime-level decision evidence would add unnecessary ceremony

UMA is for the cases where the same behavior is likely to be copied across surfaces or where execution authority must be visible.

## Where UMA Adds Value

UMA becomes useful when a system needs:

- portable business logic
- WebAssembly execution or parity testing
- explicit trust boundaries
- capability discovery
- runtime-approved workflows
- readable execution traces

The examples in this repository are intentionally small because the architectural concern is the boundary, not the size of the service.

## Related Reading

- [What Is Universal Microservices?](what-is-universal-microservices.md)
- [Portable Microservices](portable-microservices.md)
- [UMA vs traditional microservices](https://www.universalmicroservices.com/uma-vs-traditional-microservices/)
- [Chapter 4: Feature Flag Evaluator](../chapter-04-feature-flag-evaluator/README.md)
- [Chapter 13: Portable MCP Runtime](../chapter-13-portable-mcp-runtime/README.md)

