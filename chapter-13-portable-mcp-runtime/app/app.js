const scenarioSelect = document.getElementById("scenario-select");
const runButton = document.getElementById("run-scenario");
const timeline = document.getElementById("timeline");
const finalOutput = document.getElementById("final-output");
const graphScene = document.getElementById("graph-scene");
const summaryStrip = document.getElementById("summary-strip");

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

function renderSummary(report) {
  summaryStrip.innerHTML = `
    <div class="summary-grid">
      <div class="summary-card">
        <small>Goal</small>
        <strong>${report.title}</strong>
      </div>
      <div class="summary-card">
        <small>Status</small>
        <strong>${report.status}</strong>
      </div>
      <div class="summary-card">
        <small>Selected path</small>
        <strong>${report.selected_path.join(" -> ")}</strong>
      </div>
      <div class="summary-card">
        <small>Rejected capabilities</small>
        <strong>${report.rejected_capabilities.length}</strong>
      </div>
    </div>
  `;
}

function renderTimeline(report) {
  const cards = report.steps.map((step) => {
    const discovery = step.discovery.available.map((item) => item.capability).join(", ") || "none";
    const rejected = step.discovery.rejected
      .map((item) => `<li>${item.capability}: ${item.reason ?? "none"}</li>`)
      .join("");
    const reasons = step.validation.reasons
      .map((reason) => `<li>${reason}</li>`)
      .join("");
    const proposedValidation = step.proposed_validation
      ? `<p class="summary-text"><strong>Proposal rejected:</strong> ${step.proposed_validation.capability} - ${step.proposed_validation.reasons.join(", ")}</p>`
      : "";
    const events = step.events
      .map((event) => `<li>${event.type} - ${event.capability} (${event.status})</li>`)
      .join("");

    return `
      <article class="timeline-card">
        <div class="timeline-meta">
          <span class="pill">Step ${step.index}</span>
          <span class="pill ${step.validation.status}">${step.selected_capability}</span>
          <span class="pill">${step.need}</span>
        </div>
        <h3>${step.selected_capability}</h3>
        <p class="summary-text"><strong>Agent proposal:</strong> ${step.agent_proposal ?? "none"}</p>
        ${proposedValidation}
        <p class="summary-text"><strong>Discovery:</strong> ${discovery}</p>
        ${rejected ? `<ul class="reason-list">${rejected}</ul>` : ""}
        ${reasons ? `<ul class="reason-list">${reasons}</ul>` : ""}
        <p class="summary-text"><strong>Output:</strong> ${step.output_preview}</p>
        <ul class="event-list">${events}</ul>
      </article>
    `;
  });
  timeline.innerHTML = cards.join("");
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
    const el = document.createElement("div");
    el.className = `graph-node ${node.kind} ${node.state}`;
    el.style.left = `${point.x}px`;
    el.style.top = `${point.y}px`;
    el.style.transform = `translate3d(0, 0, ${point.z}px) rotateY(-16deg) rotateX(10deg)`;
    el.innerHTML = `<small>${node.kind}</small><strong>${node.label}</strong>`;
    graphScene.appendChild(el);
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

function renderOutput(report) {
  finalOutput.textContent = report.final_output;
}

async function renderScenario(id) {
  const report = await loadScenarioReport(id);
  renderSummary(report);
  renderTimeline(report);
  renderGraph(report);
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
    summaryStrip.innerHTML = `<p class="summary-text">${error.message}</p>`;
  }
}

runButton.addEventListener("click", async () => {
  await renderScenario(scenarioSelect.value);
});

scenarioSelect.addEventListener("change", async () => {
  await renderScenario(scenarioSelect.value);
});

init();
