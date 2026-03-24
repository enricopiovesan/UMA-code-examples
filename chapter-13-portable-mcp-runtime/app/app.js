const scenarioSelect = document.getElementById("scenario-select");
const stepSelect = document.getElementById("step-select");
const runButton = document.getElementById("run-scenario");
const openCommandsButton = document.getElementById("open-commands");
const closeCommandsButton = document.getElementById("close-commands");
const commandsModal = document.getElementById("commands-modal");
const commandStack = document.getElementById("command-stack");
const scenarioFacts = document.getElementById("scenario-facts");
const transformationFlow = document.getElementById("transformation-flow");
const finalOutput = document.getElementById("final-output");
const graphScene = document.getElementById("graph-scene");
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

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
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
      label: "Run this scenario",
      command: `./scripts/run_lab.sh ${report.scenario}`,
    },
    {
      label: "Validate this scenario",
      command: `./scripts/validate_lab.sh ${report.scenario}`,
    },
    {
      label: "Inspect JSON report",
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
  scenarioFacts.innerHTML = `
    <p class="summary-text command-intro">Goal: ${escapeHtml(report.goal.target)} · ${escapeHtml(goalSummary(report))} · Sources: ${report.initial_context.sourceFragments.length}</p>
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
  transformationFlow.innerHTML = report.steps
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

const IDLE_COLOR = "#6f7b91";
const IDLE_GLOW = "rgba(111, 123, 145, 0.22)";
const RUNNING_COLOR = "#c88b00";
const RUNNING_GLOW = "rgba(200, 139, 0, 0.34)";
const COMPLETE_COLOR = "#1fa055";
const COMPLETE_GLOW = "rgba(31, 160, 85, 0.30)";

function stateFillColor(state) {
  if (state === "running") {
    return "#ffe7a3";
  }
  if (state === "complete") {
    return "#c8f4d6";
  }
  return "#e3e8ef";
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
  const centerX = 392;
  const centerY = 266;
  const goalY = 64;
  const mcpY = 162;
  const runtimeY = 266;
  const resultY = 520;

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
    { x: centerX - 272, y: centerY - 132 },
    { x: centerX - 272, y: centerY - 18 },
    { x: centerX - 272, y: centerY + 96 },
    { x: centerX - 272, y: centerY + 210 },
    { x: centerX - 110, y: centerY + 244 },
  ];
  const aiSlots = [
    { x: centerX + 232, y: centerY - 132 },
    { x: centerX + 232, y: centerY + 24 },
    { x: centerX + 232, y: centerY + 180 },
  ];

  for (const [index, nodeId] of serviceNodes.entries()) {
    const point = serviceSlots[index] ?? {
      x: centerX - 272,
      y: centerY - 132 + index * 108,
    };
    layout.set(nodeId, {
      x: point.x,
      y: point.y,
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
      y: point.y,
      role: "ai-capability",
    });
  }

  for (const node of report.graph_nodes.filter(
    (item) => item.kind === "capability" && !APP_HIDDEN_CAPABILITIES.has(item.id)
  )) {
    if (layout.has(node.id)) {
      continue;
    }
    const point = { x: centerX - 272, y: centerY };
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
  const selected = report.selected_path || [];
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
    nodes.push({
      id: nodeId,
      type: nodeId === "goal" ? "diamond" : nodeId === "result" ? "diamond" : "hexagon",
      data: { label },
      style: {
        x: point.x,
        y: point.y,
        size: nodeId === "goal" ? 84 : 74,
        fill: stateFillColor(active ? "running" : complete ? "complete" : "idle"),
        stroke: stateStrokeColor(active ? "running" : complete ? "complete" : "idle"),
        shadowBlur: active ? 28 : complete ? 18 : 8,
        shadowColor: active ? RUNNING_GLOW : complete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: nodeId === "goal" ? stackedNodeLabel("◎", label) : label,
        labelPlacement: "center",
        labelFill: active || complete ? "#151821" : "#151821",
        labelFontSize: 10,
        labelFontWeight: active || complete ? 700 : 500,
        labelLineHeight: 15,
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
    const isComplete = phase !== "idle" && (state === "complete" || plannerCompleted);
    const isAvailable = report.initial_context.availableCapabilities.includes(nodeId);
    const isAI = isAICapabilityNode(nodeId);
    nodes.push({
      id: nodeId,
      type: isAI ? "circle" : "rect",
      data: { label: displayName(nodeId, source.label) },
      style: {
        x: point.x,
        y: point.y,
        size: isAI ? 108 : [164, 64],
        radius: isAI ? undefined : 18,
        fill: stateFillColor(isCurrent ? "running" : isComplete ? "complete" : "idle"),
        stroke: stateStrokeColor(isCurrent ? "running" : isComplete ? "complete" : "idle"),
        lineWidth: isCurrent ? 3 : isComplete ? 2.5 : 1.5,
        shadowBlur: isCurrent ? 28 : isComplete ? 18 : 8,
        shadowColor: isCurrent ? RUNNING_GLOW : isComplete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: isAI
          ? stackedNodeLabel("✦", labelWithRoleIcon(nodeId, displayName(nodeId, source.label)))
          : labelWithRoleIcon(nodeId, displayName(nodeId, source.label)),
        labelPlacement: "center",
        labelFill: "#151821",
        labelFontSize: isAI ? 11 : 12,
        labelFontWeight: isCurrent ? 700 : 600,
        labelLineHeight: isAI ? 16 : undefined,
        opacity: isAvailable ? 1 : 0.72,
      },
    });
  }

  const resultPoint = layout.get("result");
  if (resultPoint) {
    const resultComplete = phase === "result" && focusedStepIndex >= report.steps.length;
    nodes.push({
      id: "result",
      type: "diamond",
      data: { label: "Result" },
      style: {
        x: resultPoint.x,
        y: resultPoint.y,
        size: 84,
        fill: stateFillColor(resultComplete ? "complete" : "idle"),
        stroke: stateStrokeColor(resultComplete ? "complete" : "idle"),
        shadowBlur: resultComplete ? 18 : 8,
        shadowColor: resultComplete ? COMPLETE_GLOW : IDLE_GLOW,
        labelText: stackedNodeLabel("✓", "Result"),
        labelPlacement: "center",
        labelFill: "#151821",
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
    : `<li><span>No runtime rejections in this scenario.</span></li>`;

  graphInspector.innerHTML = `
    <div class="inspector-block">
      <small>Current focus</small>
      <p class="summary-text"><strong>${escapeHtml(currentStep?.selected_capability ?? "Complete")}</strong>${currentStep ? ` · ${escapeHtml(currentStep.need)}` : ""}</p>
      <p class="summary-text"><strong>Phase:</strong> ${escapeHtml(phaseLabel(phase))}</p>
    </div>
    <div class="inspector-block">
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
  document.body.classList.remove("modal-open");
}

function stopPlayback() {
  if (playbackTimer) {
    window.clearTimeout(playbackTimer);
    playbackTimer = null;
  }
  isPlaying = false;
  currentPhase = "idle";
  runButton.textContent = "Play scenario";
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
      option.textContent = item.title;
      scenarioSelect.appendChild(option);
    }
    if (items[0]) {
      await renderScenario(items[0].id);
      runButton.textContent = "Play scenario";
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

if (commandsModal) {
  commandsModal.addEventListener("click", (event) => {
    if (event.target.hasAttribute("data-close-commands")) {
      closeCommandsModal();
    }
  });
}

window.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    closeCommandsModal();
  }
});

init();
