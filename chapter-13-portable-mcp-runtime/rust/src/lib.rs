use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use wasm_bindgen::prelude::*;

pub mod mcp;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CapabilityContract {
    pub name: String,
    pub version: String,
    pub intent: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub constraints: ContractConstraints,
    #[serde(rename = "emitsEvents")]
    pub emits_events: Vec<String>,
    pub metadata: ContractMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractConstraints {
    pub runtime: Vec<String>,
    pub requires: Vec<String>,
    pub excludes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractMetadata {
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoalSpec {
    pub target: String,
    #[serde(rename = "targetLanguage")]
    pub target_language: String,
    #[serde(rename = "preferAI")]
    pub prefer_ai: bool,
    #[serde(rename = "allowDegraded")]
    pub allow_degraded: bool,
    #[serde(rename = "localOnly")]
    pub local_only: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextSeed {
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "sourceFragments")]
    pub source_fragments: Vec<String>,
    #[serde(rename = "availableCapabilities")]
    pub available_capabilities: Vec<String>,
    #[serde(rename = "aiAvailable")]
    pub ai_available: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub goal: GoalSpec,
    pub context: ContextSeed,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub capability: String,
    pub status: String,
    pub reason: Option<String>,
    #[serde(rename = "contextSnapshot")]
    pub context_snapshot: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryCandidate {
    pub capability: String,
    pub intent: String,
    pub reason: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoverySnapshot {
    pub need: String,
    pub available: Vec<DiscoveryCandidate>,
    pub rejected: Vec<DiscoveryCandidate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub capability: String,
    pub status: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionStep {
    pub index: usize,
    pub need: String,
    pub agent_provider: String,
    pub agent_proposal: Option<String>,
    pub proposed_validation: Option<ValidationResult>,
    pub selected_capability: String,
    pub discovery: DiscoverySnapshot,
    pub validation: ValidationResult,
    pub output_preview: String,
    pub events: Vec<RuntimeEvent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeState {
    pub source_loaded: bool,
    pub source_fragments: Vec<String>,
    pub structured_facts: Vec<String>,
    pub summary: Option<String>,
    pub translated_summary: Option<String>,
    pub report: Option<String>,
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub state: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionReport {
    pub scenario: String,
    pub title: String,
    pub summary: String,
    pub goal: GoalSpec,
    pub initial_context: ContextSeed,
    pub selected_path: Vec<String>,
    pub rejected_capabilities: Vec<ValidationResult>,
    pub steps: Vec<ExecutionStep>,
    pub final_output: String,
    pub final_language: String,
    pub status: String,
    pub graph_nodes: Vec<GraphNode>,
    pub graph_edges: Vec<GraphEdge>,
}

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("chapter root")
        .to_path_buf()
}

pub fn list_scenarios(root: &Path) -> Result<Vec<Scenario>, String> {
    let mut scenarios = Vec::new();
    for entry in fs::read_dir(root.join("examples")).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if !entry.file_type().map_err(|err| err.to_string())?.is_dir() {
            continue;
        }
        let path = entry.path().join("scenario.json");
        if path.exists() {
            let raw = fs::read_to_string(&path).map_err(|err| err.to_string())?;
            let scenario = serde_json::from_str::<Scenario>(&raw).map_err(|err| err.to_string())?;
            scenarios.push(scenario);
        }
    }
    scenarios.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(scenarios)
}

pub fn load_scenario(root: &Path, id: &str) -> Result<Scenario, String> {
    for scenario in list_scenarios(root)? {
        if scenario.id == id {
            return Ok(scenario);
        }
    }
    Err(format!("unknown scenario \"{id}\""))
}

fn contract_fixtures() -> Vec<CapabilityContract> {
    [
        include_str!("../../contracts/data-provider-local.json"),
        include_str!("../../contracts/insight-enricher.json"),
        include_str!("../../contracts/summarizer-basic.json"),
        include_str!("../../contracts/summarizer-ai.json"),
        include_str!("../../contracts/translator-fr.json"),
        include_str!("../../contracts/formatter.json"),
    ]
    .into_iter()
    .map(|raw| serde_json::from_str::<CapabilityContract>(raw).expect("valid contract fixture"))
    .collect()
}

pub fn capability_descriptors() -> Vec<CapabilityContract> {
    contract_fixtures()
}

pub fn available_capabilities_for_scenario(root: &Path, id: &str) -> Result<Vec<CapabilityContract>, String> {
    let scenario = load_scenario(root, id)?;
    Ok(contract_fixtures()
        .into_iter()
        .filter(|contract| {
            scenario
                .context
                .available_capabilities
                .iter()
                .any(|item| item == &contract.name)
        })
        .collect())
}

fn current_need(goal: &GoalSpec, state: &RuntimeState) -> &'static str {
    if !state.source_loaded {
        "provide-source-fragments"
    } else if state.structured_facts.is_empty() {
        "derive-structured-insights"
    } else if state.summary.is_none() {
        "generate-summary"
    } else if goal.target_language == "fr" && state.translated_summary.is_none() && !state.degraded {
        "translate-to-target-language"
    } else if state.report.is_none() {
        "format-structured-report"
    } else {
        "goal-satisfied"
    }
}

fn has_input(input: &str, state: &RuntimeState) -> bool {
    match input {
        "source_fragments" => state.source_loaded && !state.source_fragments.is_empty(),
        "structured_facts" => !state.structured_facts.is_empty(),
        "summary" => state.summary.is_some(),
        "translated_summary" => state.translated_summary.is_some(),
        "summary_or_translation" => state.translated_summary.is_some() || state.summary.is_some(),
        _ => false,
    }
}

fn relevant_for_need(contract: &CapabilityContract, need: &str) -> bool {
    contract.intent == need
}

#[derive(Debug, Clone, Copy)]
enum AgentProviderKind {
    DeterministicLocal,
}

impl AgentProviderKind {
    fn label(self) -> &'static str {
        match self {
            AgentProviderKind::DeterministicLocal => "deterministic-local-planner",
        }
    }
}

#[derive(Debug, Clone)]
struct AgentDecision {
    provider: AgentProviderKind,
    proposal: Option<String>,
}

fn agent_provider_for(_scenario: &Scenario) -> AgentProviderKind {
    AgentProviderKind::DeterministicLocal
}

fn propose_with_agent(
    provider: AgentProviderKind,
    need: &str,
    scenario: &Scenario,
    visible: &[CapabilityContract],
) -> AgentDecision {
    let proposal = match provider {
        AgentProviderKind::DeterministicLocal => deterministic_agent_proposal(need, scenario, visible),
    };

    AgentDecision { provider, proposal }
}

fn deterministic_agent_proposal(
    need: &str,
    scenario: &Scenario,
    visible: &[CapabilityContract],
) -> Option<String> {
    if need == "generate-summary" && scenario.goal.prefer_ai {
        if visible.iter().any(|c| c.name == "SummarizerAI") {
            return Some("SummarizerAI".to_string());
        }
    }

    visible.first().map(|item| item.name.clone())
}

fn validate_contract(
    contract: &CapabilityContract,
    scenario: &Scenario,
    state: &RuntimeState,
) -> ValidationResult {
    let mut reasons = Vec::new();

    for input in &contract.inputs {
        if !has_input(input, state) {
            reasons.push(format!("missing input: {input}"));
        }
    }

    if contract.constraints.requires.iter().any(|item| item == "ai") && !scenario.context.ai_available {
        reasons.push("AI capability is unavailable in the current context".to_string());
    }

    if scenario.goal.local_only && contract.constraints.excludes.iter().any(|item| item == "local-only") {
        reasons.push("capability violates local-only execution constraints".to_string());
    }

    if contract.constraints.requires.iter().any(|item| item == "local")
        && !contract.constraints.runtime.iter().any(|item| item.contains("browser") || item.contains("edge"))
    {
        reasons.push("capability cannot satisfy local execution placement".to_string());
    }

    if contract.name == "TranslatorFr" && !scenario.context.available_capabilities.iter().any(|item| item == "TranslatorFr") {
        reasons.push("translator is unavailable".to_string());
    }

    if reasons.is_empty() {
        ValidationResult {
            capability: contract.name.clone(),
            status: "accepted".to_string(),
            reasons,
        }
    } else {
        ValidationResult {
            capability: contract.name.clone(),
            status: "rejected".to_string(),
            reasons,
        }
    }
}

fn discover_candidates(
    scenario: &Scenario,
    _state: &RuntimeState,
    need: &str,
) -> (Vec<CapabilityContract>, DiscoverySnapshot) {
    let mut visible = Vec::new();
    let mut rejected = Vec::new();
    let mut available = Vec::new();

    for contract in contract_fixtures() {
        if !scenario
            .context
            .available_capabilities
            .iter()
            .any(|item| item == &contract.name)
        {
            rejected.push(DiscoveryCandidate {
                capability: contract.name,
                intent: contract.intent,
                reason: Some("capability not available in this scenario".to_string()),
                status: "hidden".to_string(),
            });
            continue;
        }

        if !relevant_for_need(&contract, need) {
            rejected.push(DiscoveryCandidate {
                capability: contract.name,
                intent: contract.intent,
                reason: Some("does not contribute to the current unmet need".to_string()),
                status: "not-relevant".to_string(),
            });
            continue;
        }

        available.push(DiscoveryCandidate {
            capability: contract.name.clone(),
            intent: contract.intent.clone(),
            reason: None,
            status: "candidate".to_string(),
        });
        visible.push(contract);
    }

    (
        visible,
        DiscoverySnapshot {
            need: need.to_string(),
            available,
            rejected,
        },
    )
}

fn context_snapshot(state: &RuntimeState) -> BTreeMap<String, String> {
    let mut snapshot = BTreeMap::new();
    snapshot.insert("sourceLoaded".to_string(), state.source_loaded.to_string());
    snapshot.insert("structuredFacts".to_string(), state.structured_facts.join(" | "));
    snapshot.insert(
        "summary".to_string(),
        state.summary.clone().unwrap_or_else(|| "none".to_string()),
    );
    snapshot.insert(
        "translatedSummary".to_string(),
        state.translated_summary.clone().unwrap_or_else(|| "none".to_string()),
    );
    snapshot.insert(
        "report".to_string(),
        state.report.clone().unwrap_or_else(|| "none".to_string()),
    );
    snapshot
}

fn emit_event(event_type: &str, capability: &str, status: &str, reason: Option<String>, state: &RuntimeState) -> RuntimeEvent {
    RuntimeEvent {
        event_type: event_type.to_string(),
        capability: capability.to_string(),
        status: status.to_string(),
        reason,
        context_snapshot: context_snapshot(state),
    }
}

fn enrich_facts(fragments: &[String]) -> Vec<String> {
    let mut facts = Vec::new();
    for fragment in fragments {
        let normalized = fragment.trim_end_matches('.');
        facts.push(format!("Fact: {normalized}."));
    }
    facts
}

fn summarize_basic(project: &str, facts: &[String]) -> String {
    format!(
        "{project} combines distributed browser, edge, and cloud evidence into a deterministic operational summary with {} validated insight(s).",
        facts.len()
    )
}

fn summarize_ai(project: &str, facts: &[String]) -> String {
    format!(
        "{project} shows how adaptive summarization can combine distributed sources into a richer narrative while still depending on runtime validation across {} insight(s).",
        facts.len()
    )
}

fn translate_french(summary: &str) -> String {
    let translated = summary
        .replace("combines distributed browser, edge, and cloud evidence into a deterministic operational summary with", "combine des preuves distribuees du navigateur, de l'edge et du cloud dans un resume operationnel deterministe avec")
        .replace("validated insight(s).", "observation(s) validee(s).")
        .replace("shows how adaptive summarization can combine distributed sources into a richer narrative while still depending on runtime validation across", "montre comment une synthese adaptative peut combiner des sources distribuees dans un recit plus riche tout en restant soumise a la validation du runtime sur");
    format!("Rapport en francais: {translated}")
}

fn format_report_output(project: &str, state: &RuntimeState, language: &str) -> String {
    let narrative = state
        .translated_summary
        .clone()
        .or_else(|| state.summary.clone())
        .unwrap_or_else(|| "No summary available".to_string());
    let insights = if state.structured_facts.is_empty() {
        "No structured facts".to_string()
    } else {
        state.structured_facts
            .iter()
            .take(2)
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    };
    format!(
        "Project: {project}\nLanguage: {language}\nSummary: {narrative}\nHighlights: {insights}"
    )
}

pub fn run_scenario(root: &Path, id: &str) -> Result<ExecutionReport, String> {
    let scenario = load_scenario(root, id)?;
    let mut state = RuntimeState {
        source_loaded: false,
        source_fragments: scenario.context.source_fragments.clone(),
        structured_facts: Vec::new(),
        summary: None,
        translated_summary: None,
        report: None,
        degraded: false,
    };

    let mut steps = Vec::new();
    let mut selected_path = Vec::new();
    let mut rejected_capabilities = Vec::new();

    for index in 1..=8 {
        let need = current_need(&scenario.goal, &state);
        if need == "goal-satisfied" {
            break;
        }

        if need == "translate-to-target-language"
            && !scenario.context.available_capabilities.iter().any(|item| item == "TranslatorFr")
        {
            if scenario.goal.allow_degraded {
                state.degraded = true;
            } else {
                return Err("translator required but unavailable".to_string());
            }
        }

        let need = current_need(&scenario.goal, &state);
        if need == "goal-satisfied" {
            break;
        }

        let (visible, discovery) = discover_candidates(&scenario, &state, need);
        let agent_decision = propose_with_agent(agent_provider_for(&scenario), need, &scenario, &visible);
        let proposal = agent_decision.proposal.clone();

        let mut selected_contract = None;
        let mut validation = None;
        let mut proposed_validation = None;

        if let Some(proposed_name) = proposal.clone() {
            if let Some(contract) = visible.iter().find(|item| item.name == proposed_name) {
                let result = validate_contract(contract, &scenario, &state);
                if result.status == "accepted" {
                    selected_contract = Some(contract.clone());
                    validation = Some(result);
                } else {
                    proposed_validation = Some(result.clone());
                    rejected_capabilities.push(result.clone());
                    validation = Some(result);
                }
            }
        }

        if selected_contract.is_none() {
            for contract in &visible {
                let result = validate_contract(contract, &scenario, &state);
                if result.status == "accepted" {
                    selected_contract = Some(contract.clone());
                    validation = Some(result);
                    break;
                }
                rejected_capabilities.push(result);
            }
        }

        let contract = selected_contract.ok_or_else(|| {
            if scenario.goal.allow_degraded {
                state.degraded = true;
                "no compatible capability satisfied the current need; degraded path required".to_string()
            } else {
                "no compatible capability satisfied the current need".to_string()
            }
        })?;
        let validation = validation.expect("validation exists");

        let mut events = Vec::new();
        for candidate in &discovery.available {
            events.push(emit_event(
                "CapabilityDiscovered",
                &candidate.capability,
                candidate.status.as_str(),
                None,
                &state,
            ));
        }
        for candidate in &discovery.rejected {
            events.push(emit_event(
                "CapabilityRejected",
                &candidate.capability,
                candidate.status.as_str(),
                candidate.reason.clone(),
                &state,
            ));
        }
        if let Some(ref proposed) = proposal {
            events.push(emit_event(
                "CapabilityProposed",
                proposed,
                "proposed",
                None,
                &state,
            ));
        }
        if let Some(ref rejected) = proposed_validation {
            events.push(emit_event(
                "CapabilityRejected",
                &rejected.capability,
                "rejected",
                Some(rejected.reasons.join(" | ")),
                &state,
            ));
        }
        events.push(emit_event(
            "CapabilitySelected",
            &contract.name,
            "selected",
            None,
            &state,
        ));
        events.push(emit_event(
            "CapabilityValidated",
            &contract.name,
            "accepted",
            None,
            &state,
        ));

        let output_preview = match contract.name.as_str() {
            "DataProviderLocal" => {
                state.source_loaded = true;
                events.push(emit_event(
                    "CapabilityExecuted",
                    &contract.name,
                    "success",
                    None,
                    &state,
                ));
                format!("Loaded {} distributed source fragment(s)", state.source_fragments.len())
            }
            "InsightEnricher" => {
                state.structured_facts = enrich_facts(&state.source_fragments);
                events.push(emit_event(
                    "ContextUpdated",
                    &contract.name,
                    "success",
                    None,
                    &state,
                ));
                format!("Derived {} structured fact(s)", state.structured_facts.len())
            }
            "SummarizerBasic" => {
                state.summary = Some(summarize_basic(&scenario.context.project_name, &state.structured_facts));
                events.push(emit_event(
                    "CapabilityExecuted",
                    &contract.name,
                    "success",
                    None,
                    &state,
                ));
                state.summary.clone().unwrap()
            }
            "SummarizerAI" => {
                state.summary = Some(summarize_ai(&scenario.context.project_name, &state.structured_facts));
                events.push(emit_event(
                    "CapabilityExecuted",
                    &contract.name,
                    "success",
                    None,
                    &state,
                ));
                state.summary.clone().unwrap()
            }
            "TranslatorFr" => {
                let translated = translate_french(state.summary.as_deref().unwrap_or_default());
                state.translated_summary = Some(translated.clone());
                events.push(emit_event(
                    "CapabilityExecuted",
                    &contract.name,
                    "success",
                    None,
                    &state,
                ));
                translated
            }
            "Formatter" => {
                let language = if state.translated_summary.is_some() {
                    "fr"
                } else {
                    scenario.goal.target_language.as_str()
                };
                let report = format_report_output(&scenario.context.project_name, &state, language);
                state.report = Some(report.clone());
                events.push(emit_event(
                    "GoalSatisfied",
                    &contract.name,
                    "success",
                    if state.degraded {
                        Some("runtime satisfied the goal in degraded mode".to_string())
                    } else {
                        None
                    },
                    &state,
                ));
                report
            }
            _ => "No output".to_string(),
        };

        selected_path.push(contract.name.clone());
        steps.push(ExecutionStep {
            index,
            need: need.to_string(),
            agent_provider: agent_decision.provider.label().to_string(),
            agent_proposal: proposal,
            proposed_validation,
            selected_capability: contract.name,
            discovery,
            validation,
            output_preview,
            events,
        });
    }

    let final_output = state
        .report
        .clone()
        .ok_or_else(|| "scenario ended without a final report".to_string())?;
    let final_language = if state.translated_summary.is_some() {
        "fr".to_string()
    } else if state.degraded {
        "en (degraded)".to_string()
    } else {
        scenario.goal.target_language.clone()
    };

    let mut completed = BTreeSet::new();
    for step in &steps {
        completed.insert(step.selected_capability.clone());
    }
    let mut rejected = BTreeSet::new();
    for item in &rejected_capabilities {
        rejected.insert(item.capability.clone());
    }

    let mut graph_nodes = vec![
        GraphNode { id: "goal".into(), label: "Goal".into(), kind: "goal".into(), state: "complete".into(), x: -260.0, y: -150.0, z: -80.0 },
        GraphNode { id: "mcp".into(), label: "MCP node".into(), kind: "mcp".into(), state: "active".into(), x: -140.0, y: -20.0, z: 60.0 },
        GraphNode { id: "agent".into(), label: "Agent".into(), kind: "agent".into(), state: if scenario.goal.prefer_ai { "active".into() } else { "candidate".into() }, x: 0.0, y: -170.0, z: 40.0 },
        GraphNode { id: "runtime".into(), label: "UMA runtime".into(), kind: "runtime".into(), state: "active".into(), x: 10.0, y: -10.0, z: 0.0 },
        GraphNode { id: "result".into(), label: "Result".into(), kind: "result".into(), state: "complete".into(), x: 290.0, y: 130.0, z: 40.0 },
    ];

    let capability_positions = [
        ("DataProviderLocal", -210.0, 110.0, -10.0),
        ("InsightEnricher", -60.0, 140.0, 30.0),
        ("SummarizerBasic", 60.0, 90.0, 80.0),
        ("SummarizerAI", 120.0, -80.0, 120.0),
        ("TranslatorFr", 210.0, 20.0, 20.0),
        ("Formatter", 160.0, 180.0, -30.0),
    ];

    for contract in contract_fixtures() {
        let state_value = if completed.contains(&contract.name) {
            "complete"
        } else if rejected.contains(&contract.name) {
            "rejected"
        } else if scenario.context.available_capabilities.iter().any(|item| item == &contract.name) {
            "candidate"
        } else {
            "inactive"
        };
        let (x, y, z) = capability_positions
            .iter()
            .find(|(name, _, _, _)| *name == contract.name)
            .map(|(_, x, y, z)| (*x, *y, *z))
            .unwrap_or((0.0, 0.0, 0.0));
        graph_nodes.push(GraphNode {
            id: contract.name.clone(),
            label: contract.name,
            kind: "capability".into(),
            state: state_value.into(),
            x,
            y,
            z,
        });
    }

    let mut graph_edges = vec![
        GraphEdge { from: "goal".into(), to: "mcp".into(), state: "active".into() },
        GraphEdge { from: "mcp".into(), to: "agent".into(), state: "active".into() },
        GraphEdge { from: "agent".into(), to: "runtime".into(), state: "active".into() },
    ];
    for capability in &selected_path {
        graph_edges.push(GraphEdge {
            from: "runtime".into(),
            to: capability.clone(),
            state: "complete".into(),
        });
    }
    if let Some(last) = selected_path.last() {
        graph_edges.push(GraphEdge {
            from: last.clone(),
            to: "result".into(),
            state: "complete".into(),
        });
    }

    Ok(ExecutionReport {
        scenario: scenario.id,
        title: scenario.title,
        summary: scenario.summary,
        goal: scenario.goal,
        initial_context: scenario.context,
        selected_path,
        rejected_capabilities,
        steps,
        final_output,
        final_language,
        status: if state.degraded { "degraded".into() } else { "complete".into() },
        graph_nodes,
        graph_edges,
    })
}

pub fn format_report(report: &ExecutionReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}\n", report.title);
    let _ = writeln!(out, "Scenario: {}", report.scenario);
    let _ = writeln!(out, "Summary: {}", report.summary);
    let _ = writeln!(out, "Status: {}", report.status);
    let _ = writeln!(out, "Target Language: {}\n", report.goal.target_language);

    let _ = writeln!(out, "Goal:");
    let _ = writeln!(out, "- target: {}", report.goal.target);
    let _ = writeln!(out, "- prefer AI: {}", report.goal.prefer_ai);
    let _ = writeln!(out, "- local only: {}", report.goal.local_only);
    let _ = writeln!(out, "- allow degraded: {}\n", report.goal.allow_degraded);

    let _ = writeln!(out, "Initial Context:");
    let _ = writeln!(out, "- project: {}", report.initial_context.project_name);
    let _ = writeln!(out, "- source fragments: {}", report.initial_context.source_fragments.len());
    let _ = writeln!(out, "- available capabilities: {}", report.initial_context.available_capabilities.join(", "));
    let _ = writeln!(out, "- AI available: {}\n", report.initial_context.ai_available);

    let _ = writeln!(out, "Selected Path:");
    for item in &report.selected_path {
        let _ = writeln!(out, "- {}", item);
    }
    let _ = writeln!(out);

    if report.rejected_capabilities.is_empty() {
        let _ = writeln!(out, "Rejected Capabilities: none\n");
    } else {
        let _ = writeln!(out, "Rejected Capabilities:");
        for rejection in &report.rejected_capabilities {
            let _ = writeln!(
                out,
                "- {}: {}",
                rejection.capability,
                rejection.reasons.join(" | ")
            );
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "Execution Timeline:");
    for step in &report.steps {
        let _ = writeln!(out, "Step {}: {}", step.index, step.selected_capability);
        let _ = writeln!(out, "  unmet need: {}", step.need);
        let _ = writeln!(
            out,
            "  agent provider: {}",
            step.agent_provider
        );
        let _ = writeln!(
            out,
            "  agent proposal: {}",
            step.agent_proposal.as_deref().unwrap_or("none")
        );
        if let Some(rejected) = &step.proposed_validation {
            let _ = writeln!(
                out,
                "  proposal rejected: {} ({})",
                rejected.capability,
                rejected.reasons.join(" | ")
            );
        }
        let _ = writeln!(out, "  discovery candidates:");
        for item in &step.discovery.available {
            let _ = writeln!(out, "  - {}", item.capability);
        }
        if !step.discovery.rejected.is_empty() {
            let _ = writeln!(out, "  rejected during discovery:");
            for item in &step.discovery.rejected {
                let _ = writeln!(
                    out,
                    "  - {}: {}",
                    item.capability,
                    item.reason.as_deref().unwrap_or("none")
                );
            }
        }
        let _ = writeln!(
            out,
            "  validation: {} ({})",
            step.validation.capability,
            step.validation.status
        );
        if !step.validation.reasons.is_empty() {
            let _ = writeln!(out, "  validation reasons: {}", step.validation.reasons.join(" | "));
        }
        let _ = writeln!(out, "  output: {}", step.output_preview);
        let _ = writeln!(out, "  events:");
        for event in &step.events {
            let _ = writeln!(
                out,
                "  - {} {} ({})",
                event.event_type, event.capability, event.status
            );
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "Final Output:");
    let _ = writeln!(out, "{}\n", report.final_output);

    let _ = writeln!(out, "Graph Nodes:");
    for node in &report.graph_nodes {
        let _ = writeln!(out, "- {} [{}] {}", node.label, node.kind, node.state);
    }
    let _ = writeln!(out);

    out
}

#[wasm_bindgen]
pub fn list_scenarios_json() -> String {
    serde_json::to_string_pretty(&list_scenarios(&project_root()).expect("list scenarios")).unwrap()
}

#[wasm_bindgen]
pub fn run_scenario_json(id: &str) -> String {
    serde_json::to_string_pretty(&run_scenario(&project_root(), id).expect("run scenario")).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_report_uses_deterministic_path() {
        let report = run_scenario(&project_root(), "use-case-1-basic-report").unwrap();
        assert_eq!(report.status, "complete");
        assert_eq!(
            report.selected_path,
            vec!["DataProviderLocal", "InsightEnricher", "SummarizerBasic", "Formatter"]
        );
        assert_eq!(report.final_language, "en");
    }

    #[test]
    fn french_report_inserts_translation() {
        let report = run_scenario(&project_root(), "use-case-3-french-report").unwrap();
        assert!(report.selected_path.iter().any(|item| item == "TranslatorFr"));
        assert_eq!(report.final_language, "fr");
        assert!(report.final_output.contains("Language: fr"));
    }

    #[test]
    fn agent_proposal_can_be_rejected_by_runtime() {
        let report = run_scenario(&project_root(), "use-case-5-agent-validation").unwrap();
        assert!(report
            .rejected_capabilities
            .iter()
            .any(|item| item.capability == "SummarizerAI"));
        assert!(report.selected_path.iter().any(|item| item == "SummarizerBasic"));
        assert!(!report.selected_path.iter().any(|item| item == "SummarizerAI"));
    }

    #[test]
    fn runtime_can_degrade_when_translation_is_unavailable() {
        let report = run_scenario(&project_root(), "use-case-4-runtime-adapts").unwrap();
        assert_eq!(report.status, "degraded");
        assert_eq!(report.final_language, "en (degraded)");
    }

    #[test]
    fn ai_report_prefers_ai_when_constraints_allow_it() {
        let report = run_scenario(&project_root(), "use-case-2-ai-report").unwrap();
        assert!(report.selected_path.iter().any(|item| item == "SummarizerAI"));
        assert!(!report.selected_path.iter().any(|item| item == "SummarizerBasic"));
        assert_eq!(report.final_language, "fr");
    }
}
