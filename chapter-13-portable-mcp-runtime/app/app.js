import { Graph } from "@antv/g6";

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
const summaryStrip = document.getElementById("summary-strip");
const graphInspector = document.getElementById("graph-inspector");
const openReport = document.getElementById("open-report");
let currentReport = null;
let playbackTimer = null;
let isPlaying = false;
let graph = null;
let graphResizeObserver = null;
let currentPhase = "capability";

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

function renderSummary(report) {
  const reportHref = `./fixtures/${report.scenario}.json`;
  if (openReport) {
    openReport.href = reportHref;
  }

  summaryStrip.innerHTML = `
    <div class="summary-inline">
      <span>${escapeHtml(report.title)}</span>
      <span>${escapeHtml(goalSummary(report))}</span>
      <a href="${reportHref}" target="_blank" rel="noreferrer">JSON report</a>
    </div>
  `;
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

function renderTransformations(report, focusedStepIndex) {
  transformationFlow.innerHTML = report.steps
    .map((step) => {
      const isFocused = step.index === focusedStepIndex;
      const before = step.index === 1 ? formatInitialState(report) : report.steps[step.index - 2].output_preview;
      const rejected = step.discovery.rejected
        .filter(
          (item) =>
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

      return `
        <article class="transform-card ${isFocused ? "is-focused" : ""}">
          <div class="timeline-meta">
            <span class="pill">Step ${step.index}</span>
            <span class="pill ${step.validation.status}">${escapeHtml(step.selected_capability)}</span>
            <span class="pill">${escapeHtml(step.need)}</span>
          </div>
          <h3>${escapeHtml(step.selected_capability)}</h3>
          <p class="summary-text"><strong>Agent:</strong> ${escapeHtml(step.agent_provider)} · ${escapeHtml(step.agent_mode)}</p>
          <p class="summary-text"><strong>Agent proposal:</strong> ${escapeHtml(step.agent_proposal ?? "none")}</p>
          ${agentNote}
          ${proposalStatus}
          ${fallbackNote}
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
    let state = "inactive";

    if (node.id === "goal") {
      state = "complete";
    } else if (node.id === "mcp" || node.id === "runtime") {
      state = "active";
    } else if (node.id === "agent") {
      state = proposedCapability ? "active" : "candidate";
    } else if (node.id === "result") {
      state = focusedStepIndex >= report.steps.length ? "complete" : "inactive";
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

function isAIAgentNode(nodeId) {
  return nodeId === "agent" || nodeId === "PlannerAI";
}

function isAICapabilityNode(nodeId) {
  return nodeId === "SummarizerAI" || nodeId === "PlannerAI";
}

function layoutGraph(report) {
  const layout = new Map();
  const selected = report.selected_path || [];
  const hiddenNodes = new Set(["PlannerAI"]);

  const supportNodes = [
    ["goal", { x: 88, y: 78, role: "support-start" }],
    ["mcp", { x: 222, y: 78, role: "support" }],
    ["agent", { x: 382, y: 78, role: "support" }],
    ["runtime", { x: 548, y: 78, role: "support" }],
  ];

  for (const [id, point] of supportNodes) {
    layout.set(id, point);
  }

  const startX = 96;
  const gap = selected.length >= 5 ? 128 : 142;
  const flowY = 250;
  selected.forEach((id, index) => {
    layout.set(id, {
      x: startX + index * gap,
      y: flowY,
      role: "workflow",
    });
  });

  const resultX = startX + selected.length * gap + 132;
  layout.set("result", { x: resultX, y: flowY, role: "result" });

  const secondary = report.graph_nodes.filter(
    (node) =>
      node.kind === "capability" &&
      !selected.includes(node.id) &&
      !hiddenNodes.has(node.id)
  );

  const secondaryStart = 214;
  const secondaryGap = secondary.length > 1 ? 176 : 0;
  secondary.forEach((node, index) => {
    layout.set(node.id, {
      x: secondaryStart + index * secondaryGap,
      y: 358,
      role: "secondary",
    });
  });

  return layout;
}

function ensureGraph() {
  if (graph) {
    return graph;
  }

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
        endArrow: false,
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
  const supportPath = ["goal", "mcp", ...(usesPlanner(currentStep) ? ["agent"] : []), "runtime"];
  const activeSupportNode =
    phase === "mcp" ? "mcp" :
    phase === "planner" ? "agent" :
    phase === "runtime" ? "runtime" :
    null;
  const activeSupportIndex = activeSupportNode ? supportPath.indexOf(activeSupportNode) : -1;

  const supportNodeIds = ["goal", "mcp", "agent", "runtime"];
  for (const nodeId of supportNodeIds) {
    const source = report.graph_nodes.find((item) => item.id === nodeId);
    const point = layout.get(nodeId);
    if (!source || !point) {
      continue;
    }
    const label =
      nodeId === "mcp"
        ? "MCP"
        : nodeId === "agent"
          ? "Planner"
          : nodeId === "runtime"
            ? "Runtime"
            : source.label;
    const onPath = supportPath.includes(nodeId);
    const pathIndex = supportPath.indexOf(nodeId);
    const active = nodeId === activeSupportNode;
    const complete =
      nodeId === "goal" ||
      (phase === "capability" && onPath) ||
      (phase === "result" && onPath) ||
      (activeSupportIndex > -1 && onPath && pathIndex > -1 && pathIndex < activeSupportIndex);
    nodes.push({
      id: nodeId,
      type: "circle",
      data: { label },
      style: {
        x: point.x,
        y: point.y,
        size: 64,
        fill: nodeId === "agent"
          ? active ? "#f3b44b" : complete ? "#7c5f18" : "#241d10"
          : active ? "#8a6dff" : complete ? "#4734a7" : "#1a1d28",
        stroke: nodeId === "agent"
          ? active ? "#fff1c8" : complete ? "#f3d27c" : "rgba(243, 180, 75, 0.28)"
          : active ? "#f2ecff" : complete ? "#cdbfff" : onPath ? "rgba(181, 185, 204, 0.34)" : "rgba(181, 185, 204, 0.22)",
        shadowBlur: active ? 34 : complete ? 24 : 12,
        shadowColor: nodeId === "agent"
          ? active || complete ? "rgba(243, 180, 75, 0.34)" : "rgba(17, 19, 26, 0.22)"
          : active || complete ? "rgba(124, 92, 255, 0.42)" : "rgba(17, 19, 26, 0.22)",
        labelText: label,
        labelPlacement: "bottom",
        labelOffsetY: 8,
        labelFill: active || complete ? "#ffffff" : "rgba(229, 231, 241, 0.78)",
        labelFontSize: 11,
        labelFontWeight: active || complete ? 700 : 500,
      },
    });
  }

  for (const nodeId of selected) {
    const source = report.graph_nodes.find((item) => item.id === nodeId);
    const point = layout.get(nodeId);
    if (!source || !point) {
      continue;
    }
    const state = nodeStates.get(nodeId) || "inactive";
    const isCurrent = nodeId === currentStep?.selected_capability && phase === "capability";
    const isComplete = state === "complete";
    const isAI = isAICapabilityNode(nodeId);
    nodes.push({
      id: nodeId,
      type: "rect",
      data: { label: source.label },
      style: {
        x: point.x,
        y: point.y,
        size: isAI ? [148, 56] : [136, 56],
        radius: 18,
        fill: isAI
          ? isCurrent ? "#fff3d8" : isComplete ? "#f7e0a6" : "#2b2415"
          : isCurrent ? "#f5f3ff" : isComplete ? "#ede8ff" : phase === "result" && nodeId === selected[selected.length - 1] ? "#f5f3ff" : "#ffffff",
        stroke: isAI
          ? isCurrent ? "#f3b44b" : isComplete ? "#e2b24b" : "rgba(243, 180, 75, 0.30)"
          : isCurrent ? "#7c5cff" : isComplete ? "#9a82ff" : "rgba(181, 185, 204, 0.22)",
        lineWidth: isCurrent ? 3 : isComplete ? 2.5 : 1.5,
        shadowBlur: isCurrent ? 32 : isComplete ? 20 : 14,
        shadowColor: isAI
          ? isCurrent ? "rgba(243, 180, 75, 0.38)" : isComplete ? "rgba(243, 180, 75, 0.20)" : "rgba(17, 19, 26, 0.10)"
          : isCurrent ? "rgba(124, 92, 255, 0.36)" : isComplete ? "rgba(124, 92, 255, 0.20)" : "rgba(17, 19, 26, 0.10)",
        labelText: isAI ? `AI · ${source.label}` : source.label,
        labelPlacement: "center",
        labelFill: isAI ? "#241b07" : "#12131a",
        labelFontSize: isAI ? 12 : 13,
        labelFontWeight: isCurrent ? 700 : 600,
      },
    });
  }

  for (const node of report.graph_nodes) {
    const point = layout.get(node.id);
    if (!point || selected.includes(node.id) || ["goal", "mcp", "agent", "runtime", "result"].includes(node.id)) {
      continue;
    }
    const showNode = node.id === currentProposalCapability || node.id === currentAgentProposal;
    if (!showNode) {
      continue;
    }
    const state = nodeStates.get(node.id) || "inactive";
    const isAI = isAIAgentNode(node.id) || isAICapabilityNode(node.id);
    nodes.push({
      id: node.id,
      type: isAI ? "rect" : "circle",
      data: { label: node.label },
      style: {
        x: point.x,
        y: point.y,
        size: isAI ? [110, 38] : 38,
        radius: isAI ? 12 : undefined,
        fill: isAI ? "#241d10" : "#171922",
        stroke: state === "rejected" ? "#ea6c6c" : isAI ? "rgba(243, 180, 75, 0.36)" : "rgba(181, 185, 204, 0.18)",
        shadowBlur: 8,
        shadowColor: isAI ? "rgba(243, 180, 75, 0.14)" : "rgba(17, 19, 26, 0.12)",
        labelText: isAI ? `AI · ${node.label}` : node.label,
        labelPlacement: isAI ? "center" : "bottom",
        labelOffsetY: isAI ? 0 : 6,
        labelFill: state === "rejected" ? "#f28e8e" : isAI ? "rgba(255, 223, 162, 0.72)" : "rgba(207, 210, 221, 0.52)",
        labelFontSize: 10,
        opacity: 0.58,
      },
    });
  }

  const resultPoint = layout.get("result");
  if (resultPoint) {
    const resultActive = phase === "result";
    nodes.push({
      id: "result",
      type: "circle",
      data: { label: "Result" },
      style: {
        x: resultPoint.x,
        y: resultPoint.y,
        size: 72,
        fill: resultActive ? "#8a6dff" : "#1a1d28",
        stroke: resultActive ? "#f2ecff" : "rgba(181, 185, 204, 0.22)",
        shadowBlur: resultActive ? 34 : 12,
        shadowColor: resultActive ? "rgba(124, 92, 255, 0.40)" : "rgba(17, 19, 26, 0.18)",
        labelText: "Result",
        labelPlacement: "bottom",
        labelOffsetY: 10,
        labelFill: resultActive ? "#ffffff" : "rgba(229, 231, 241, 0.78)",
        labelFontSize: 12,
        labelFontWeight: 700,
      },
    });
  }

  const edges = [];
  for (const capabilityId of selected) {
    edges.push({
      id: `runtime-capability-${capabilityId}`,
      source: "runtime",
      target: capabilityId,
      style: {
        stroke: "rgba(255, 255, 255, 0.08)",
        lineWidth: 1.5,
        lineDash: [4, 6],
      },
    });
  }

  if (currentProposalCapability && layout.has(currentProposalCapability)) {
    edges.push({
      id: `runtime-capability-proposed-${currentProposalCapability}`,
      source: "runtime",
      target: currentProposalCapability,
      style: {
        stroke: "rgba(255, 255, 255, 0.08)",
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
        stroke: "rgba(255, 255, 255, 0.08)",
        lineWidth: 1.5,
        lineDash: [4, 6],
      },
    });
  }

  for (let index = 0; index < supportPath.length - 1; index += 1) {
    const source = supportPath[index];
    const target = supportPath[index + 1];
    const sourceIndex = supportPath.indexOf(source);
    const targetIndex = supportPath.indexOf(target);
    const active = activeSupportIndex > -1 && sourceIndex < activeSupportIndex && targetIndex === activeSupportIndex;
    const complete = phase === "capability" || phase === "result" || (activeSupportIndex > -1 && targetIndex < activeSupportIndex);
    edges.push({
      id: `support-${source}-${target}`,
      source,
      target,
      style: {
        stroke: active ? "#d4c6ff" : complete ? "#9f8cff" : "rgba(124, 92, 255, 0.24)",
        lineWidth: active ? 6 : complete ? 4 : 2,
        shadowBlur: active ? 18 : complete ? 10 : 0,
        shadowColor: active ? "rgba(124, 92, 255, 0.38)" : "rgba(124, 92, 255, 0.18)",
      },
    });
  }

  if (selected.length > 0) {
    edges.push({
      id: "workflow-runtime-start",
      source: "runtime",
      target: selected[0],
      style: {
        stroke: phase === "capability" || phase === "result" ? "#d4c6ff" : "rgba(124, 92, 255, 0.14)",
        lineWidth: phase === "capability" || phase === "result" ? 6 : 2,
        shadowBlur: phase === "capability" || phase === "result" ? 24 : 0,
        shadowColor: "rgba(124, 92, 255, 0.34)",
      },
    });
  }

  selected.forEach((nodeId, index) => {
    const next = selected[index + 1];
    if (!next) {
      return;
    }
    const currentEdgeStep = index + 2;
    const complete = currentEdgeStep < focusedStepIndex || (currentEdgeStep === focusedStepIndex && (phase === "capability" || phase === "result"));
    const active = currentEdgeStep === focusedStepIndex && phase === "capability";
    edges.push({
      id: `workflow-${nodeId}-${next}`,
      source: nodeId,
      target: next,
      style: {
        stroke: active ? "#d4c6ff" : complete ? "#7c5cff" : "rgba(124, 92, 255, 0.14)",
        lineWidth: active ? 6 : complete ? 4 : 2,
        shadowBlur: active ? 24 : complete ? 10 : 0,
        shadowColor: active ? "rgba(124, 92, 255, 0.34)" : "rgba(124, 92, 255, 0.18)",
      },
    });
  });

  if (selected.length > 0) {
    edges.push({
      id: "workflow-result",
      source: selected[selected.length - 1],
      target: "result",
      style: {
        stroke: phase === "result" ? "#d4c6ff" : "rgba(124, 92, 255, 0.14)",
        lineWidth: phase === "result" ? 6 : 2,
        shadowBlur: phase === "result" ? 24 : 0,
        shadowColor: "rgba(124, 92, 255, 0.34)",
      },
    });
  }

  if (currentStep?.proposed_validation?.capability && layout.has(currentStep.proposed_validation.capability)) {
    edges.push({
      id: `rejected-${currentStep.proposed_validation.capability}`,
      source: "runtime",
      target: currentStep.proposed_validation.capability,
      style: {
        stroke: "#ea6c6c",
        lineWidth: 2,
        lineDash: [6, 4],
        shadowBlur: 10,
        shadowColor: "rgba(234, 108, 108, 0.16)",
      },
    });
  }

  return { nodes, edges };
}

function renderGraph(report, focusedStepIndex, phase = currentPhase) {
  const instance = ensureGraph();
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
  const rejected = report.rejected_capabilities.length
    ? report.rejected_capabilities
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
  renderSummary(report);
  renderCommands(report);
  stepSelect.innerHTML = report.steps
    .map((step) => `<option value="${step.index}">Step ${step.index}: ${escapeHtml(step.selected_capability)}</option>`)
    .join("");
  const focusedStepIndex = report.steps[0]?.index ?? 1;
  stepSelect.value = String(focusedStepIndex);
  renderStep(report, focusedStepIndex, "capability");
}

function renderStep(report, focusedStepIndex, phase = "capability") {
  currentPhase = phase;
  stepSelect.value = String(focusedStepIndex);
  renderTransformations(report, focusedStepIndex);
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
  currentPhase = "capability";
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
    summaryStrip.innerHTML = `<p class="summary-text">${escapeHtml(error.message)}</p>`;
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
  renderStep(currentReport, Number(stepSelect.value), "capability");
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
