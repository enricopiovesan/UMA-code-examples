# UI Specification

## Layout

The app uses a split-screen layout.

Left panel:
- goal and initial context
- discovery result
- proposal
- validation
- execution steps
- final output

Right panel:
- a minimal 3D graph of the runtime surfaces
- MCP node
- agent
- runtime
- capability nodes
- active path highlighting

## Visual priorities

The UI must prioritize:

- clarity
- synchronized left and right views
- visible runtime decisions
- visible rejected capabilities and reasons
- readable final output

## Runtime states

Recommended app states:

- idle
- discovery
- proposal
- validation
- execution
- complete
- degraded
