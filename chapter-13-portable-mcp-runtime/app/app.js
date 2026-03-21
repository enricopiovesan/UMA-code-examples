const scenarioSelect = document.getElementById("scenario-select");
const runButton = document.getElementById("run-scenario");
const timeline = document.getElementById("timeline");
const finalOutput = document.getElementById("final-output");
const graphScene = document.getElementById("graph-scene");
const summaryStrip = document.getElementById("summary-strip");
const graphInspector = document.getElementById("graph-inspector");
const openReport = document.getElementById("open-report");

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

function renderSummary(report) {
  const reportHref = `./fixtures/${report.scenario}.json`;
  if (openReport) {
    openReport.href = reportHref;
  }

  const rejectedSummary = report.rejected_capabilities.length
    ? report.rejected_capabilities
        .map((item) => `${escapeHtml(item.capability)} (${escapeHtml(item.reasons.join(", "))})`)
        .join(" · ")
    : "No runtime rejections in this scenario.";

  summaryStrip.innerHTML = `
    <div class="summary-intro">
      <p class="eyebrow">Scenario</p>
      <h2>${escapeHtml(report.title)}</h2>
      <p class="summary-text">${escapeHtml(report.summary)}</p>
      <p class="summary-meta">
        <span><strong>Scenario id:</strong> ${escapeHtml(report.scenario)}</span>
        <a href="${reportHref}" target="_blank" rel="noreferrer">Inspect JSON report</a>
      </p>
      <p class="summary-text"><strong>Goal framing:</strong> ${escapeHtml(goalSummary(report))}</p>
    </div>
    <div class="summary-grid">
      <div class="summary-card">
        <small>Status</small>
        <strong>${escapeHtml(report.status)}</strong>
      </div>
      <div class="summary-card">
        <small>Selected steps</small>
        <strong>${report.selected_path.length}</strong>
      </div>
      <div class="summary-card">
        <small>Rejected capabilities</small>
        <strong>${report.rejected_capabilities.length}</strong>
      </div>
      <div class="summary-card">
        <small>Output language</small>
        <strong>${escapeHtml(report.final_language)}</strong>
      </div>
    </div>
    <div class="summary-note">
      <small>Selected path</small>
      <div class="chip-row">
        ${report.selected_path.map((item) => `<span class="mini-chip active">${escapeHtml(item)}</span>`).join("")}
      </div>
    </div>
    <div class="summary-note ${report.rejected_capabilities.length ? "danger" : "ok"}">
      <small>Runtime verdict on proposals</small>
      <p class="summary-text">${rejectedSummary}</p>
    </div>
  `;
}

function renderTimeline(report) {
  timeline.innerHTML = report.steps
    .map((step) => {
      const discovery = step.discovery.available
        .map((item) => `<span class="mini-chip">${escapeHtml(item.capability)}</span>`)
        .join("");
      const rejected = step.discovery.rejected
        .map(
          (item) =>
            `<li><strong>${escapeHtml(item.capability)}</strong>: ${escapeHtml(item.reason ?? "none")}</li>`
        )
        .join("");
      const reasons = step.validation.reasons
        .map((reason) => `<li>${escapeHtml(reason)}</li>`)
        .join("");
      const events = step.events
        .map(
          (event) =>
            `<li><span>${escapeHtml(event.type)}</span><span>${escapeHtml(event.capability)}</span><span>${escapeHtml(event.status)}</span></li>`
        )
        .join("");
      const proposalStatus = step.proposed_validation
        ? `<p class="summary-text proposal-reject"><strong>Proposal rejected:</strong> ${escapeHtml(
            step.proposed_validation.capability
          )} - ${escapeHtml(step.proposed_validation.reasons.join(", "))}</p>`
        : "";

      return `
        <article class="timeline-card">
          <div class="timeline-meta">
            <span class="pill">Step ${step.index}</span>
            <span class="pill ${step.validation.status}">${escapeHtml(step.selected_capability)}</span>
            <span class="pill">${escapeHtml(step.need)}</span>
          </div>
          <h3>${escapeHtml(step.selected_capability)}</h3>
          <p class="summary-text"><strong>Agent proposal:</strong> ${escapeHtml(step.agent_proposal ?? "none")}</p>
          ${proposalStatus}
          <div class="timeline-section">
            <small>Discovery candidates</small>
            <div class="chip-row">${discovery || '<span class="mini-chip muted">none</span>'}</div>
          </div>
          ${
            rejected
              ? `<div class="timeline-section"><small>Rejected during discovery</small><ul class="reason-list">${rejected}</ul></div>`
              : ""
          }
          ${
            reasons
              ? `<div class="timeline-section"><small>Validation reasons</small><ul class="reason-list">${reasons}</ul></div>`
              : ""
          }
          <div class="timeline-section">
            <small>Output preview</small>
            <p class="summary-text output-preview">${escapeHtml(step.output_preview)}</p>
          </div>
          <div class="timeline-section">
            <small>Events emitted</small>
            <ul class="event-list grid-events">${events}</ul>
          </div>
        </article>
      `;
    })
    .join("");
}

function graphPoint(node) {
  return {
    x: node.x + 360,
    y: node.y + 240,
    z: node.z,
  };
}

function renderGraph(report) {
  graphScene.innerHTML = "";

  const nodeMap = new Map();
  for (const node of report.graph_nodes) {
    const point = graphPoint(node);
    nodeMap.set(node.id, point);
    const element = document.createElement("div");
    element.className = `graph-node ${node.kind} ${node.state}`;
    element.style.left = `${point.x}px`;
    element.style.top = `${point.y}px`;
    element.style.transform = `translate3d(0, 0, ${point.z}px) rotateY(-16deg) rotateX(10deg)`;
    element.innerHTML = `<small>${escapeHtml(node.kind)}</small><strong>${escapeHtml(node.label)}</strong>`;
    graphScene.appendChild(element);
  }

  for (const edge of report.graph_edges) {
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
    line.className = `graph-edge ${edge.state}`;
    line.style.left = `${from.x + 56}px`;
    line.style.top = `${from.y + 26}px`;
    line.style.width = `${distance}px`;
    line.style.transform = `rotate(${angle}deg)`;
    graphScene.appendChild(line);
  }
}

function renderGraphInspector(report) {
  const selected = report.selected_path
    .map((item, index) => `<li><span class="counter">${index + 1}</span><span>${escapeHtml(item)}</span></li>`)
    .join("");
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
      <small>Selected path</small>
      <ol class="path-list">${selected}</ol>
    </div>
    <div class="inspector-block">
      <small>Rejected capabilities</small>
      <ul class="inspector-list">${rejected}</ul>
    </div>
  `;
}

function renderOutput(report) {
  finalOutput.textContent = report.final_output;
}

async function renderScenario(id) {
  const report = await loadScenarioReport(id);
  renderSummary(report);
  renderTimeline(report);
  renderGraph(report);
  renderGraphInspector(report);
  renderOutput(report);
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
    }
  } catch (error) {
    summaryStrip.innerHTML = `<p class="summary-text">${escapeHtml(error.message)}</p>`;
  }
}

runButton.addEventListener("click", async () => {
  await renderScenario(scenarioSelect.value);
});

scenarioSelect.addEventListener("change", async () => {
  await renderScenario(scenarioSelect.value);
});

init();
