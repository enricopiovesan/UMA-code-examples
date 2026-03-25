use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

mod hosted;
pub mod mcp;
mod rendering;
mod storage;

pub use rendering::{format_report, list_scenarios_json, run_scenario_json};
pub use storage::{
    available_capabilities_for_scenario, capability_descriptors, list_scenarios, load_scenario, project_root,
    run_scenario,
};

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
    pub agent_mode: String,
    pub agent_fallback_reason: Option<String>,
    pub agent_proposal: Option<String>,
    pub proposed_validation: Option<ValidationResult>,
    pub selected_capability: String,
    pub execution_provider: String,
    pub execution_mode: String,
    pub fallback_reason: Option<String>,
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
    pub translated_facts: Vec<String>,
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
    pub planner_provider: String,
    pub planner_mode: String,
    pub planner_fallback_reason: Option<String>,
    pub summarizer_ai_provider: String,
    pub summarizer_ai_mode: String,
    pub summarizer_ai_fallback_reason: Option<String>,
    pub translator_ai_provider: String,
    pub translator_ai_mode: String,
    pub translator_ai_fallback_reason: Option<String>,
    pub selected_path: Vec<String>,
    pub rejected_capabilities: Vec<ValidationResult>,
    pub steps: Vec<ExecutionStep>,
    pub final_output: String,
    pub final_language: String,
    pub status: String,
    pub graph_nodes: Vec<GraphNode>,
    pub graph_edges: Vec<GraphEdge>,
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
    PlannerAI,
}

impl AgentProviderKind {
    fn label(self) -> &'static str {
        match self {
            AgentProviderKind::DeterministicLocal => "deterministic-local-planner",
            AgentProviderKind::PlannerAI => "PlannerAI",
        }
    }
}

#[derive(Debug, Clone)]
struct AgentDecision {
    provider: String,
    mode: String,
    fallback_reason: Option<String>,
    proposal: Option<String>,
}

#[derive(Debug, Clone)]
struct CapabilityExecutionMode {
    provider: String,
    mode: String,
    fallback_reason: Option<String>,
    hosted_summary: Option<String>,
    hosted_translation: Option<String>,
    hosted_translated_facts: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RuntimeHostedSummarizerResult {
    summary: String,
    provider: String,
    mode: String,
    #[serde(rename = "model_id")]
    _model_id: String,
    #[serde(rename = "model_revision")]
    _model_revision: String,
    #[serde(rename = "model_checksum")]
    _model_checksum: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeHostedTranslatorResult {
    translated_text: String,
    translated_facts: Vec<String>,
    provider: String,
    mode: String,
    #[serde(rename = "model_id")]
    _model_id: String,
    #[serde(rename = "model_revision")]
    _model_revision: String,
    #[serde(rename = "model_checksum")]
    _model_checksum: String,
}

#[derive(Debug, Serialize)]
struct PlannerCapabilityDescriptor {
    name: String,
    intent: String,
    description: String,
    tags: Vec<String>,
    runtime: Vec<String>,
    requires: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RuntimeHostedPlannerResult {
    proposal: String,
    provider: String,
    mode: String,
    #[serde(rename = "model_id")]
    _model_id: String,
    #[serde(rename = "model_revision")]
    _model_revision: String,
    #[serde(rename = "model_checksum")]
    _model_checksum: String,
}

fn agent_provider_for(_scenario: &Scenario) -> AgentProviderKind {
    if _scenario.goal.prefer_ai
        && !_scenario.goal.local_only
        && _scenario.context.ai_available
        && _scenario
            .context
            .available_capabilities
            .iter()
            .any(|item| item == "PlannerAI")
    {
        AgentProviderKind::PlannerAI
    } else {
        AgentProviderKind::DeterministicLocal
    }
}

fn propose_with_agent(
    root: &Path,
    provider: AgentProviderKind,
    need: &str,
    scenario: &Scenario,
    visible: &[CapabilityContract],
) -> AgentDecision {
    if visible.len() <= 1 {
        return AgentDecision {
            provider: "runtime-direct-selection".to_string(),
            mode: "direct".to_string(),
            fallback_reason: None,
            proposal: visible.first().map(|item| item.name.clone()),
        };
    }

    match provider {
        AgentProviderKind::DeterministicLocal => AgentDecision {
            provider: provider.label().to_string(),
            mode: "deterministic".to_string(),
            fallback_reason: None,
            proposal: deterministic_agent_proposal(need, scenario, visible),
        },
        AgentProviderKind::PlannerAI => match hosted::invoke_runtime_hosted_planner(root, need, scenario, visible) {
            Ok(result) => AgentDecision {
                provider: result.provider,
                mode: result.mode,
                fallback_reason: None,
                proposal: Some(result.proposal),
            },
            Err(reason) => AgentDecision {
                provider: AgentProviderKind::DeterministicLocal.label().to_string(),
                mode: "fallback".to_string(),
                fallback_reason: Some(reason),
                proposal: deterministic_agent_proposal(need, scenario, visible),
            },
        },
    }
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

    for contract in storage::contract_fixtures() {
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
        "translatedFacts".to_string(),
        if state.translated_facts.is_empty() {
            "none".to_string()
        } else {
            state.translated_facts.join(" | ")
        },
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

fn initial_state(scenario: &Scenario) -> RuntimeState {
    RuntimeState {
        source_loaded: false,
        source_fragments: scenario.context.source_fragments.clone(),
        structured_facts: Vec::new(),
        summary: None,
        translated_summary: None,
        translated_facts: Vec::new(),
        report: None,
        degraded: false,
    }
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
    let narrative = match state.translated_summary.clone().or_else(|| state.summary.clone()) {
        Some(text) => text,
        None => "No summary available".to_string(),
    };
    let insights_source = if !state.translated_facts.is_empty() {
        &state.translated_facts
    } else {
        &state.structured_facts
    };
    let insights = if insights_source.is_empty() {
        "No structured facts".to_string()
    } else {
        insights_source
            .iter()
            .take(2)
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let (project_label, language_label, summary_label, highlights_label) = if language == "fr" {
        ("Projet", "Langue", "Resume", "Points cles")
    } else {
        ("Project", "Language", "Summary", "Highlights")
    };
    format!(
        "{project_label}: {project}\n{language_label}: {language}\n{summary_label}: {narrative}\n{highlights_label}: {insights}"
    )
}

fn select_contract_for_step(
    visible: &[CapabilityContract],
    scenario: &Scenario,
    state: &RuntimeState,
    proposal: &Option<String>,
    rejected_capabilities: &mut Vec<ValidationResult>,
) -> (Option<CapabilityContract>, Option<ValidationResult>, Option<ValidationResult>) {
    let mut selected_contract = None;
    let mut validation = None;
    let mut proposed_validation = None;

    if let Some(proposed_name) = proposal.clone() {
        if let Some(contract) = visible.iter().find(|item| item.name == proposed_name) {
            let result = validate_contract(contract, scenario, state);
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
        for contract in visible {
            let result = validate_contract(contract, scenario, state);
            if result.status == "accepted" {
                selected_contract = Some(contract.clone());
                validation = Some(result);
                break;
            }
            rejected_capabilities.push(result);
        }
    }

    (selected_contract, validation, proposed_validation)
}

fn execute_selected_capability(
    contract: &CapabilityContract,
    scenario: &Scenario,
    state: &mut RuntimeState,
    execution_mode: &CapabilityExecutionMode,
    events: &mut Vec<RuntimeEvent>,
) -> String {
    match contract.name.as_str() {
        "DataProviderLocal" => {
            state.source_loaded = true;
            events.push(emit_event(
                "CapabilityExecuted",
                &contract.name,
                "success",
                None,
                state,
            ));
            format!(
                "Loaded {} distributed source fragment(s)",
                state.source_fragments.len()
            )
        }
        "InsightEnricher" => {
            state.structured_facts = enrich_facts(&state.source_fragments);
            events.push(emit_event(
                "ContextUpdated",
                &contract.name,
                "success",
                None,
                state,
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
                state,
            ));
            state.summary.clone().unwrap()
        }
        "SummarizerAI" => {
            let ai_summary = if execution_mode.mode == "runtime-hosted-extractive" {
                execution_mode
                    .hosted_summary
                    .clone()
                    .expect("runtime-hosted SummarizerAI missing cached hosted summary")
            } else {
                summarize_ai(&scenario.context.project_name, &state.structured_facts)
            };
            state.summary = Some(ai_summary);
            events.push(emit_event(
                "CapabilityExecuted",
                &contract.name,
                if execution_mode.fallback_reason.is_some() {
                    "fallback"
                } else {
                    "success"
                },
                execution_mode.fallback_reason.clone(),
                state,
            ));
            state.summary.clone().unwrap()
        }
        "TranslatorFr" => {
            let translated = if execution_mode.mode == "runtime-hosted-translation" {
                execution_mode
                    .hosted_translation
                    .clone()
                    .expect("runtime-hosted TranslatorFr missing cached hosted translation")
            } else {
                translate_french(state.summary.as_deref().unwrap_or_default())
            };
            state.translated_summary = Some(translated.clone());
            state.translated_facts = execution_mode
                .hosted_translated_facts
                .clone()
                .unwrap_or_else(|| {
                    state
                        .structured_facts
                        .iter()
                        .map(|fact| translate_french(fact))
                        .collect()
                });
            events.push(emit_event(
                "CapabilityExecuted",
                &contract.name,
                if execution_mode.fallback_reason.is_some() {
                    "fallback"
                } else {
                    "success"
                },
                execution_mode.fallback_reason.clone(),
                state,
            ));
            translated
        }
        "Formatter" => {
            let language = if state.translated_summary.is_some() {
                "fr"
            } else {
                scenario.goal.target_language.as_str()
            };
            let report = format_report_output(&scenario.context.project_name, state, language);
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
                state,
            ));
            report
        }
        _ => "No output".to_string(),
    }
}

fn push_result_edge(graph_edges: &mut Vec<GraphEdge>, last: &str) {
    graph_edges.push(GraphEdge {
        from: last.to_string(),
        to: "result".into(),
        state: "complete".into(),
    });
}

pub(crate) fn run_loaded_scenario(root: &Path, scenario: Scenario) -> Result<ExecutionReport, String> {
    let mut state = initial_state(&scenario);

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

        let (visible, discovery) = discover_candidates(&scenario, &state, need);
        let agent_decision = propose_with_agent(root, agent_provider_for(&scenario), need, &scenario, &visible);
        let proposal = agent_decision.proposal.clone();
        let (selected_contract, validation, proposed_validation) = select_contract_for_step(
            &visible,
            &scenario,
            &state,
            &proposal,
            &mut rejected_capabilities,
        );

        let contract = match selected_contract {
            Some(contract) => contract,
            None => {
                if scenario.goal.allow_degraded {
                    state.degraded = true;
                    return Err("no compatible capability satisfied the current need; degraded path required".to_string());
                }
                return Err("no compatible capability satisfied the current need".to_string());
            }
        };
        let validation = validation.expect("validation exists");
        let proposal = proposal.expect("selected step must have an agent proposal");
        let execution_mode = hosted::execution_mode_for(
            root,
            &contract,
            &scenario.context.project_name,
            &state.structured_facts,
            state.summary.as_deref(),
        );

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
        events.push(emit_event(
            "CapabilityProposed",
            &proposal,
            "proposed",
            None,
            &state,
        ));
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

        let output_preview = execute_selected_capability(
            &contract,
            &scenario,
            &mut state,
            &execution_mode,
            &mut events,
        );

        selected_path.push(contract.name.clone());
        steps.push(ExecutionStep {
            index,
            need: need.to_string(),
            agent_provider: agent_decision.provider.clone(),
            agent_mode: agent_decision.mode.clone(),
            agent_fallback_reason: agent_decision.fallback_reason.clone(),
            agent_proposal: Some(proposal),
            proposed_validation,
            selected_capability: contract.name,
            execution_provider: execution_mode.provider,
            execution_mode: execution_mode.mode,
            fallback_reason: execution_mode.fallback_reason,
            discovery,
            validation,
            output_preview,
            events,
        });
    }

    let final_output = state
        .report
        .clone()
        .expect("scenario ended without a final report");
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

    let planner_summary_step = steps.iter().find(|step| step.agent_mode != "direct");
    let planner_provider = match planner_summary_step {
        Some(step) => step.agent_provider.clone(),
        None => agent_provider_for(&scenario).label().to_string(),
    };
    let planner_mode = match planner_summary_step {
        Some(step) => step.agent_mode.clone(),
        None => "direct".to_string(),
    };
    let planner_fallback_reason = steps
        .iter()
        .find_map(|step| step.agent_fallback_reason.clone());
    let translator_ai_step = steps.iter().find(|step| step.selected_capability == "TranslatorFr");
    let translator_ai_provider = match translator_ai_step {
        Some(step) => step.execution_provider.clone(),
        None => "not-used".to_string(),
    };
    let translator_ai_mode = match translator_ai_step {
        Some(step) => step.execution_mode.clone(),
        None => "not-used".to_string(),
    };
    let translator_ai_fallback_reason = translator_ai_step
        .and_then(|step| step.fallback_reason.clone());

    let mut graph_nodes = vec![
        GraphNode { id: "goal".into(), label: "Goal".into(), kind: "goal".into(), state: "complete".into(), x: -280.0, y: -180.0, z: -80.0 },
        GraphNode { id: "mcp".into(), label: "MCP node".into(), kind: "mcp".into(), state: "active".into(), x: -150.0, y: -40.0, z: 40.0 },
        GraphNode {
            id: "agent".into(),
            label: planner_provider.clone(),
            kind: "agent".into(),
            state: if scenario.goal.prefer_ai || scenario.context.available_capabilities.iter().any(|item| item == "PlannerAI") {
                "active".into()
            } else {
                "candidate".into()
            },
            x: 130.0,
            y: -180.0,
            z: 80.0,
        },
        GraphNode { id: "runtime".into(), label: "UMA runtime".into(), kind: "runtime".into(), state: "active".into(), x: -20.0, y: -60.0, z: -20.0 },
        GraphNode { id: "result".into(), label: "Result".into(), kind: "result".into(), state: "complete".into(), x: 320.0, y: 180.0, z: 40.0 },
    ];

    let capability_positions = [
        ("DataProviderLocal", -250.0, 110.0, -10.0),
        ("InsightEnricher", -90.0, 150.0, 20.0),
        ("PlannerAI", 60.0, -150.0, 100.0),
        ("SummarizerBasic", 80.0, 40.0, 60.0),
        ("SummarizerAI", 120.0, 140.0, 110.0),
        ("TranslatorFr", 250.0, 90.0, 20.0),
        ("Formatter", 210.0, 210.0, -20.0),
    ];

    for contract in storage::contract_fixtures() {
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
    let last = selected_path
        .last()
        .expect("completed scenario has a selected capability");
    push_result_edge(&mut graph_edges, last);

    let summarizer_ai_step = steps
        .iter()
        .find(|step| step.selected_capability == "SummarizerAI");
    let summarizer_ai_provider = match summarizer_ai_step {
        Some(step) => step.execution_provider.clone(),
        None => "not-invoked".to_string(),
    };
    let summarizer_ai_mode = match summarizer_ai_step {
        Some(step) => step.execution_mode.clone(),
        None => "not-invoked".to_string(),
    };
    let summarizer_ai_fallback_reason = summarizer_ai_step.and_then(|step| step.fallback_reason.clone());

    Ok(ExecutionReport {
        scenario: scenario.id,
        title: scenario.title,
        summary: scenario.summary,
        goal: scenario.goal,
        initial_context: scenario.context,
        planner_provider,
        planner_mode,
        planner_fallback_reason,
        summarizer_ai_provider,
        summarizer_ai_mode,
        summarizer_ai_fallback_reason,
        translator_ai_provider,
        translator_ai_mode,
        translator_ai_fallback_reason,
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

#[cfg(test)]
mod lib_tests;
