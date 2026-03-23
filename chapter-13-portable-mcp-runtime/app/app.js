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

function buildGraphData(report, focusedStepIndex) {
  const { nodeStates } = graphStatesForStep(report, focusedStepIndex);
  const currentStep = report.steps.find((step) => step.index === focusedStepIndex) || null;
  const layout = layoutGraph(report);
  const selected = report.selected_path || [];
  const nodes = [];
  const currentProposalCapability = currentStep?.proposed_validation?.capability ?? null;
  const currentAgentProposal = currentStep?.agent_proposal ?? null;

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
    const active = nodeId === "agent" ? currentStep?.agent_mode === "runtime-hosted-ranking" : nodeStates.get(nodeId) === "active";
    const complete = nodeId === "goal" || nodeId === "mcp" || nodeId === "runtime";
    nodes.push({
      id: nodeId,
      type: "circle",
      data: { label },
      style: {
        x: point.x,
        y: point.y,
        size: 64,
        fill: active ? "#7c5cff" : complete ? "#272039" : "#1a1d28",
        stroke: active || complete ? "#b6a8ff" : "rgba(181, 185, 204, 0.22)",
        shadowBlur: active ? 28 : complete ? 18 : 12,
        shadowColor: active || complete ? "rgba(124, 92, 255, 0.26)" : "rgba(17, 19, 26, 0.22)",
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
    const isCurrent = nodeId === currentStep?.selected_capability;
    nodes.push({
      id: nodeId,
      type: "rect",
      data: { label: source.label },
      style: {
        x: point.x,
        y: point.y,
        size: [136, 56],
        radius: 18,
        fill: isCurrent ? "#f5f3ff" : "#ffffff",
        stroke: state === "complete" ? "#7c5cff" : isCurrent ? "#7c5cff" : "rgba(181, 185, 204, 0.22)",
        shadowBlur: isCurrent ? 26 : 14,
        shadowColor: isCurrent ? "rgba(124, 92, 255, 0.26)" : "rgba(17, 19, 26, 0.10)",
        labelText: source.label,
        labelPlacement: "center",
        labelFill: "#12131a",
        labelFontSize: 13,
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
    nodes.push({
      id: node.id,
      type: "circle",
      data: { label: node.label },
      style: {
        x: point.x,
        y: point.y,
        size: 38,
        fill: "#171922",
        stroke: state === "rejected" ? "#ea6c6c" : "rgba(181, 185, 204, 0.18)",
        shadowBlur: 8,
        shadowColor: "rgba(17, 19, 26, 0.12)",
        labelText: node.label,
        labelPlacement: "bottom",
        labelOffsetY: 6,
        labelFill: state === "rejected" ? "#f28e8e" : "rgba(207, 210, 221, 0.52)",
        labelFontSize: 10,
        opacity: 0.58,
      },
    });
  }

  const resultPoint = layout.get("result");
  if (resultPoint) {
    nodes.push({
      id: "result",
      type: "circle",
      data: { label: "Result" },
      style: {
        x: resultPoint.x,
        y: resultPoint.y,
        size: 72,
        fill: focusedStepIndex >= report.steps.length ? "#7c5cff" : "#1a1d28",
        stroke: focusedStepIndex >= report.steps.length ? "#b6a8ff" : "rgba(181, 185, 204, 0.22)",
        shadowBlur: focusedStepIndex >= report.steps.length ? 26 : 12,
        shadowColor: focusedStepIndex >= report.steps.length ? "rgba(124, 92, 255, 0.30)" : "rgba(17, 19, 26, 0.18)",
        labelText: "Result",
        labelPlacement: "bottom",
        labelOffsetY: 10,
        labelFill: focusedStepIndex >= report.steps.length ? "#ffffff" : "rgba(229, 231, 241, 0.78)",
        labelFontSize: 12,
        labelFontWeight: 700,
      },
    });
  }

  const edges = [];
  const supportChain = ["goal", "mcp", "agent", "runtime"];
  for (let index = 0; index < supportChain.length - 1; index += 1) {
    const active = index === 2 && currentStep?.agent_mode === "runtime-hosted-ranking";
    edges.push({
      id: `support-${supportChain[index]}-${supportChain[index + 1]}`,
      source: supportChain[index],
      target: supportChain[index + 1],
      style: {
        stroke: active || index < 2 ? "rgba(124, 92, 255, 0.78)" : "rgba(124, 92, 255, 0.28)",
        lineWidth: active || index < 2 ? 3 : 2,
        shadowBlur: active ? 16 : 0,
        shadowColor: "rgba(124, 92, 255, 0.20)",
      },
    });
  }

  if (selected.length > 0) {
    edges.push({
      id: "workflow-runtime-start",
      source: "runtime",
      target: selected[0],
      style: {
        stroke: focusedStepIndex >= 1 ? "#7c5cff" : "rgba(124, 92, 255, 0.18)",
        lineWidth: focusedStepIndex >= 1 ? 4 : 2,
        shadowBlur: focusedStepIndex >= 1 ? 18 : 0,
        shadowColor: "rgba(124, 92, 255, 0.24)",
      },
    });
  }

  selected.forEach((nodeId, index) => {
    const next = selected[index + 1];
    if (!next) {
      return;
    }
    const currentEdgeStep = index + 2;
    const complete = currentEdgeStep < focusedStepIndex;
    const active = currentEdgeStep === focusedStepIndex;
    edges.push({
      id: `workflow-${nodeId}-${next}`,
      source: nodeId,
      target: next,
      style: {
        stroke: active ? "#7c5cff" : complete ? "rgba(124, 92, 255, 0.70)" : "rgba(124, 92, 255, 0.18)",
        lineWidth: active ? 4 : complete ? 3 : 2,
        shadowBlur: active ? 18 : 0,
        shadowColor: "rgba(124, 92, 255, 0.24)",
      },
    });
  });

  if (selected.length > 0) {
    edges.push({
      id: "workflow-result",
      source: selected[selected.length - 1],
      target: "result",
      style: {
        stroke: focusedStepIndex >= report.steps.length ? "#7c5cff" : "rgba(124, 92, 255, 0.18)",
        lineWidth: focusedStepIndex >= report.steps.length ? 4 : 2,
        shadowBlur: focusedStepIndex >= report.steps.length ? 18 : 0,
        shadowColor: "rgba(124, 92, 255, 0.24)",
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

function renderGraph(report, focusedStepIndex) {
  const instance = ensureGraph();
  const data = buildGraphData(report, focusedStepIndex);
  try {
    instance.setData(data);
    instance.render();
  } catch (error) {
    graphScene.innerHTML = `<div class="graph-error">${escapeHtml(error.message)}</div>`;
  }
}

function renderGraphInspector(report, focusedStepIndex) {
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
  renderStep(report, focusedStepIndex);
}

function renderStep(report, focusedStepIndex) {
  stepSelect.value = String(focusedStepIndex);
  renderTransformations(report, focusedStepIndex);
  renderGraph(report, focusedStepIndex);
  renderGraphInspector(report, focusedStepIndex);
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
  runButton.textContent = "Play scenario";
}

function scheduleNextStep(report, nextStepIndex) {
  playbackTimer = window.setTimeout(() => {
    renderStep(report, nextStepIndex);
    if (nextStepIndex >= report.steps.length) {
      stopPlayback();
      return;
    }
    scheduleNextStep(report, nextStepIndex + 1);
  }, 1100);
}

function startPlayback(report) {
  if (!report?.steps?.length) {
    return;
  }
  stopPlayback();
  isPlaying = true;
  runButton.textContent = "Stop playback";
  renderStep(report, report.steps[0].index);
  if (report.steps.length === 1) {
    stopPlayback();
    return;
  }
  scheduleNextStep(report, report.steps[1].index);
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
  renderStep(currentReport, Number(stepSelect.value));
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
