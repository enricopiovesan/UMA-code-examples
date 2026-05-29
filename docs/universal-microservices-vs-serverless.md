# Universal Microservices vs Serverless

Serverless platforms simplify deployment, scaling, and operational ownership for functions and services. Universal Microservices Architecture (UMA) addresses a different question: how can a capability keep the same behavior, contract, and governance model as execution moves across runtime surfaces?

UMA can use serverless infrastructure. It is not a replacement for it.

## Comparison

| Concern | Serverless | Universal Microservices Architecture |
| --- | --- | --- |
| Primary concern | Managed execution and scaling | Portable behavior and runtime authority |
| Deployment model | Provider-managed functions or containers | Capability packaged for one or more host runtimes |
| Portability | Often tied to provider APIs and event shapes | Preserved through explicit contracts and portable cores |
| Governance | IAM, platform policy, gateway controls | Runtime validation, trust boundaries, approval, and traces |
| Typical risk | Provider coupling and event-shape sprawl | More up-front modeling of contracts and runtime roles |

## When Serverless Is Enough

Serverless is usually enough when:

- the logic belongs in one cloud provider context
- scaling and deployment are the dominant concerns
- provider-specific integrations are acceptable
- there is no need to run the same behavior in browser, edge, backend, and workflow contexts

## When UMA Is A Better Lens

UMA is useful when:

- a capability needs to run in more than one place
- runtime-specific glue is causing duplicated business behavior
- WebAssembly portability is part of the execution strategy
- agent-generated proposals need runtime validation before execution
- traceability matters as much as successful execution

In this repository, [Chapter 6](../chapter-06-portability-lab/README.md) focuses on proving portability. [Chapter 13](../chapter-13-portable-mcp-runtime/README.md) shows how a runtime can validate and execute discovered capabilities.

## Related Reading

- [What Is Universal Microservices?](what-is-universal-microservices.md)
- [WebAssembly Portability](webassembly-portability.md)
- [What belongs in the runtime layer?](https://www.universalmicroservices.com/what-belongs-in-the-runtime-layer/)
- [Book page](https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4)

