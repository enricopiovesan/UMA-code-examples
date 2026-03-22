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
          <p class="summary-text"><strong>Agent proposal:</strong> ${escapeHtml(step.agent_proposal ?? "none")}</p>
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

  return { nodeStates, edgeStates };
}

function graphPoint(node) {
  return {
    x: node.x + 360,
    y: node.y + 240,
    z: node.z,
  };
}

function renderGraph(report, focusedStepIndex) {
  graphScene.innerHTML = "";
  const { nodeStates, edgeStates } = graphStatesForStep(report, focusedStepIndex);
  const currentStep = report.steps.find((step) => step.index === focusedStepIndex) || null;
  const activeCapability = currentStep?.selected_capability ?? null;

  const nodeMap = new Map();
  for (const node of report.graph_nodes) {
    const point = graphPoint(node);
    nodeMap.set(node.id, point);
    const element = document.createElement("div");
    const classes = ["graph-node", node.kind, nodeStates.get(node.id) || node.state];
    if (node.id === activeCapability) {
      classes.push("is-current");
    }
    if (currentStep?.proposed_validation?.capability === node.id) {
      classes.push("is-proposed-rejected");
    }
    element.className = classes.join(" ");
    element.style.left = `${point.x}px`;
    element.style.top = `${point.y}px`;
    element.style.transform = `translate3d(0, 0, ${point.z}px) rotateY(-16deg) rotateX(10deg)`;
    element.innerHTML = `<small>${escapeHtml(node.kind)}</small><strong>${escapeHtml(node.label)}</strong>`;
    graphScene.appendChild(element);
  }

  for (const edge of edgeStates) {
    const from = nodeMap.get(edge.from);
    const to = nodeMap.get(edge.to);
    if (!from || !to) {
      continue;
    }
    const dx = to.x - from.x;
    const dy = to.y - from.y;
    const distance = Math.sqrt(dx * dx + dy * dy);
    const angle = (Math.atan2(dy, dx) * 180) / Math.PI;

    const line = document.createElement("div");
    const classes = ["graph-edge", edge.state];
    if (
      (edge.from === "runtime" && edge.to === activeCapability) ||
      (edge.from === "goal" && edge.to === "mcp") ||
      (edge.from === "mcp" && edge.to === "agent") ||
      (edge.from === "agent" && edge.to === "runtime")
    ) {
      classes.push("is-current");
    }
    line.className = classes.join(" ");
    line.style.left = `${from.x + 56}px`;
    line.style.top = `${from.y + 26}px`;
    line.style.width = `${distance}px`;
    line.style.transform = `rotate(${angle}deg)`;
    graphScene.appendChild(line);
  }

  if (activeCapability && nodeMap.has(activeCapability)) {
    const point = nodeMap.get(activeCapability);
    const offsetX = Math.max(Math.min(420 - point.x, 110), -110);
    const offsetY = Math.max(Math.min(250 - point.y, 90), -90);
    graphScene.style.transform = `translate3d(${offsetX}px, ${offsetY}px, 0)`;
  } else {
    graphScene.style.transform = "translate3d(0, 0, 0)";
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
