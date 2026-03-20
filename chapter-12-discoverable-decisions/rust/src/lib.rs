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
    let mut labs = fs::read_dir(root.join("scenarios"))
        .map_err(|err| err.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
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
    let raw = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    let scenario = serde_json::from_str::<RawScenario>(&raw).map_err(|err| err.to_string())?;
    validate_scenario(scenario, lab)
}

pub fn validate_all(root: &Path) -> Result<Vec<String>, String> {
    let mut summaries = Vec::new();
    for lab in list_labs(root)? {
        let report = load_report(root, &lab)?;
        summaries.push(format!(
            "Validated {}: {} surfaces, verdict={}",
            report.scenario,
            report.surfaces.len(),
            report.assessment.verdict
        ));
    }
    Ok(summaries)
}

pub fn diff_reports(from: &ScenarioReport, to: &ScenarioReport) -> ScenarioDiff {
    let left = from
        .assessment
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect::<BTreeSet<_>>();
    let right = to
        .assessment
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect::<BTreeSet<_>>();

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
}
