const scenarioSelect = document.getElementById("scenario-select");
const stepSelect = document.getElementById("step-select");
const runButton = document.getElementById("run-scenario");
const openCommandsButton = document.getElementById("open-commands");
const closeCommandsButton = document.getElementById("close-commands");
const commandsModal = document.getElementById("commands-modal");
const onboardingModal = document.getElementById("onboarding-modal");
const closeOnboardingButton = document.getElementById("close-onboarding");
const dismissOnboardingButton = document.getElementById("dismiss-onboarding");
const commandStack = document.getElementById("command-stack");
const scenarioFacts = document.getElementById("scenario-facts");
const transformationFlow = document.getElementById("transformation-flow");
const finalOutput = document.getElementById("final-output");
const graphScene = document.getElementById("graph-scene");
const graphStage = document.querySelector(".graph-stage");
const graphInspector = document.getElementById("graph-inspector");
const openReport = document.getElementById("open-report");
let currentReport = null;
let playbackTimer = null;
let isPlaying = false;
let graph = null;
let graphResizeObserver = null;
let currentPhase = "idle";
let graphLibraryPromise = null;
let graphRenderToken = 0;
const APP_HIDDEN_CAPABILITIES = new Set(["SummarizerBasic"]);
const ONBOARDING_STORAGE_KEY = "chapter13-ref-app-onboarding-dismissed";

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function workflowTitle(title) {
  const value = String(title);
  if (value.startsWith("Workflow:")) {
    return value;
  }
  return value.replace(/^Use Case\s+\d+:\s+/i, "Workflow: ");
}

function workflowTeachingMeta(report) {
  const workflowCapabilities = (report.selected_path || []).map((capability) => displayName(capability, capability));

  if (report.scenario === "use-case-2-ai-report") {
    return {
      startHere: true,
      proves:
        "How a workflow is composed from capabilities, with WASM MCP exposing options, Planner AI ranking them, UMA runtime validating them, and AI translation extending the path.",
      readerQuestion:
        "How does a reader get from a goal to a final French output without hardwiring the workflow?",
      workflowCapabilities,
    };
  }

  if (report.scenario === "use-case-5-agent-validation") {
    return {
      startHere: false,
      proves:
        "Planner AI is advisory, not authoritative. The UMA runtime can reject a proposed capability and still complete the workflow with a valid alternative.",
      readerQuestion:
        "What happens when the AI proposes a capability that violates the current constraints?",
      workflowCapabilities,
    };
  }

  if (report.scenario === "use-case-6-ai-executive-briefing") {
    return {
      startHere: false,
      proves:
        "The same capability system can produce a different output by composing a different workflow, without changing the runtime model.",
      readerQuestion:
        "How does the same runtime produce a different result when the goal changes?",
      workflowCapabilities,
    };
  }

  return {
    startHere: false,
    proves: "How the current workflow is composed from visible capabilities under runtime control.",
    readerQuestion: "How did the runtime choose this workflow?",
    workflowCapabilities,
  };
}

async function loadIndex() {
  const response = await fetch("./fixtures/index.json");
  if (!response.ok) {
    throw new Error("Missing app fixtures. Run ./scripts/export_app_fixtures.sh first.");
  }
  return response.json();
}

async function loadScenarioReport(id) {
  const response = await fetch(`./fixtures/${id}.json`);
  if (!response.ok) {
    throw new Error(`Missing fixture for ${id}.`);
  }
  return response.json();
}

function goalSummary(report) {
  return [
    report.goal.targetLanguage === "fr" ? "French output" : "English output",
    report.goal.localOnly ? "Local-only placement" : "Cross-runtime placement allowed",
    report.goal.preferAI ? "AI path preferred" : "Deterministic path preferred",
    report.goal.allowDegraded ? "Degraded mode allowed" : "Degraded mode disabled",
  ].join(" · ");
}

function formatInitialState(report) {
  const fragments = report.initial_context.sourceFragments || [];
  const preview = fragments
    .slice(0, 2)
    .map((item, index) => `Source ${index + 1}: ${item}`)
    .join("\n");
  const more = fragments.length > 2 ? `\n+ ${fragments.length - 2} more fragment(s)` : "";
  return `Project: ${report.initial_context.projectName}\n${preview}${more}`;
}

function commandRows(report) {
  return [
    {
      label: "Run this workflow",
      command: `./scripts/run_lab.sh ${report.scenario}`,
    },
    {
      label: "Validate this workflow",
      command: `./scripts/validate_lab.sh ${report.scenario}`,
    },
    {
      label: "Inspect workflow JSON",
      command: `cargo run --manifest-path rust/Cargo.toml -- render ${report.scenario} json`,
    },
  ];
}

function updateReportLink(report) {
  const reportHref = `./fixtures/${report.scenario}.json`;
  if (openReport) {
    openReport.href = reportHref;
  }
}

function renderCommands(report) {
  const meta = workflowTeachingMeta(report);
  scenarioFacts.innerHTML = `
    <p class="summary-text command-intro"><strong>${escapeHtml(workflowTitle(report.title))}</strong></p>
    ${meta.startHere ? `<p class="summary-text command-intro"><strong>Start here.</strong> This is the clearest first workflow for understanding how the Chapter 13 system works.</p>` : ""}
    <p class="summary-text command-intro">Workflow: a combination of capabilities for the current goal.</p>
    <p class="summary-text command-intro">Capability: a reusable execution unit that the runtime can discover, validate, and invoke.</p>
    <p class="summary-text command-intro"><strong>What this workflow proves:</strong> ${escapeHtml(meta.proves)}</p>
    <p class="summary-text command-intro"><strong>Reader question:</strong> ${escapeHtml(meta.readerQuestion)}</p>
    <p class="summary-text command-intro"><strong>Capabilities in this workflow:</strong> ${escapeHtml(meta.workflowCapabilities.join(" → "))}</p>
    <p class="summary-text command-intro">Goal: ${escapeHtml(report.goal.target)} · ${escapeHtml(goalSummary(report))} · Capabilities: ${report.selected_path.length}</p>
  `;

  commandStack.innerHTML = commandRows(report)
    .map(
      (item) => `
        <article class="command-card">
          <small>${escapeHtml(item.label)}</small>
          <pre><code>${escapeHtml(item.command)}</code></pre>
        </article>
      `
    )
    .join("");
}

function renderTransformations(report, focusedStepIndex, phase = "idle") {
  const meta = workflowTeachingMeta(report);
  const introCard = `
    <article class="transform-card workflow-overview">
      <div class="timeline-meta">
        <span class="pill">${meta.startHere ? "Start here" : "Workflow"}</span>
        <span class="pill">${escapeHtml(report.selected_path.length)} capabilities</span>
      </div>
      <h3>${escapeHtml(workflowTitle(report.title))}</h3>
      <p class="summary-text"><strong>What this workflow proves:</strong> ${escapeHtml(meta.proves)}</p>
      <p class="summary-text"><strong>Why a reader should care:</strong> ${escapeHtml(meta.readerQuestion)}</p>
      <p class="summary-text"><strong>Workflow path:</strong> ${escapeHtml(meta.workflowCapabilities.join(" → "))}</p>
    </article>
  `;

  transformationFlow.innerHTML = introCard + report.steps
    .map((step) => {
      const isFocused = step.index === focusedStepIndex;
      const cardPhase = isFocused ? phase : "idle";
      const phaseStrip = phaseItemsForStep(step, report)
        .map((item) => {
          const state = phaseItemState(item, cardPhase);
          return `<span class="phase-chip is-${state}">${escapeHtml(item.label)}</span>`;
        })
        .join("");
      const before = step.index === 1 ? formatInitialState(report) : report.steps[step.index - 2].output_preview;
      const rejected = step.discovery.rejected
        .filter(
          (item) =>
            !APP_HIDDEN_CAPABILITIES.has(item.capability) &&
            item.status !== "hidden" &&
            item.reason !== "does not contribute to the current unmet need"
        )
        .slice(0, 3)
        .map((item) => `<li><strong>${escapeHtml(item.capability)}</strong>: ${escapeHtml(item.reason ?? "none")}</li>`)
        .join("");
      const reasons = step.validation.reasons
        .map((reason) => `<li>${escapeHtml(reason)}</li>`)
        .join("");
      const proposalStatus = step.proposed_validation
        ? `<p class="summary-text proposal-reject"><strong>Proposal rejected:</strong> ${escapeHtml(step.proposed_validation.capability)} - ${escapeHtml(
            step.proposed_validation.reasons.join(", ")
          )}</p>`
        : "";
      const agentNote = step.agent_fallback_reason
        ? `<p class="summary-text proposal-reject"><strong>Planner fallback:</strong> ${escapeHtml(step.agent_fallback_reason)}</p>`
        : "";
      const fallbackNote = step.fallback_reason
        ? `<p class="summary-text proposal-reject"><strong>Fallback:</strong> ${escapeHtml(step.fallback_reason)}</p>`
        : "";
      const executionLog = executionLogEntries(step, cardPhase)
        .map(
          (entry) => `
            <li class="execution-log-item">
              <span class="execution-log-status is-${escapeHtml(entry.status)}">${escapeHtml(entry.status)}</span>
              <span><strong>${escapeHtml(entry.component)}</strong> ${escapeHtml(entry.text)}</span>
            </li>
          `
        )
        .join("");

      return `
        <article class="transform-card ${isFocused ? "is-focused" : ""}">
          <div class="timeline-meta">
            <span class="pill">Step ${step.index}</span>
            <span class="pill ${step.validation.status}">${escapeHtml(step.selected_capability)}</span>
            <span class="pill">${escapeHtml(step.need)}</span>
          </div>
          <h3>${escapeHtml(step.selected_capability)}</h3>
          <div class="phase-strip">${phaseStrip}</div>
          <p class="summary-text"><strong>Execution phase:</strong> ${escapeHtml(phaseLabel(cardPhase))}</p>
          <p class="summary-text"><strong>Control flow:</strong> ${escapeHtml(phaseExecutionSummary(step, cardPhase))}</p>
          <p class="summary-text"><strong>Planner AI:</strong> ${escapeHtml(step.agent_provider)} · ${escapeHtml(step.agent_mode)}</p>
          <p class="summary-text"><strong>Planner proposal:</strong> ${escapeHtml(step.agent_proposal ?? "none")}</p>
          <p class="summary-text"><strong>Why this capability:</strong> ${escapeHtml(selectionReason(step))}</p>
          <p class="summary-text"><strong>State change:</strong> ${escapeHtml(stateChangeSummary(step))}</p>
          ${agentNote}
          ${proposalStatus}
          ${fallbackNote}
          <div class="timeline-section">
            <small>Execution log</small>
            <ul class="execution-log">${executionLog}</ul>
          </div>
          <div class="transform-boxes">
            <div class="transform-box">
              <small>Input state</small>
              <pre>${escapeHtml(before)}</pre>
            </div>
            <div class="transform-arrow" aria-hidden="true">→</div>
            <div class="transform-box">
              <small>Output state</small>
              <pre>${escapeHtml(step.output_preview)}</pre>
            </div>
          </div>
          ${
            rejected ? `<div class="timeline-section"><small>Rejected during discovery</small><ul class="reason-list">${rejected}</ul></div>` : ""
          }
          ${
            reasons ? `<div class="timeline-section"><small>Validation reasons</small><ul class="reason-list">${reasons}</ul></div>` : ""
          }
        </article>
      `;
    })
    .join("");
}

function syncFocusedTransformationCard() {
  const focusedCard = transformationFlow.querySelector(".transform-card.is-focused");
  if (!focusedCard) {
    return;
  }
  focusedCard.scrollIntoView({
    block: "nearest",
    inline: "nearest",
    behavior: isPlaying ? "smooth" : "auto",
  });
}

function graphStatesForStep(report, focusedStepIndex) {
  const completedCapabilities = new Set();
  const rejectedCapabilities = new Set();

  for (const step of report.steps) {
    if (step.index < focusedStepIndex) {
      completedCapabilities.add(step.selected_capability);
      if (step.proposed_validation) {
        rejectedCapabilities.add(step.proposed_validation.capability);
      }
    }
    if (step.index === focusedStepIndex && step.proposed_validation) {
      rejectedCapabilities.add(step.proposed_validation.capability);
    }
  }

  const currentStep = report.steps.find((step) => step.index === focusedStepIndex) || null;
  const activeCapability = currentStep?.selected_capability ?? null;
  const proposedCapability = currentStep?.agent_proposal ?? null;
  const workflowNodes = report.steps.map((step) => step.selected_capability);

  const nodeStates = new Map();
  for (const node of report.graph_nodes) {
    if (APP_HIDDEN_CAPABILITIES.has(node.id)) {
      continue;
    }
    let state = "inactive";

    if (node.id === "goal") {
      state = "complete";
    } else if (node.id === "mcp" || node.id === "runtime") {
      state = "active";
    } else if (node.id === "agent") {
      state = proposedCapability ? "active" : "candidate";
    } else if (node.id === "result") {
      state = focusedStepIndex >= report.steps.length ? "complete" : "inactive";
    } else if (node.id === "PlannerAI") {
      if (completedCapabilities.has(node.id)) {
        state = "complete";
      } else if (currentStep && usesPlanner(currentStep)) {
        state = "candidate";
      } else if (report.initial_context.availableCapabilities.includes(node.id)) {
        state = "candidate";
      }
    } else if (completedCapabilities.has(node.id)) {
      state = "complete";
    } else if (node.id === activeCapability) {
      state = "active";
    } else if (rejectedCapabilities.has(node.id)) {
      state = "rejected";
    } else if (report.initial_context.availableCapabilities.includes(node.id)) {
      state = "candidate";
    }

    nodeStates.set(node.id, state);
  }

  if (proposedCapability && proposedCapability !== activeCapability && nodeStates.has(proposedCapability)) {
    nodeStates.set(proposedCapability, rejectedCapabilities.has(proposedCapability) ? "rejected" : "active");
  }

  const edgeStates = report.graph_edges.map((edge) => {
    let state = "inactive";
    if (edge.from === "goal" || edge.from === "mcp" || edge.from === "agent") {
      state = "active";
    }
    if (completedCapabilities.has(edge.to)) {
      state = "complete";
    }
    if (edge.to === activeCapability || edge.from === activeCapability) {
      state = "active";
    }
    if (edge.to === "result" && focusedStepIndex >= report.steps.length) {
      state = "complete";
    }
    return { ...edge, state };
  });

  const workflowEdges = workflowNodes.map((nodeId, index) => {
    const previousNode = index === 0 ? "goal" : workflowNodes[index - 1];
    let state = "inactive";
    if (index + 1 < focusedStepIndex) {
      state = "complete";
    } else if (index + 1 === focusedStepIndex) {
      state = "active";
    }
    return {
      from: previousNode,
      to: nodeId,
      state,
    };
  });

  if (workflowNodes.length > 0) {
    workflowEdges.push({
      from: workflowNodes[workflowNodes.length - 1],
      to: "result",
      state: focusedStepIndex >= report.steps.length ? "active" : "inactive",
    });
  }

  return { nodeStates, edgeStates, workflowEdges };
}

function usesPlanner(step) {
  return step?.agent_mode === "runtime-hosted-ranking";
}

function phasesForStep(step, report) {
  if (!step) {
    return ["capability"];
  }

  const phases = ["mcp"];
  if (usesPlanner(step)) {
    phases.push("planner");
  }
  phases.push("runtime", "capability");

  if (step.index === report.steps.length) {
    phases.push("result");
  }

  return phases;
}

function phaseLabel(phase) {
  switch (phase) {
    case "mcp":
      return "MCP running";
    case "planner":
      return "Planner running";
    case "runtime":
      return "Runtime running";
    case "capability":
      return "Capability running";
    case "result":
      return "Result produced";
    case "idle":
      return "Ready";
    default:
      return "Capability running";
  }
}

function phaseDuration(phase) {
  switch (phase) {
    case "mcp":
      return 260;
    case "planner":
      return 320;
    case "runtime":
      return 320;
    case "capability":
      return 520;
    case "result":
      return 600;
    default:
      return 420;
  }
}

const IDLE_COLOR = "#4e6383";
const IDLE_GLOW = "rgba(78, 99, 131, 0.24)";
const RUNNING_COLOR = "#ff9f2a";
const RUNNING_GLOW = "rgba(255, 159, 42, 0.28)";
const COMPLETE_COLOR = "#45d483";
const COMPLETE_GLOW = "rgba(69, 212, 131, 0.28)";

function stateFillColor(state) {
  if (state === "running") {
    return "rgba(255, 159, 42, 0.18)";
  }
  if (state === "complete") {
    return "rgba(69, 212, 131, 0.16)";
  }
  return "#1d2a40";
}

function stateStrokeColor(state) {
  if (state === "running") {
    return RUNNING_COLOR;
  }
  if (state === "complete") {
    return COMPLETE_COLOR;
  }
  return IDLE_COLOR;
}

function stateLabelColor(state) {
  if (state === "idle") {
    return "#e7eefb";
  }
  return "#eef5ff";
}

function phaseExecutionSummary(step, phase) {
  switch (phase) {
    case "mcp":
      return "WASM MCP inspects the goal and current capability contracts.";
    case "planner":
      return `Planner AI ranks candidate capabilities for "${step.need}".`;
    case "runtime":
      return "UMA runtime validates the selected capability against contracts and constraints.";
    case "capability":
      return `${step.selected_capability} executes and publishes the next state.`;
    case "result":
      return "UMA runtime emits the final result.";
    default:
      return "Ready to execute this step.";
  }
}

function phaseItemsForStep(step, report) {
  const items = [
    { id: "mcp", label: "WASM MCP" },
    { id: "planner", label: "Planner AI", skipped: !usesPlanner(step) },
    { id: "runtime", label: "UMA runtime" },
    { id: "capability", label: step.selected_capability },
  ];

  if (step.index === report.steps.length) {
    items.push({ id: "result", label: "Result" });
  }

  return items;
}

function phaseItemState(item, currentPhase) {
  if (currentPhase === "idle") {
    return "idle";
  }
  if (item.skipped) {
    return "skipped";
  }

  const phaseOrder = ["mcp", "planner", "runtime", "capability", "result"];
  const itemIndex = phaseOrder.indexOf(item.id);
  const currentIndex = phaseOrder.indexOf(currentPhase);

  if (currentPhase === "result") {
    return itemIndex > -1 && itemIndex <= currentIndex ? "complete" : "idle";
  }

  if (item.id === currentPhase) {
    return "running";
  }
  if (itemIndex > -1 && currentIndex > -1 && itemIndex < currentIndex) {
    return "complete";
  }
  return "idle";
}

function plainStatusLabel(step, itemId, phase) {
  const state = phaseItemState(
    { id: itemId, skipped: itemId === "planner" && !usesPlanner(step) },
    phase
  );
  if (state === "running") {
    return "running";
  }
  if (state === "complete") {
    return "completed";
  }
  if (state === "skipped") {
    return "skipped";
  }
  return "waiting";
}

function candidateList(step) {
  return (
    (step.discovery.available || [])
      .map((item) => item.capability)
      .filter((capability) => !APP_HIDDEN_CAPABILITIES.has(capability))
      .join(", ") || "none"
  );
}

function validationSummary(step) {
  if (step.validation.status === "accepted") {
    return "accepted";
  }
  return step.validation.reasons.join(", ") || step.validation.status;
}

function selectionReason(step) {
  if (step.proposed_validation) {
    return `Planner AI proposed ${step.proposed_validation.capability}, but UMA runtime rejected it and selected ${step.selected_capability}.`;
  }
  if (usesPlanner(step)) {
    return `Planner AI ranked the visible candidates and UMA runtime accepted ${step.selected_capability}.`;
  }
  return `WASM MCP exposed one valid capability for this need, so UMA runtime selected ${step.selected_capability} directly.`;
}

function stateChangeSummary(step) {
  return step.output_preview || "The workflow state advanced.";
}

function executionLogEntries(step, phase) {
  const entries = [
    {
      component: "WASM MCP",
      status: plainStatusLabel(step, "mcp", phase),
      text: `inspected the unmet need "${step.need}" and exposed candidate capabilities: ${candidateList(step)}.`,
    },
  ];

  if (usesPlanner(step)) {
    entries.push({
      component: "Planner AI",
      status: plainStatusLabel(step, "planner", phase),
      text: step.proposed_validation
        ? `proposed ${step.agent_proposal ?? "none"}, but the runtime later rejected that proposal.`
        : `ranked the visible candidates and proposed ${step.agent_proposal ?? step.selected_capability}.`,
    });
  } else {
    entries.push({
      component: "Planner AI",
      status: "skipped",
      text: "was not needed because the runtime had a direct valid selection.",
    });
  }

  entries.push({
    component: "UMA runtime",
    status: plainStatusLabel(step, "runtime", phase),
    text: `validated ${step.selected_capability} against the current contracts and constraints: ${validationSummary(step)}.`,
  });

  entries.push({
    component: step.selected_capability,
    status: plainStatusLabel(step, "capability", phase),
    text: step.fallback_reason
      ? `executed through ${step.execution_provider} in ${step.execution_mode} mode, with fallback: ${step.fallback_reason}.`
      : `executed through ${step.execution_provider} in ${step.execution_mode} mode and produced the next state.`,
  });

  if (step.proposed_validation) {
    entries.push({
      component: "Runtime verdict",
      status: "completed",
      text: `rejected ${step.proposed_validation.capability} because ${step.proposed_validation.reasons.join(", ")}.`,
    });
  }

  return entries;
}

function isAIAgentNode(nodeId) {
  return nodeId === "agent" || nodeId === "PlannerAI";
}

function isAICapabilityNode(nodeId) {
  return nodeId === "SummarizerAI" || nodeId === "PlannerAI" || nodeId === "TranslatorFr";
}

function displayName(nodeId, fallbackLabel) {
  if (nodeId === "agent" || nodeId === "PlannerAI") {
    return "Planner AI";
  }
  if (nodeId === "SummarizerAI") {
    return "Summarizer AI";
  }
  if (nodeId === "TranslatorFr") {
    return "Translator AI";
  }
  return fallbackLabel;
}

function labelWithRoleIcon(nodeId, label) {
  return label;
}

function graphNodeLabel(nodeId, label, isAI) {
  if (nodeId === "goal") {
    return stackedNodeLabel("◎", label);
  }
  if (nodeId === "result") {
    return stackedNodeLabel("✓", label);
  }
  if (nodeId === "mcp") {
    return "WASM\nMCP";
  }
  if (nodeId === "runtime") {
    return "UMA\nRuntime";
  }
  if (isAI) {
    return `${label}\nAGENT`;
  }
  return `${label}\nMICROSERVICE`;
}

function specialNodeIconFill({ active = false, complete = false, darkFill = false }) {
  if (active) {
    return "#08352a";
  }
  if (complete && darkFill) {
    return "#ffffff";
  }
  if (complete) {
    return "#241b07";
  }
  return darkFill ? "rgba(255, 255, 255, 0.94)" : "#3a2d09";
}

function stackedNodeLabel(icon, label) {
  return `${icon}\n${label}`;
}

function nodeFootprint(nodeId) {
  if (nodeId === "goal" || nodeId === "result") {
    return { width: 112, height: 112 };
  }
  if (nodeId === "mcp" || nodeId === "runtime") {
    return { width: 104, height: 104 };
  }
  if (isAICapabilityNode(nodeId) || nodeId === "PlannerAI") {
    return { width: 124, height: 124 };
  }
  return { width: 188, height: 86 };
}

function resolveLayoutCollisions(layout, lockedIds) {
  const nodes = [...layout.entries()].map(([id, point]) => ({
    id,
    x: point.x,
    y: point.y,
    role: point.role,
    locked: lockedIds.has(id),
    ...nodeFootprint(id),
  }));

  const minGap = 22;
  for (let iteration = 0; iteration < 120; iteration += 1) {
    let moved = false;
    for (let i = 0; i < nodes.length; i += 1) {
      for (let j = i + 1; j < nodes.length; j += 1) {
        const a = nodes[i];
        const b = nodes[j];
        const dx = b.x - a.x;
        const dy = b.y - a.y;
        const minX = (a.width + b.width) / 2 + minGap;
        const minY = (a.height + b.height) / 2 + minGap;
        if (Math.abs(dx) >= minX || Math.abs(dy) >= minY) {
          continue;
        }

        const overlapX = minX - Math.abs(dx);
        const overlapY = minY - Math.abs(dy);
        const pushX = overlapX / 2 + 1;
        const pushY = overlapY / 2 + 1;
        const signX = dx === 0 ? (i % 2 === 0 ? -1 : 1) : Math.sign(dx);
        const signY = dy === 0 ? (j % 2 === 0 ? -1 : 1) : Math.sign(dy);

        if (!a.locked) {
          a.x -= signX * pushX;
          a.y -= signY * pushY;
          moved = true;
        }
        if (!b.locked) {
          b.x += signX * pushX;
          b.y += signY * pushY;
          moved = true;
        }
      }
    }

    if (!moved) {
      break;
    }
  }

  const resolved = new Map();
  for (const node of nodes) {
    resolved.set(node.id, {
      x: node.x,
      y: node.y,
      role: node.role,
    });
  }
  return resolved;
}

function layoutGraph(report) {
  const layout = new Map();
  const stageWidth = graphStage?.clientWidth || graphScene.clientWidth || 760;
  const stageHeight = graphStage?.clientHeight || graphScene.clientHeight || 760;
  const centerX = Math.round(stageWidth / 2);
  const centerY = Math.round(stageHeight / 2) + 12;
  const goalY = 118;
  const mcpY = Math.max(212, centerY - 132);
  const runtimeY = centerY;
  const resultY = stageHeight - 132;
  const serviceX = centerX - Math.min(280, Math.round(stageWidth * 0.28));
  const aiX = centerX + Math.min(230, Math.round(stageWidth * 0.22));

  const supportNodes = [
    ["goal", { x: centerX, y: goalY, role: "support-start" }],
    ["mcp", { x: centerX, y: mcpY, role: "support" }],
    ["runtime", { x: centerX, y: runtimeY, role: "support" }],
    ["result", { x: centerX, y: resultY, role: "result" }],
  ];

  for (const [id, point] of supportNodes) {
    layout.set(id, point);
  }

  const serviceNodes = report.graph_nodes
    .filter(
      (item) =>
        item.kind === "capability" &&
        !APP_HIDDEN_CAPABILITIES.has(item.id) &&
        !isAICapabilityNode(item.id)
    )
    .map((item) => item.id);
  const aiNodes = report.graph_nodes
    .filter(
      (item) =>
        item.kind === "capability" &&
        !APP_HIDDEN_CAPABILITIES.has(item.id) &&
        isAICapabilityNode(item.id)
    )
    .map((item) => item.id);

  const serviceSlots = [
    { x: serviceX, y: centerY - 144 },
    { x: serviceX, y: centerY - 24 },
    { x: serviceX, y: centerY + 96 },
    { x: serviceX, y: centerY + 216 },
    { x: centerX - 108, y: centerY + 252 },
  ];
  const aiSlots = [
    { x: aiX, y: centerY - 140 },
    { x: aiX, y: centerY + 12 },
    { x: aiX, y: centerY + 164 },
  ];

  for (const [index, nodeId] of serviceNodes.entries()) {
    const point = serviceSlots[index] ?? {
      x: centerX - 272,
      y: centerY - 132 + index * 108,
    };
    layout.set(nodeId, {
      x: point.x,
      y: Math.min(point.y, stageHeight - 96),
      role: "service-capability",
    });
  }

  for (const [index, nodeId] of aiNodes.entries()) {
    const point = aiSlots[index] ?? {
      x: centerX + 232,
      y: centerY - 132 + index * 132,
    };
    layout.set(nodeId, {
      x: point.x,
      y: Math.min(point.y, stageHeight - 96),
      role: "ai-capability",
    });
  }

  for (const node of report.graph_nodes.filter(
    (item) => item.kind === "capability" && !APP_HIDDEN_CAPABILITIES.has(item.id)
  )) {
    if (layout.has(node.id)) {
      continue;
    }
    const point = { x: serviceX, y: centerY };
    layout.set(node.id, {
      x: point.x,
      y: point.y,
      role: "capability",
    });
  }

  return resolveLayoutCollisions(layout, new Set(["goal", "mcp", "runtime", "result"]));
}

async function loadGraphLibrary() {
  if (!graphLibraryPromise) {
    graphLibraryPromise = import("@antv/g6");
  }
  return graphLibraryPromise;
}

async function ensureGraph() {
  if (graph) {
    return graph;
  }

  const { Graph } = await loadGraphLibrary();

  const width = graphScene.clientWidth || 760;
  const height = graphScene.clientHeight || 560;

  graph = new Graph({
    container: graphScene,
    width,
    height,
    animation: true,
    autoFit: false,
    background: "#11131a",
    behaviors: ["drag-canvas", "zoom-canvas"],
    node: {
      style: {
        lineWidth: 1.5,
        shadowBlur: 18,
        shadowColor: "rgba(98, 87, 255, 0.18)",
        label: true,
        labelFontFamily: "IBM Plex Sans, system-ui, sans-serif",
      },
    },
    edge: {
      style: {
        lineWidth: 2,
        endArrow: true,
        strokeOpacity: 1,
      },
    },
  });

  if (!graphResizeObserver) {
    graphResizeObserver = new ResizeObserver(() => {
      if (!graph) {
        return;
      }
      const nextWidth = graphScene.clientWidth || 760;
      const nextHeight = graphScene.clientHeight || 560;
      graph.setSize(nextWidth, nextHeight);
    });
    graphResizeObserver.observe(graphScene);
  }

  return graph;
}

function buildGraphData(report, focusedStepIndex, phase = "capability") {
  const { nodeStates } = graphStatesForStep(report, focusedStepIndex);
  const currentStep = report.steps.find((step) => step.index === focusedStepIndex) || null;
  const layout = layoutGraph(report);
  const selected = (report.selected_path || []).filter(
    (capabilityId) => layout.has(capabilityId) && !APP_HIDDEN_CAPABILITIES.has(capabilityId)
  );
  const nodes = [];
  const currentProposalCapability = currentStep?.proposed_validation?.capability ?? null;
  const currentAgentProposal = currentStep?.agent_proposal ?? null;
  const supportPath = ["goal", "mcp", "runtime"];
  const activeSupportNode =
    phase === "mcp" ? "mcp" :
    phase === "runtime" ? "runtime" :
    null;
  const activeSupportIndex = activeSupportNode ? supportPath.indexOf(activeSupportNode) : -1;

  const supportNodeIds = ["goal", "mcp", "runtime"];
  for (const nodeId of supportNodeIds) {
    const source = report.graph_nodes.find((item) => item.id === nodeId);
    const point = layout.get(nodeId);
    if (!source || !point) {
      continue;
    }
    const label =
      nodeId === "mcp"
        ? "WASM MCP"
        : nodeId === "runtime"
            ? "UMA runtime"
            : displayName(nodeId, source.label);
    const onPath = supportPath.includes(nodeId);
    const pathIndex = supportPath.indexOf(nodeId);
    const active = nodeId === activeSupportNode;
    const complete =
      phase !== "idle" &&
      (
        nodeId === "goal" ||
        ((phase === "capability" || phase === "result") && onPath) ||
        (activeSupportIndex > -1 && onPath && pathIndex > -1 && pathIndex < activeSupportIndex)
      );
    const visualState = active ? "running" : complete ? "complete" : "idle";
    nodes.push({
      id: nodeId,
      type: nodeId === "goal" ? "diamond" : nodeId === "mcp" ? "rect" : nodeId === "result" ? "diamond" : "hexagon",
      data: { label },
      style: {
        x: point.x,
        y: point.y,
        size: nodeId === "goal" ? 84 : nodeId === "runtime" ? 124 : nodeId === "mcp" ? [90, 90] : 74,
        radius: nodeId === "mcp" ? 12 : undefined,
        fill: stateFillColor(visualState),
        stroke: stateStrokeColor(visualState),
        shadowBlur: active ? 28 : complete ? 18 : 8,
        shadowColor: active ? RUNNING_GLOW : complete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: graphNodeLabel(nodeId, label, false),
        labelPlacement: "center",
        labelFill: stateLabelColor(visualState),
        labelFontSize: nodeId === "runtime" ? 13 : 10,
        labelFontWeight: active || complete ? 700 : 500,
        labelLineHeight: nodeId === "runtime" ? 18 : 15,
      },
    });
  }

  const visibleRingNodes = report.graph_nodes.filter(
    (node) => node.kind === "capability" && !APP_HIDDEN_CAPABILITIES.has(node.id)
  );
  const plannerUsedBeforeCurrentStep = report.steps.some((step) => step.index < focusedStepIndex && usesPlanner(step));

  for (const source of visibleRingNodes) {
    const nodeId = source.id;
    const point = layout.get(nodeId);
    if (!source || !point) {
      continue;
    }
    const state = nodeStates.get(nodeId) || "inactive";
    const isPlannerPhaseNode = phase === "planner" && nodeId === "PlannerAI";
    const plannerCompleted = nodeId === "PlannerAI" && (plannerUsedBeforeCurrentStep || (currentStep && usesPlanner(currentStep) && (phase === "runtime" || phase === "capability" || phase === "result")));
    const isCurrent = (nodeId === currentStep?.selected_capability && phase === "capability") || isPlannerPhaseNode;
    const isComplete =
      phase !== "idle" &&
      (
        state === "complete" ||
        plannerCompleted ||
        (phase === "result" && nodeId === currentStep?.selected_capability)
      );
    const isAvailable = report.initial_context.availableCapabilities.includes(nodeId);
    const isAI = isAICapabilityNode(nodeId);
    const visualState = isCurrent ? "running" : isComplete ? "complete" : "idle";
    nodes.push({
      id: nodeId,
      type: isAI ? "circle" : "rect",
      data: { label: displayName(nodeId, source.label) },
      style: {
        x: point.x,
        y: point.y,
        size: isAI ? 108 : [164, 64],
        radius: isAI ? undefined : 18,
        fill: stateFillColor(visualState),
        stroke: stateStrokeColor(visualState),
        lineWidth: isCurrent ? 3 : isComplete ? 2.5 : 1.5,
        shadowBlur: isCurrent ? 28 : isComplete ? 18 : 8,
        shadowColor: isCurrent ? RUNNING_GLOW : isComplete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: graphNodeLabel(
          nodeId,
          labelWithRoleIcon(nodeId, displayName(nodeId, source.label)),
          isAI
        ),
        labelPlacement: "center",
        labelFill: stateLabelColor(visualState),
        labelFontSize: isAI ? 12 : 12,
        labelFontWeight: isCurrent ? 700 : 600,
        labelLineHeight: isAI ? 16 : 16,
        opacity: isAvailable ? 1 : 0.72,
      },
    });
  }

  const resultPoint = layout.get("result");
  if (resultPoint) {
    const resultComplete = phase === "result" && focusedStepIndex >= report.steps.length;
    const visualState = resultComplete ? "complete" : "idle";
    nodes.push({
      id: "result",
      type: "diamond",
      data: { label: "Result" },
      style: {
        x: resultPoint.x,
        y: resultPoint.y,
        size: 84,
        fill: stateFillColor(visualState),
        stroke: stateStrokeColor(visualState),
        shadowBlur: resultComplete ? 18 : 8,
        shadowColor: resultComplete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: graphNodeLabel("result", "Result", false),
        labelPlacement: "center",
        labelFill: stateLabelColor(visualState),
        labelFontSize: 11,
        labelFontWeight: 700,
        labelLineHeight: 16,
      },
    });
  }

  const edges = [];
  for (const capabilityId of report.graph_nodes
    .filter((item) => item.kind === "capability" && !APP_HIDDEN_CAPABILITIES.has(item.id))
    .map((item) => item.id)) {
    edges.push({
      id: `runtime-capability-${capabilityId}`,
      source: "runtime",
      target: capabilityId,
      style: {
        stroke: IDLE_COLOR,
        lineWidth: 1.5,
        lineDash: [],
        endArrow: false,
      },
    });
  }

  if (currentProposalCapability && layout.has(currentProposalCapability)) {
    edges.push({
      id: `runtime-capability-proposed-${currentProposalCapability}`,
      source: "runtime",
      target: currentProposalCapability,
      style: {
        stroke: IDLE_COLOR,
        lineWidth: 1.5,
        lineDash: [4, 6],
      },
    });
  }

  if (currentAgentProposal && layout.has(currentAgentProposal) && currentAgentProposal !== currentProposalCapability) {
    edges.push({
      id: `runtime-capability-agent-${currentAgentProposal}`,
      source: "runtime",
      target: currentAgentProposal,
      style: {
        stroke: IDLE_COLOR,
        lineWidth: 1.5,
        lineDash: [4, 6],
      },
    });
  }

  const architectureEdges = [
    ["goal", "mcp"],
    ["mcp", "runtime"],
  ];
  for (const [source, target] of architectureEdges) {
    const sourceIndex = supportPath.indexOf(source);
    const targetIndex = supportPath.indexOf(target);
    const onPath = sourceIndex > -1 && targetIndex > -1;
    const active = onPath && activeSupportIndex > -1 && sourceIndex < activeSupportIndex && targetIndex === activeSupportIndex;
    const complete = phase !== "idle" && onPath && (phase === "capability" || phase === "result" || (activeSupportIndex > -1 && targetIndex < activeSupportIndex));
    edges.push({
      id: `support-${source}-${target}`,
      source,
      target,
      style: {
        stroke: active ? RUNNING_COLOR : complete ? COMPLETE_COLOR : IDLE_COLOR,
        lineWidth: active ? 6 : complete ? 4 : 2,
        shadowBlur: active ? 18 : complete ? 10 : 0,
        shadowColor: active ? RUNNING_GLOW : complete ? COMPLETE_GLOW : IDLE_GLOW,
        endArrow: active || complete ? true : false,
      },
    });
  }

  selected.forEach((nodeId, index) => {
    const stepNumber = index + 1;
    const complete = phase !== "idle" && stepNumber < focusedStepIndex;
    const active = stepNumber === focusedStepIndex && phase === "capability";
    edges.push({
      id: `workflow-runtime-${nodeId}`,
      source: "runtime",
      target: nodeId,
      style: {
        stroke: active ? RUNNING_COLOR : complete ? COMPLETE_COLOR : IDLE_COLOR,
        lineWidth: active ? 6 : complete ? 4 : 2,
        shadowBlur: active ? 24 : complete ? 10 : 0,
        shadowColor: active ? RUNNING_GLOW : complete ? COMPLETE_GLOW : "rgba(17, 19, 26, 0)",
        endArrow: true,
      },
    });
  });

  edges.push({
    id: "workflow-runtime-result",
    source: "runtime",
    target: "result",
    style: {
      stroke: phase === "result" && focusedStepIndex >= report.steps.length ? COMPLETE_COLOR : IDLE_COLOR,
      lineWidth: phase === "result" && focusedStepIndex >= report.steps.length ? 4 : 2,
      shadowBlur: phase === "result" && focusedStepIndex >= report.steps.length ? 10 : 0,
      shadowColor: phase === "result" && focusedStepIndex >= report.steps.length ? COMPLETE_GLOW : "rgba(17, 19, 26, 0)",
      endArrow: true,
    },
  });

  if (currentStep?.proposed_validation?.capability && layout.has(currentStep.proposed_validation.capability)) {
    edges.push({
      id: `rejected-${currentStep.proposed_validation.capability}`,
      source: "runtime",
      target: currentStep.proposed_validation.capability,
      style: {
        stroke: IDLE_COLOR,
        lineWidth: 2,
        lineDash: [6, 4],
        shadowBlur: 0,
        shadowColor: "rgba(17, 19, 26, 0)",
      },
    });
  }

  return { nodes, edges };
}

async function renderGraph(report, focusedStepIndex, phase = currentPhase) {
  const token = ++graphRenderToken;
  const instance = await ensureGraph();
  if (token !== graphRenderToken) {
    return;
  }
  const data = buildGraphData(report, focusedStepIndex, phase);
  try {
    instance.setData(data);
    instance.render();
  } catch (error) {
    graphScene.innerHTML = `<div class="graph-error">${escapeHtml(error.message)}</div>`;
  }
}

function renderGraphInspector(report, focusedStepIndex, phase = currentPhase) {
  const currentStep = report.steps.find((step) => step.index === focusedStepIndex) || null;
  const rejected = report.rejected_capabilities
    .filter((item) => !APP_HIDDEN_CAPABILITIES.has(item.capability)).length
    ? report.rejected_capabilities
        .filter((item) => !APP_HIDDEN_CAPABILITIES.has(item.capability))
        .map(
          (item) =>
            `<li><strong>${escapeHtml(item.capability)}</strong><span>${escapeHtml(item.reasons.join(", "))}</span></li>`
        )
        .join("")
    : `<li><span>No runtime rejections in this workflow.</span></li>`;

  graphInspector.innerHTML = `
    <div class="inspector-block">
      <small>Current focus</small>
      <p class="summary-text"><strong>${escapeHtml(currentStep ? displayName(currentStep.selected_capability, currentStep.selected_capability) : "Complete")}</strong>${currentStep ? ` · ${escapeHtml(currentStep.need)}` : ""}</p>
      <p class="summary-text"><strong>Phase:</strong> ${escapeHtml(phaseLabel(phase))}</p>
      <small>Runtime verdict</small>
      <ul class="inspector-list">${rejected}</ul>
    </div>
  `;
}

function renderOutput(report) {
  finalOutput.textContent = report.final_output;
}

async function renderScenario(id) {
  const report = await loadScenarioReport(id);
  currentReport = report;
  updateReportLink(report);
  renderCommands(report);
  stepSelect.innerHTML = report.steps
    .map((step) => `<option value="${step.index}">Step ${step.index}: ${escapeHtml(step.selected_capability)}</option>`)
    .join("");
  const focusedStepIndex = report.steps[0]?.index ?? 1;
  stepSelect.value = String(focusedStepIndex);
  renderStep(report, focusedStepIndex, "idle");
}

function renderStep(report, focusedStepIndex, phase = "capability") {
  currentPhase = phase;
  stepSelect.value = String(focusedStepIndex);
  renderTransformations(report, focusedStepIndex, phase);
  renderGraph(report, focusedStepIndex, phase);
  renderGraphInspector(report, focusedStepIndex, phase);
  renderOutput(report);
  window.requestAnimationFrame(syncFocusedTransformationCard);
}

function openCommandsModal() {
  if (!commandsModal) {
    return;
  }
  commandsModal.hidden = false;
  document.body.classList.add("modal-open");
}

function closeCommandsModal() {
  if (!commandsModal) {
    return;
  }
  commandsModal.hidden = true;
  if (!onboardingModal || onboardingModal.hidden) {
    document.body.classList.remove("modal-open");
  }
}

function openOnboardingModal() {
  if (!onboardingModal) {
    return;
  }
  onboardingModal.hidden = false;
  document.body.classList.add("modal-open");
}

function closeOnboardingModal({ persist = true } = {}) {
  if (!onboardingModal) {
    return;
  }
  onboardingModal.hidden = true;
  if (!commandsModal || commandsModal.hidden) {
    document.body.classList.remove("modal-open");
  }
  if (persist) {
    window.localStorage.setItem(ONBOARDING_STORAGE_KEY, "true");
  }
}

function shouldShowOnboarding() {
  return window.localStorage.getItem(ONBOARDING_STORAGE_KEY) !== "true";
}

function stopPlayback() {
  if (playbackTimer) {
    window.clearTimeout(playbackTimer);
    playbackTimer = null;
  }
  isPlaying = false;
  currentPhase = "idle";
  runButton.textContent = "Play workflow";
}

function schedulePlayback(report, stepIndex, phaseIndex = 0) {
  const step = report.steps.find((item) => item.index === stepIndex);
  if (!step) {
    stopPlayback();
    return;
  }

  const phases = phasesForStep(step, report);
  const phase = phases[phaseIndex] || "capability";
  renderStep(report, stepIndex, phase);

  playbackTimer = window.setTimeout(() => {
    const nextPhaseIndex = phaseIndex + 1;
    if (nextPhaseIndex < phases.length) {
      schedulePlayback(report, stepIndex, nextPhaseIndex);
      return;
    }

    if (stepIndex >= report.steps.length) {
      stopPlayback();
      return;
    }
    schedulePlayback(report, stepIndex + 1, 0);
  }, phaseDuration(phase));
}

function startPlayback(report) {
  if (!report?.steps?.length) {
    return;
  }
  stopPlayback();
  isPlaying = true;
  runButton.textContent = "Stop playback";
  schedulePlayback(report, report.steps[0].index, 0);
}

async function init() {
  try {
    const items = await loadIndex();
    for (const item of items) {
      const option = document.createElement("option");
      option.value = item.id;
      option.textContent = workflowTitle(item.title);
      scenarioSelect.appendChild(option);
    }
    if (items[0]) {
      await renderScenario(items[0].id);
      runButton.textContent = "Play workflow";
      if (shouldShowOnboarding()) {
        openOnboardingModal();
      }
    }
  } catch (error) {
    transformationFlow.innerHTML = `<p class="summary-text">${escapeHtml(error.message)}</p>`;
  }
}

runButton.addEventListener("click", async () => {
  if (!currentReport || currentReport.scenario !== scenarioSelect.value) {
    await renderScenario(scenarioSelect.value);
  }
  if (isPlaying) {
    stopPlayback();
    return;
  }
  startPlayback(currentReport);
});

scenarioSelect.addEventListener("change", async () => {
  stopPlayback();
  await renderScenario(scenarioSelect.value);
});

stepSelect.addEventListener("change", () => {
  if (!currentReport) {
    return;
  }
  stopPlayback();
  renderStep(currentReport, Number(stepSelect.value), "idle");
});

if (openCommandsButton) {
  openCommandsButton.addEventListener("click", openCommandsModal);
}

if (closeCommandsButton) {
  closeCommandsButton.addEventListener("click", closeCommandsModal);
}

if (closeOnboardingButton) {
  closeOnboardingButton.addEventListener("click", () => closeOnboardingModal());
}

if (dismissOnboardingButton) {
  dismissOnboardingButton.addEventListener("click", () => closeOnboardingModal());
}

if (commandsModal) {
  commandsModal.addEventListener("click", (event) => {
    if (event.target.hasAttribute("data-close-commands")) {
      closeCommandsModal();
    }
  });
}

if (onboardingModal) {
  onboardingModal.addEventListener("click", (event) => {
    if (event.target.hasAttribute("data-close-onboarding")) {
      closeOnboardingModal();
    }
  });
}

window.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    closeCommandsModal();
    closeOnboardingModal({ persist: false });
  }
});

init();
