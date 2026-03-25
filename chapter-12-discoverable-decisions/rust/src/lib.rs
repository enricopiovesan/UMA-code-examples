use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawScenario {
    title: String,
    summary: String,
    decision_question: String,
    surfaces: Vec<RawSurface>,
    proposal: RawProposal,
    validation: RawValidation,
    execution: RawExecution,
    trace: RawTrace,
    functions: RawFunctions,
    axes: RawAxes,
    expectations: Vec<String>,
    #[serde(default)]
    runtime_decisions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawSurface {
    name: String,
    runtime: String,
    scope: String,
    authority: String,
    visible_capabilities: Vec<String>,
    hidden_constraints: Vec<String>,
    queries: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawProposal {
    status: String,
    summary: String,
    assumptions: Vec<String>,
    unresolved: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawValidation {
    status: String,
    summary: String,
    #[serde(default)]
    violations: Vec<String>,
    #[serde(default)]
    guidance: Vec<String>,
    #[serde(default)]
    authority_notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawExecution {
    status: String,
    summary: String,
    #[serde(default)]
    selected_capabilities: Vec<String>,
    #[serde(default)]
    placement: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawTrace {
    status: String,
    summary: String,
    artifacts: Vec<String>,
    queries: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawFunctions {
    planning: String,
    validation: String,
    execution: String,
}

#[derive(Debug, Deserialize)]
struct RawAxes {
    projection_scope: String,
    proposal_visibility: String,
    authority_model: String,
    revision_model: String,
    execution_model: String,
    traceability: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Surface {
    pub name: String,
    pub runtime: String,
    pub scope: String,
    pub authority: String,
    #[serde(rename = "visibleCapabilities")]
    pub visible_capabilities: Vec<String>,
    #[serde(rename = "hiddenConstraints")]
    pub hidden_constraints: Vec<String>,
    pub queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Proposal {
    pub status: String,
    pub summary: String,
    pub assumptions: Vec<String>,
    pub unresolved: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Validation {
    pub status: String,
    pub summary: String,
    pub violations: Vec<String>,
    pub guidance: Vec<String>,
    #[serde(rename = "authorityNotes")]
    pub authority_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Execution {
    pub status: String,
    pub summary: String,
    #[serde(rename = "selectedCapabilities")]
    pub selected_capabilities: Vec<String>,
    pub placement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Trace {
    pub status: String,
    pub summary: String,
    pub artifacts: Vec<String>,
    pub queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Functions {
    pub planning: String,
    pub validation: String,
    pub execution: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Axes {
    #[serde(rename = "projectionScope")]
    pub projection_scope: String,
    #[serde(rename = "proposalVisibility")]
    pub proposal_visibility: String,
    #[serde(rename = "authorityModel")]
    pub authority_model: String,
    #[serde(rename = "revisionModel")]
    pub revision_model: String,
    #[serde(rename = "executionModel")]
    pub execution_model: String,
    pub traceability: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Warning {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Assessment {
    pub verdict: String,
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ScenarioReport {
    pub scenario: String,
    pub title: String,
    pub summary: String,
    #[serde(rename = "decisionQuestion")]
    pub decision_question: String,
    pub surfaces: Vec<Surface>,
    pub proposal: Proposal,
    pub validation: Validation,
    pub execution: Execution,
    pub trace: Trace,
    pub functions: Functions,
    pub axes: Axes,
    pub expectations: Vec<String>,
    #[serde(rename = "runtimeDecisions")]
    pub runtime_decisions: Vec<String>,
    pub assessment: Assessment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDiff {
    pub from: String,
    pub to: String,
    pub verdict_change: String,
    pub added_warnings: Vec<String>,
    pub removed_warnings: Vec<String>,
    pub changed_axes: Vec<String>,
    pub changed_stages: Vec<String>,
}

fn err_string<E: ToString>(err: E) -> String {
    err.to_string()
}

fn warning_codes(warnings: &[Warning]) -> BTreeSet<String> {
    let mut codes = BTreeSet::new();
    for warning in warnings {
        codes.insert(warning.code.clone());
    }
    codes
}

fn ensure_non_empty(value: &str, field: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{label}: \"{field}\" must be a non-empty string"));
    }
    Ok(())
}

fn ensure_list(values: &[String], field: &str, label: &str) -> Result<(), String> {
    for value in values {
        ensure_non_empty(value, field, label)?;
    }
    Ok(())
}

fn validate_scenario(raw: RawScenario, label: &str) -> Result<ScenarioReport, String> {
    ensure_non_empty(&raw.title, "title", label)?;
    ensure_non_empty(&raw.summary, "summary", label)?;
    ensure_non_empty(&raw.decision_question, "decision_question", label)?;
    ensure_non_empty(&raw.proposal.status, "proposal.status", label)?;
    ensure_non_empty(&raw.proposal.summary, "proposal.summary", label)?;
    ensure_non_empty(&raw.validation.status, "validation.status", label)?;
    ensure_non_empty(&raw.validation.summary, "validation.summary", label)?;
    ensure_non_empty(&raw.execution.status, "execution.status", label)?;
    ensure_non_empty(&raw.execution.summary, "execution.summary", label)?;
    ensure_non_empty(&raw.trace.status, "trace.status", label)?;
    ensure_non_empty(&raw.trace.summary, "trace.summary", label)?;
    ensure_non_empty(&raw.functions.planning, "functions.planning", label)?;
    ensure_non_empty(&raw.functions.validation, "functions.validation", label)?;
    ensure_non_empty(&raw.functions.execution, "functions.execution", label)?;
    ensure_non_empty(&raw.axes.projection_scope, "axes.projection_scope", label)?;
    ensure_non_empty(&raw.axes.proposal_visibility, "axes.proposal_visibility", label)?;
    ensure_non_empty(&raw.axes.authority_model, "axes.authority_model", label)?;
    ensure_non_empty(&raw.axes.revision_model, "axes.revision_model", label)?;
    ensure_non_empty(&raw.axes.execution_model, "axes.execution_model", label)?;
    ensure_non_empty(&raw.axes.traceability, "axes.traceability", label)?;
    ensure_list(&raw.expectations, "expectations", label)?;
    ensure_list(&raw.runtime_decisions, "runtime_decisions", label)?;
    ensure_list(&raw.proposal.assumptions, "proposal.assumptions", label)?;
    ensure_list(&raw.proposal.unresolved, "proposal.unresolved", label)?;
    ensure_list(&raw.validation.violations, "validation.violations", label)?;
    ensure_list(&raw.validation.guidance, "validation.guidance", label)?;
    ensure_list(&raw.validation.authority_notes, "validation.authority_notes", label)?;
    ensure_list(
        &raw.execution.selected_capabilities,
        "execution.selected_capabilities",
        label,
    )?;
    ensure_list(&raw.execution.placement, "execution.placement", label)?;
    ensure_list(&raw.trace.artifacts, "trace.artifacts", label)?;
    ensure_list(&raw.trace.queries, "trace.queries", label)?;

    let mut surface_names = BTreeSet::new();
    let mut surfaces = Vec::new();
    for surface in raw.surfaces {
        ensure_non_empty(&surface.name, "surfaces[].name", label)?;
        ensure_non_empty(&surface.runtime, "surfaces[].runtime", label)?;
        ensure_non_empty(&surface.scope, "surfaces[].scope", label)?;
        ensure_non_empty(&surface.authority, "surfaces[].authority", label)?;
        ensure_list(
            &surface.visible_capabilities,
            "surfaces[].visible_capabilities",
            label,
        )?;
        ensure_list(
            &surface.hidden_constraints,
            "surfaces[].hidden_constraints",
            label,
        )?;
        ensure_list(&surface.queries, "surfaces[].queries", label)?;
        if !surface_names.insert(surface.name.clone()) {
            return Err(format!("{label}: duplicate surface name \"{}\"", surface.name));
        }
        surfaces.push(Surface {
            name: surface.name,
            runtime: surface.runtime,
            scope: surface.scope,
            authority: surface.authority,
            visible_capabilities: surface.visible_capabilities,
            hidden_constraints: surface.hidden_constraints,
            queries: surface.queries,
        });
    }
    surfaces.sort_by(|a, b| a.name.cmp(&b.name));

    let proposal = Proposal {
        status: raw.proposal.status,
        summary: raw.proposal.summary,
        assumptions: raw.proposal.assumptions,
        unresolved: raw.proposal.unresolved,
    };

    let validation = Validation {
        status: raw.validation.status,
        summary: raw.validation.summary,
        violations: raw.validation.violations,
        guidance: raw.validation.guidance,
        authority_notes: raw.validation.authority_notes,
    };

    let execution = Execution {
        status: raw.execution.status,
        summary: raw.execution.summary,
        selected_capabilities: raw.execution.selected_capabilities,
        placement: raw.execution.placement,
    };

    let trace = Trace {
        status: raw.trace.status,
        summary: raw.trace.summary,
        artifacts: raw.trace.artifacts,
        queries: raw.trace.queries,
    };

    let functions = Functions {
        planning: raw.functions.planning,
        validation: raw.functions.validation,
        execution: raw.functions.execution,
    };

    let axes = Axes {
        projection_scope: raw.axes.projection_scope,
        proposal_visibility: raw.axes.proposal_visibility,
        authority_model: raw.axes.authority_model,
        revision_model: raw.axes.revision_model,
        execution_model: raw.axes.execution_model,
        traceability: raw.axes.traceability,
    };

    let assessment = assess(&axes, &validation, &execution);

    Ok(ScenarioReport {
        scenario: label.to_string(),
        title: raw.title,
        summary: raw.summary,
        decision_question: raw.decision_question,
        surfaces,
        proposal,
        validation,
        execution,
        trace,
        functions,
        axes,
        expectations: raw.expectations,
        runtime_decisions: raw.runtime_decisions,
        assessment,
    })
}

fn assess(axes: &Axes, validation: &Validation, execution: &Execution) -> Assessment {
    let mut warnings = Vec::new();

    if axes.proposal_visibility == "hidden" {
        warnings.push(Warning {
            code: "proposal_hidden".into(),
            message: "The system is changing behavior, but the proposed decision is not visible as an inspectable artifact."
                .into(),
        });
    }

    if axes.authority_model == "local-only" {
        warnings.push(Warning {
            code: "authority_gap".into(),
            message: "Local reasoning is acting without authoritative validation of trust, compliance, or global constraints."
                .into(),
        });
    }

    if validation.status == "violations" && validation.guidance.is_empty() {
        warnings.push(Warning {
            code: "validation_without_guidance".into(),
            message: "Constraint failures are detected, but the planner is not given actionable remediation guidance."
                .into(),
        });
    }

    if axes.revision_model == "unbounded" {
        warnings.push(Warning {
            code: "unbounded_revision".into(),
            message: "The planner and validator can keep negotiating indefinitely, which makes the decision lifecycle difficult to reason about."
                .into(),
        });
    }

    if axes.execution_model == "implicit-replan" {
        warnings.push(Warning {
            code: "implicit_replanning".into(),
            message: "Execution is reinterpreting the approved intent instead of resolving a validated proposal."
                .into(),
        });
    }

    if axes.traceability == "absent" {
        warnings.push(Warning {
            code: "missing_trace".into(),
            message: "The system produces an outcome without first-class trace artifacts that explain how the decision was formed."
                .into(),
        });
    }

    if axes.traceability == "proposal-only" && execution.status == "resolved" {
        warnings.push(Warning {
            code: "partial_trace".into(),
            message: "The system records planning artifacts, but not the full path from approved proposal to concrete execution."
                .into(),
        });
    }

    let verdict = if warnings
        .iter()
        .any(|warning| matches!(warning.code.as_str(), "implicit_replanning" | "unbounded_revision"))
    {
        "unstable"
    } else if warnings
        .iter()
        .any(|warning| matches!(warning.code.as_str(), "proposal_hidden" | "authority_gap"))
    {
        "opaque"
    } else if axes.authority_model == "authoritative-validation"
        && axes.revision_model == "single-revision"
        && axes.execution_model == "approved-resolution"
        && axes.traceability == "full-trace"
        && axes.proposal_visibility == "queryable"
    {
        "governed"
    } else {
        "discoverable"
    };

    Assessment {
        verdict: verdict.into(),
        warnings,
    }
}

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("chapter root")
        .to_path_buf()
}

pub fn list_labs(root: &Path) -> Result<Vec<String>, String> {
    let mut labs = Vec::new();
    for entry in fs::read_dir(root.join("scenarios"))
        .map_err(err_string)?
        .flatten()
    {
        if entry.file_type().is_ok_and(|file_type| file_type.is_dir()) {
            labs.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    labs.sort();
    Ok(labs)
}

fn format_list(values: &[String], separator: &str) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(separator)
    }
}

pub fn load_report(root: &Path, lab: &str) -> Result<ScenarioReport, String> {
    let labs = list_labs(root)?;
    if !labs.iter().any(|item| item == lab) {
        return Err(format!(
            "unknown lab \"{lab}\". Available labs: {}",
            labs.join(", ")
        ));
    }

    let path = root.join("scenarios").join(lab).join("scenario.json");
    let raw = fs::read_to_string(&path).map_err(err_string)?;
    let scenario = serde_json::from_str::<RawScenario>(&raw).map_err(err_string)?;
    validate_scenario(scenario, lab)
}

pub fn validate_all(root: &Path) -> Result<Vec<String>, String> {
    let labs = list_labs(root)?;
    let mut summaries = Vec::with_capacity(labs.len());
    let mut index = 0usize;
    while index < labs.len() {
        let report = load_report(root, &labs[index])?;
        summaries.push(format!(
            "Validated {}: {} surfaces, verdict={}",
            report.scenario,
            report.surfaces.len(),
            report.assessment.verdict
        ));
        index += 1;
    }
    Ok(summaries)
}

pub fn diff_reports(from: &ScenarioReport, to: &ScenarioReport) -> ScenarioDiff {
    let left = warning_codes(&from.assessment.warnings);
    let right = warning_codes(&to.assessment.warnings);

    let axes = [
        (
            "projection scope",
            from.axes.projection_scope.as_str(),
            to.axes.projection_scope.as_str(),
        ),
        (
            "proposal visibility",
            from.axes.proposal_visibility.as_str(),
            to.axes.proposal_visibility.as_str(),
        ),
        (
            "authority model",
            from.axes.authority_model.as_str(),
            to.axes.authority_model.as_str(),
        ),
        (
            "revision model",
            from.axes.revision_model.as_str(),
            to.axes.revision_model.as_str(),
        ),
        (
            "execution model",
            from.axes.execution_model.as_str(),
            to.axes.execution_model.as_str(),
        ),
        (
            "traceability",
            from.axes.traceability.as_str(),
            to.axes.traceability.as_str(),
        ),
    ];

    let mut changed_axes = Vec::new();
    for (label, left_value, right_value) in axes {
        if left_value != right_value {
            changed_axes.push(format!("{label}: {left_value} -> {right_value}"));
        }
    }

    let stages = [
        (
            "proposal status",
            from.proposal.status.as_str(),
            to.proposal.status.as_str(),
        ),
        (
            "validation status",
            from.validation.status.as_str(),
            to.validation.status.as_str(),
        ),
        (
            "execution status",
            from.execution.status.as_str(),
            to.execution.status.as_str(),
        ),
        ("trace status", from.trace.status.as_str(), to.trace.status.as_str()),
    ];

    let mut changed_stages = Vec::new();
    for (label, left_value, right_value) in stages {
        if left_value != right_value {
            changed_stages.push(format!("{label}: {left_value} -> {right_value}"));
        }
    }

    ScenarioDiff {
        from: from.scenario.clone(),
        to: to.scenario.clone(),
        verdict_change: format!("{} -> {}", from.assessment.verdict, to.assessment.verdict),
        added_warnings: right.difference(&left).cloned().collect(),
        removed_warnings: left.difference(&right).cloned().collect(),
        changed_axes,
        changed_stages,
    }
}

pub fn format_report(report: &ScenarioReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}\n", report.title);
    let _ = writeln!(out, "Scenario: {}", report.scenario);
    let _ = writeln!(out, "Summary: {}", report.summary);
    let _ = writeln!(out, "Decision Question: {}\n", report.decision_question);

    let _ = writeln!(out, "Verdict: {}", report.assessment.verdict);
    if report.assessment.warnings.is_empty() {
        let _ = writeln!(out, "Warnings: none\n");
    } else {
        let _ = writeln!(out, "Warnings:");
        for warning in &report.assessment.warnings {
            let _ = writeln!(out, "- {}: {}", warning.code, warning.message);
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "Decision Axes:");
    let _ = writeln!(out, "- projection scope: {}", report.axes.projection_scope);
    let _ = writeln!(out, "- proposal visibility: {}", report.axes.proposal_visibility);
    let _ = writeln!(out, "- authority model: {}", report.axes.authority_model);
    let _ = writeln!(out, "- revision model: {}", report.axes.revision_model);
    let _ = writeln!(out, "- execution model: {}", report.axes.execution_model);
    let _ = writeln!(out, "- traceability: {}\n", report.axes.traceability);

    let _ = writeln!(out, "Discoverable Surfaces:");
    for surface in &report.surfaces {
        let _ = writeln!(
            out,
            "- {} ({}, scope={}, authority={})",
            surface.name, surface.runtime, surface.scope, surface.authority
        );
        let _ = writeln!(
            out,
            "  visible: {}",
            format_list(&surface.visible_capabilities, ", ")
        );
        let hidden = if surface.hidden_constraints.is_empty() {
            "none".to_string()
        } else {
            surface.hidden_constraints.join(", ")
        };
        let _ = writeln!(out, "  hidden: {}", hidden);
        let _ = writeln!(out, "  queries: {}", format_list(&surface.queries, " | "));
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "Proposal:");
    let _ = writeln!(out, "- status: {}", report.proposal.status);
    let _ = writeln!(out, "- summary: {}", report.proposal.summary);
    let _ = writeln!(
        out,
        "- assumptions: {}",
        format_list(&report.proposal.assumptions, " | ")
    );
    let unresolved = format_list(&report.proposal.unresolved, " | ");
    let _ = writeln!(out, "- unresolved: {}\n", unresolved);

    let _ = writeln!(out, "Validation:");
    let _ = writeln!(out, "- status: {}", report.validation.status);
    let _ = writeln!(out, "- summary: {}", report.validation.summary);
    if report.validation.violations.is_empty() {
        let _ = writeln!(out, "- violations: none");
    } else {
        let _ = writeln!(
            out,
            "- violations: {}",
            format_list(&report.validation.violations, " | ")
        );
    }
    if report.validation.guidance.is_empty() {
        let _ = writeln!(out, "- guidance: none");
    } else {
        let _ = writeln!(
            out,
            "- guidance: {}",
            format_list(&report.validation.guidance, " | ")
        );
    }
    if report.validation.authority_notes.is_empty() {
        let _ = writeln!(out, "- authority notes: none\n");
    } else {
        let _ = writeln!(
            out,
            "- authority notes: {}\n",
            format_list(&report.validation.authority_notes, " | ")
        );
    }

    let _ = writeln!(out, "Execution:");
    let _ = writeln!(out, "- status: {}", report.execution.status);
    let _ = writeln!(out, "- summary: {}", report.execution.summary);
    if report.execution.selected_capabilities.is_empty() {
        let _ = writeln!(out, "- selected capabilities: none");
    } else {
        let _ = writeln!(
            out,
            "- selected capabilities: {}",
            format_list(&report.execution.selected_capabilities, ", ")
        );
    }
    if report.execution.placement.is_empty() {
        let _ = writeln!(out, "- placement: none\n");
    } else {
        let _ = writeln!(
            out,
            "- placement: {}\n",
            format_list(&report.execution.placement, " | ")
        );
    }

    let _ = writeln!(out, "Trace:");
    let _ = writeln!(out, "- status: {}", report.trace.status);
    let _ = writeln!(out, "- summary: {}", report.trace.summary);
    let _ = writeln!(out, "- artifacts: {}", format_list(&report.trace.artifacts, " | "));
    let _ = writeln!(out, "- queries: {}\n", format_list(&report.trace.queries, " | "));

    let _ = writeln!(out, "Functions, Not Fixed Roles:");
    let _ = writeln!(out, "- planning: {}", report.functions.planning);
    let _ = writeln!(out, "- validation: {}", report.functions.validation);
    let _ = writeln!(out, "- execution: {}\n", report.functions.execution);

    let _ = writeln!(out, "Runtime Decisions:");
    for item in &report.runtime_decisions {
        let _ = writeln!(out, "- {}", item);
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "Reader Value:");
    for item in &report.expectations {
        let _ = writeln!(out, "- {}", item);
    }

    out
}

pub fn format_diff(diff: &ScenarioDiff) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "From: {}", diff.from);
    let _ = writeln!(out, "To: {}", diff.to);
    let _ = writeln!(out, "Verdict: {}", diff.verdict_change);

    if diff.changed_axes.is_empty() {
        let _ = writeln!(out, "Changed Axes: none");
    } else {
        let _ = writeln!(out, "Changed Axes:");
        for axis in &diff.changed_axes {
            let _ = writeln!(out, "- {}", axis);
        }
    }

    if diff.changed_stages.is_empty() {
        let _ = writeln!(out, "Changed Stages: none");
    } else {
        let _ = writeln!(out, "Changed Stages:");
        for stage in &diff.changed_stages {
            let _ = writeln!(out, "- {}", stage);
        }
    }

    if diff.added_warnings.is_empty() {
        let _ = writeln!(out, "Added Warnings: none");
    } else {
        let _ = writeln!(out, "Added Warnings: {}", diff.added_warnings.join(", "));
    }

    if diff.removed_warnings.is_empty() {
        let _ = writeln!(out, "Removed Warnings: none");
    } else {
        let _ = writeln!(
            out,
            "Removed Warnings: {}",
            diff.removed_warnings.join(", ")
        );
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("chapter12-tests-{pid}-{nanos}-{counter}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_file(path: &Path, contents: &str) {
        let parent = path.parent().expect("write_file paths should have a parent");
        fs::create_dir_all(parent).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn raw_scenario() -> RawScenario {
        RawScenario {
            title: "Traceable decision".into(),
            summary: "A discoverable decision lifecycle.".into(),
            decision_question: "Which capability should resolve the request?".into(),
            surfaces: vec![
                RawSurface {
                    name: "Runtime view".into(),
                    runtime: "uma-runtime".into(),
                    scope: "global".into(),
                    authority: "authoritative-validation".into(),
                    visible_capabilities: vec!["planner".into(), "formatter".into()],
                    hidden_constraints: vec!["policy".into()],
                    queries: vec!["why was formatter selected?".into()],
                },
                RawSurface {
                    name: "Planner view".into(),
                    runtime: "planner".into(),
                    scope: "local".into(),
                    authority: "proposal".into(),
                    visible_capabilities: vec!["formatter".into()],
                    hidden_constraints: vec![],
                    queries: vec!["what was unresolved?".into()],
                },
            ],
            proposal: RawProposal {
                status: "proposed".into(),
                summary: "Planner proposes formatter.".into(),
                assumptions: vec!["formatter is available".into()],
                unresolved: vec!["none".into()],
            },
            validation: RawValidation {
                status: "approved".into(),
                summary: "Runtime approves the choice.".into(),
                violations: vec![],
                guidance: vec!["keep the trace".into()],
                authority_notes: vec!["validated against policy".into()],
            },
            execution: RawExecution {
                status: "resolved".into(),
                summary: "Formatter executed.".into(),
                selected_capabilities: vec!["formatter".into()],
                placement: vec!["cloud".into()],
            },
            trace: RawTrace {
                status: "captured".into(),
                summary: "Trace is queryable.".into(),
                artifacts: vec!["proposal".into(), "validation".into()],
                queries: vec!["why formatter".into()],
            },
            functions: RawFunctions {
                planning: "planner narrows options".into(),
                validation: "runtime enforces policy".into(),
                execution: "approved execution resolves".into(),
            },
            axes: RawAxes {
                projection_scope: "runtime-wide".into(),
                proposal_visibility: "queryable".into(),
                authority_model: "authoritative-validation".into(),
                revision_model: "single-revision".into(),
                execution_model: "approved-resolution".into(),
                traceability: "full-trace".into(),
            },
            expectations: vec!["Reader should see the decision path".into()],
            runtime_decisions: vec!["Formatter selected after validation".into()],
        }
    }

    #[test]
    fn hidden_proposal_is_opaque() {
        let report = load_report(&project_root(), "lab1-capability-projection").unwrap();
        assert_eq!(report.assessment.verdict, "opaque");
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "proposal_hidden"));
    }

    #[test]
    fn approved_execution_without_full_trace_is_flagged() {
        let report = load_report(&project_root(), "lab5-approved-execution").unwrap();
        assert_eq!(report.assessment.verdict, "discoverable");
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "partial_trace"));
    }

    #[test]
    fn final_lab_is_governed() {
        let report = load_report(&project_root(), "lab6-queryable-trace").unwrap();
        assert_eq!(report.assessment.verdict, "governed");
        assert!(report.assessment.warnings.is_empty());
    }

    #[test]
    fn formatter_prints_none_for_empty_trace_artifacts() {
        let report = load_report(&project_root(), "lab1-capability-projection").unwrap();
        let rendered = format_report(&report);
        assert!(rendered.contains("- artifacts: none"));
        assert!(rendered.contains("- queries: none"));
    }

    #[test]
    fn validate_scenario_rejects_required_blank_fields_and_duplicates() {
        let top_level_cases: [(&str, fn(&mut RawScenario), &str); 18] = [
            ("title", |raw| raw.title = " ".into(), "\"title\""),
            ("summary", |raw| raw.summary = " ".into(), "\"summary\""),
            (
                "decision_question",
                |raw| raw.decision_question = " ".into(),
                "\"decision_question\"",
            ),
            (
                "proposal.status",
                |raw| raw.proposal.status = " ".into(),
                "\"proposal.status\"",
            ),
            (
                "proposal.summary",
                |raw| raw.proposal.summary = " ".into(),
                "\"proposal.summary\"",
            ),
            (
                "validation.status",
                |raw| raw.validation.status = " ".into(),
                "\"validation.status\"",
            ),
            (
                "validation.summary",
                |raw| raw.validation.summary = " ".into(),
                "\"validation.summary\"",
            ),
            (
                "execution.status",
                |raw| raw.execution.status = " ".into(),
                "\"execution.status\"",
            ),
            (
                "execution.summary",
                |raw| raw.execution.summary = " ".into(),
                "\"execution.summary\"",
            ),
            ("trace.status", |raw| raw.trace.status = " ".into(), "\"trace.status\""),
            ("trace.summary", |raw| raw.trace.summary = " ".into(), "\"trace.summary\""),
            (
                "functions.planning",
                |raw| raw.functions.planning = " ".into(),
                "\"functions.planning\"",
            ),
            (
                "functions.validation",
                |raw| raw.functions.validation = " ".into(),
                "\"functions.validation\"",
            ),
            (
                "functions.execution",
                |raw| raw.functions.execution = " ".into(),
                "\"functions.execution\"",
            ),
            (
                "axes.projection_scope",
                |raw| raw.axes.projection_scope = " ".into(),
                "\"axes.projection_scope\"",
            ),
            (
                "axes.proposal_visibility",
                |raw| raw.axes.proposal_visibility = " ".into(),
                "\"axes.proposal_visibility\"",
            ),
            (
                "axes.authority_model",
                |raw| raw.axes.authority_model = " ".into(),
                "\"axes.authority_model\"",
            ),
            (
                "axes.traceability",
                |raw| raw.axes.traceability = " ".into(),
                "\"axes.traceability\"",
            ),
        ];
        for (label, mutate, expected) in top_level_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let list_cases: [(&str, fn(&mut RawScenario), &str); 10] = [
            (
                "expectations",
                |raw| raw.expectations[0] = " ".into(),
                "\"expectations\"",
            ),
            (
                "runtime_decisions",
                |raw| raw.runtime_decisions[0] = " ".into(),
                "\"runtime_decisions\"",
            ),
            (
                "proposal.assumptions",
                |raw| raw.proposal.assumptions[0] = " ".into(),
                "\"proposal.assumptions\"",
            ),
            (
                "proposal.unresolved",
                |raw| raw.proposal.unresolved[0] = " ".into(),
                "\"proposal.unresolved\"",
            ),
            (
                "validation.guidance",
                |raw| raw.validation.guidance[0] = " ".into(),
                "\"validation.guidance\"",
            ),
            (
                "validation.authority_notes",
                |raw| raw.validation.authority_notes[0] = " ".into(),
                "\"validation.authority_notes\"",
            ),
            (
                "execution.selected_capabilities",
                |raw| raw.execution.selected_capabilities[0] = " ".into(),
                "\"execution.selected_capabilities\"",
            ),
            (
                "execution.placement",
                |raw| raw.execution.placement[0] = " ".into(),
                "\"execution.placement\"",
            ),
            (
                "trace.artifacts",
                |raw| raw.trace.artifacts[0] = " ".into(),
                "\"trace.artifacts\"",
            ),
            (
                "trace.queries",
                |raw| raw.trace.queries[0] = " ".into(),
                "\"trace.queries\"",
            ),
        ];
        for (label, mutate, expected) in list_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let mut duplicate_surface = raw_scenario();
        duplicate_surface.surfaces[1].name = duplicate_surface.surfaces[0].name.clone();
        assert!(validate_scenario(duplicate_surface, "duplicate-surface")
            .unwrap_err()
            .contains("duplicate surface name"));
    }

    #[test]
    fn validate_scenario_rejects_surface_blank_fields() {
        let surface_cases: [(&str, fn(&mut RawScenario), &str); 7] = [
            ("surfaces[].name", |raw| raw.surfaces[0].name = " ".into(), "\"surfaces[].name\""),
            (
                "surfaces[].runtime",
                |raw| raw.surfaces[0].runtime = " ".into(),
                "\"surfaces[].runtime\"",
            ),
            (
                "surfaces[].scope",
                |raw| raw.surfaces[0].scope = " ".into(),
                "\"surfaces[].scope\"",
            ),
            (
                "surfaces[].authority",
                |raw| raw.surfaces[0].authority = " ".into(),
                "\"surfaces[].authority\"",
            ),
            (
                "surfaces[].visible_capabilities",
                |raw| raw.surfaces[0].visible_capabilities[0] = " ".into(),
                "\"surfaces[].visible_capabilities\"",
            ),
            (
                "surfaces[].hidden_constraints",
                |raw| raw.surfaces[0].hidden_constraints[0] = " ".into(),
                "\"surfaces[].hidden_constraints\"",
            ),
            (
                "surfaces[].queries",
                |raw| raw.surfaces[0].queries[0] = " ".into(),
                "\"surfaces[].queries\"",
            ),
        ];
        for (label, mutate, expected) in surface_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }
    }

    #[test]
    fn validate_scenario_rejects_late_axis_and_validation_fields() {
        let mut raw = raw_scenario();
        raw.axes.revision_model = " ".into();
        let error = validate_scenario(raw, "late-axis").unwrap_err();
        assert!(error.contains("\"axes.revision_model\""), "{error}");

        let mut raw = raw_scenario();
        raw.axes.execution_model = " ".into();
        let error = validate_scenario(raw, "late-axis").unwrap_err();
        assert!(error.contains("\"axes.execution_model\""), "{error}");

        let mut raw = raw_scenario();
        raw.validation.violations = vec![" ".into()];
        let error = validate_scenario(raw, "late-list").unwrap_err();
        assert!(error.contains("\"validation.violations\""), "{error}");
    }

    #[test]
    fn assess_covers_all_warning_and_verdict_paths() {
        let mut raw = raw_scenario();
        raw.axes.proposal_visibility = "hidden".into();
        raw.axes.authority_model = "local-only".into();
        raw.axes.revision_model = "unbounded".into();
        raw.axes.execution_model = "implicit-replan".into();
        raw.axes.traceability = "absent".into();
        raw.validation.status = "violations".into();
        raw.validation.guidance.clear();
        let report = validate_scenario(raw, "unstable").unwrap();
        let codes = report
            .assessment
            .warnings
            .iter()
            .map(|warning| warning.code.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(report.assessment.verdict, "unstable");
        assert!(codes.contains("proposal_hidden"));
        assert!(codes.contains("authority_gap"));
        assert!(codes.contains("validation_without_guidance"));
        assert!(codes.contains("unbounded_revision"));
        assert!(codes.contains("implicit_replanning"));
        assert!(codes.contains("missing_trace"));

        let mut raw = raw_scenario();
        raw.axes.proposal_visibility = "hidden".into();
        raw.axes.authority_model = "local-only".into();
        let report = validate_scenario(raw, "opaque").unwrap();
        assert_eq!(report.assessment.verdict, "opaque");

        let mut raw = raw_scenario();
        raw.axes.traceability = "proposal-only".into();
        let report = validate_scenario(raw, "discoverable").unwrap();
        assert_eq!(report.assessment.verdict, "discoverable");
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "partial_trace"));
    }

    #[test]
    fn list_labs_load_report_validate_all_and_unknown_paths_are_covered() {
        let root = temp_root();
        let error = list_labs(&root).unwrap_err();
        assert!(!error.is_empty());

        let error = load_report(&root, "missing").unwrap_err();
        assert!(!error.is_empty());

        let error = validate_all(&root).unwrap_err();
        assert!(!error.is_empty());

        write_file(
            &root.join("scenarios/b-lab/scenario.json"),
            include_str!("../../scenarios/lab1-capability-projection/scenario.json"),
        );
        write_file(
            &root.join("scenarios/a-lab/scenario.json"),
            include_str!("../../scenarios/lab6-queryable-trace/scenario.json"),
        );
        write_file(&root.join("scenarios/readme.txt"), "ignore");
        assert_eq!(list_labs(&root).unwrap(), vec!["a-lab", "b-lab"]);

        let error = load_report(&root, "missing").unwrap_err();
        assert!(error.contains("unknown lab"));

        let invalid_root = temp_root();
        write_file(&invalid_root.join("scenarios/lab1/scenario.json"), "{ invalid");
        let error = load_report(&invalid_root, "lab1").unwrap_err();
        assert!(!error.is_empty());

        let invalid_root = temp_root();
        write_file(
            &invalid_root.join("scenarios/lab1/scenario.json"),
            r#"{"title":"x"}"#,
        );
        let error = load_report(&invalid_root, "lab1").unwrap_err();
        assert!(!error.is_empty());

        let unreadable_root = temp_root();
        std::fs::create_dir_all(unreadable_root.join("scenarios/lab1/scenario.json")).unwrap();
        let error = load_report(&unreadable_root, "lab1").unwrap_err();
        assert!(!error.is_empty());

        let invalid_validate_all_root = temp_root();
        write_file(
            &invalid_validate_all_root.join("scenarios/lab1/scenario.json"),
            r#"{"title":"x"}"#,
        );
        let error = validate_all(&invalid_validate_all_root).unwrap_err();
        assert!(!error.is_empty());

        let empty_root = temp_root();
        std::fs::create_dir_all(empty_root.join("scenarios")).unwrap();
        let summaries = validate_all(&empty_root).unwrap();
        assert!(summaries.is_empty());

        let summaries = validate_all(&root).unwrap();
        assert_eq!(summaries.len(), 2);
        assert!(summaries[0].contains("Validated a-lab"));
        assert!(summaries[1].contains("Validated b-lab"));
    }

    #[test]
    fn format_report_and_diff_cover_empty_and_full_sections() {
        let governed = load_report(&project_root(), "lab6-queryable-trace").unwrap();
        let rendered = format_report(&governed);
        assert!(rendered.contains("Decision Axes:"));
        assert!(rendered.contains("Discoverable Surfaces:"));
        assert!(rendered.contains("Proposal:"));
        assert!(rendered.contains("Validation:"));
        assert!(rendered.contains("Execution:"));
        assert!(rendered.contains("Trace:"));
        assert!(rendered.contains("Runtime Decisions:"));
        assert!(rendered.contains("Reader Value:"));

        let hidden = load_report(&project_root(), "lab1-capability-projection").unwrap();
        let diff = diff_reports(&hidden, &governed);
        let rendered = format_diff(&diff);
        assert!(rendered.contains("Changed Axes:"));
        assert!(rendered.contains("Changed Stages:"));
        assert!(rendered.contains("Added Warnings:"));
        assert!(rendered.contains("Removed Warnings:"));

        let empty = ScenarioDiff {
            from: "a".into(),
            to: "b".into(),
            verdict_change: "governed -> governed".into(),
            added_warnings: Vec::new(),
            removed_warnings: Vec::new(),
            changed_axes: Vec::new(),
            changed_stages: Vec::new(),
        };
        let rendered = format_diff(&empty);
        assert!(rendered.contains("Changed Axes: none"));
        assert!(rendered.contains("Changed Stages: none"));
        assert!(rendered.contains("Added Warnings: none"));
        assert!(rendered.contains("Removed Warnings: none"));

        let mut empty_report = governed.clone();
        empty_report.assessment.warnings.clear();
        empty_report.validation.violations.clear();
        empty_report.validation.guidance.clear();
        empty_report.validation.authority_notes.clear();
        empty_report.execution.selected_capabilities.clear();
        empty_report.execution.placement.clear();
        empty_report.trace.artifacts.clear();
        empty_report.trace.queries.clear();
        let rendered = format_report(&empty_report);
        assert!(rendered.contains("Warnings: none"));
        assert!(rendered.contains("- violations: none"));
        assert!(rendered.contains("- guidance: none"));
        assert!(rendered.contains("- authority notes: none"));
        assert!(rendered.contains("- selected capabilities: none"));
        assert!(rendered.contains("- placement: none"));
        assert!(rendered.contains("- artifacts: none"));
        assert!(rendered.contains("- queries: none"));
    }

    #[test]
    fn format_report_renders_non_empty_validation_lists() {
        let mut report = load_report(&project_root(), "lab6-queryable-trace").unwrap();
        report.validation.violations = vec![
            "authority note missing".into(),
            "trace contract incomplete".into(),
        ];
        report.validation.guidance = vec![
            "add runtime authority notes".into(),
            "persist the decision log".into(),
        ];

        let rendered = format_report(&report);
        assert!(rendered.contains(
            "- violations: authority note missing | trace contract incomplete"
        ));
        assert!(rendered.contains(
            "- guidance: add runtime authority notes | persist the decision log"
        ));
    }

    #[test]
    fn format_diff_renders_added_warnings_branch() {
        let diff = ScenarioDiff {
            from: "proposal-only".into(),
            to: "discoverable".into(),
            verdict_change: "opaque -> discoverable".into(),
            added_warnings: vec!["missing_decision_log".into()],
            removed_warnings: vec!["hidden_capability_projection".into()],
            changed_axes: vec!["traceability: proposal-only -> discoverable".into()],
            changed_stages: vec!["approved_execution".into()],
        };
        let rendered = format_diff(&diff);

        assert!(rendered.contains("Added Warnings: missing_decision_log"));
        assert!(rendered.contains("Removed Warnings: hidden_capability_projection"));
    }

    #[test]
    fn diff_reports_captures_changed_stage_transitions() {
        let from = validate_scenario(raw_scenario(), "opaque").unwrap();
        let mut to = from.clone();
        to.proposal.status = "revised".into();
        to.execution.status = "approved".into();
        to.trace.status = "discoverable".into();

        let diff = diff_reports(&from, &to);
        assert!(!diff.changed_stages.is_empty());
        assert!(diff
            .changed_stages
            .iter()
            .any(|item| item.starts_with("proposal status: ")));
        assert!(diff
            .changed_stages
            .iter()
            .any(|item| item.starts_with("execution status: ")));
        assert!(diff
            .changed_stages
            .iter()
            .any(|item| item.starts_with("trace status: ")));
    }
}
