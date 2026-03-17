use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawScenario {
    title: String,
    summary: String,
    services: Vec<RawService>,
    interactions: Vec<RawInteraction>,
    choices: RawChoices,
    expectations: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawService {
    id: String,
    capability: String,
    summary: String,
    placements: Vec<String>,
    events: RawEvents,
    #[serde(default)]
    metadata: MetadataProfile,
}

#[derive(Debug, Deserialize)]
struct RawEvents {
    #[serde(default)]
    consumes: Vec<String>,
    #[serde(default)]
    emits: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct MetadataProfile {
    #[serde(default)]
    fields: Vec<String>,
    #[serde(default)]
    selection_rank: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct RawInteraction {
    from: String,
    event: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct RawChoices {
    granularity: String,
    event_semantics: String,
    metadata_quality: String,
    orchestration: String,
    runtime_placement: String,
    state_model: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Service {
    pub id: String,
    pub capability: String,
    pub summary: String,
    pub placements: Vec<String>,
    pub consumes: Vec<String>,
    pub emits: Vec<String>,
    pub metadata: MetadataProfile,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Interaction {
    pub from: String,
    pub event: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Choices {
    pub granularity: String,
    #[serde(rename = "eventSemantics")]
    pub event_semantics: String,
    #[serde(rename = "metadataQuality")]
    pub metadata_quality: String,
    pub orchestration: String,
    #[serde(rename = "runtimePlacement")]
    pub runtime_placement: String,
    #[serde(rename = "stateModel")]
    pub state_model: String,
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
    pub services: Vec<Service>,
    pub interactions: Vec<Interaction>,
    pub choices: Choices,
    pub expectations: Vec<String>,
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
    ensure_non_empty(&raw.choices.granularity, "choices.granularity", label)?;
    ensure_non_empty(&raw.choices.event_semantics, "choices.event_semantics", label)?;
    ensure_non_empty(&raw.choices.metadata_quality, "choices.metadata_quality", label)?;
    ensure_non_empty(&raw.choices.orchestration, "choices.orchestration", label)?;
    ensure_non_empty(&raw.choices.runtime_placement, "choices.runtime_placement", label)?;
    ensure_non_empty(&raw.choices.state_model, "choices.state_model", label)?;
    ensure_list(&raw.expectations, "expectations", label)?;

    let mut services = Vec::new();
    let mut ids = BTreeSet::new();
    for service in raw.services {
        ensure_non_empty(&service.id, "services[].id", label)?;
        ensure_non_empty(&service.capability, "services[].capability", label)?;
        ensure_non_empty(&service.summary, "services[].summary", label)?;
        ensure_list(&service.placements, "services[].placements", label)?;
        ensure_list(&service.events.consumes, "services[].events.consumes", label)?;
        ensure_list(&service.events.emits, "services[].events.emits", label)?;
        ensure_list(&service.metadata.fields, "services[].metadata.fields", label)?;
        if !ids.insert(service.id.clone()) {
            return Err(format!("{label}: duplicate service id \"{}\"", service.id));
        }
        services.push(Service {
            id: service.id,
            capability: service.capability,
            summary: service.summary,
            placements: service.placements,
            consumes: service.events.consumes,
            emits: service.events.emits,
            metadata: service.metadata,
        });
    }
    services.sort_by(|a, b| a.id.cmp(&b.id));

    let mut interactions = Vec::new();
    for interaction in raw.interactions {
        ensure_non_empty(&interaction.from, "interactions[].from", label)?;
        ensure_non_empty(&interaction.event, "interactions[].event", label)?;
        ensure_non_empty(&interaction.to, "interactions[].to", label)?;
        interactions.push(Interaction {
            from: interaction.from,
            event: interaction.event,
            to: interaction.to,
        });
    }
    interactions.sort_by(|a, b| {
        a.from
            .cmp(&b.from)
            .then(a.event.cmp(&b.event))
            .then(a.to.cmp(&b.to))
    });

    let choices = Choices {
        granularity: raw.choices.granularity,
        event_semantics: raw.choices.event_semantics,
        metadata_quality: raw.choices.metadata_quality,
        orchestration: raw.choices.orchestration,
        runtime_placement: raw.choices.runtime_placement,
        state_model: raw.choices.state_model,
    };
    let assessment = assess(&services, &interactions, &choices);

    Ok(ScenarioReport {
        scenario: label.to_string(),
        title: raw.title,
        summary: raw.summary,
        services,
        interactions,
        choices,
        expectations: raw.expectations,
        assessment,
    })
}

fn infer_over_granular(services: &[Service], interactions: &[Interaction], choices: &Choices) -> bool {
    if choices.granularity == "over-granular" {
        return true;
    }

    let staged_pipeline = interactions.len() >= 4
        && services.len() >= 5
        && choices.orchestration != "rigid-centralized";

    staged_pipeline
}

fn assess(services: &[Service], interactions: &[Interaction], choices: &Choices) -> Assessment {
    let mut warnings = Vec::new();

    if infer_over_granular(services, interactions, choices) {
        warnings.push(Warning {
            code: "over_granular".into(),
            message: "The workflow is split across too many narrowly scoped capabilities.".into(),
        });
    }

    if choices.event_semantics == "ambiguous" {
        warnings.push(Warning {
            code: "hidden_event_coupling".into(),
            message: "Events are too vague and create hidden coupling between consumers.".into(),
        });
    }

    if choices.metadata_quality == "bloated" {
        warnings.push(Warning {
            code: "metadata_bloat".into(),
            message: "Metadata contains too much detail and loses clarity as a control plane.".into(),
        });
    }

    if choices.orchestration == "rigid-centralized" {
        warnings.push(Warning {
            code: "over_orchestrated".into(),
            message: "A central coordinator is dictating too much workflow behavior.".into(),
        });
    }

    if choices.runtime_placement == "ambiguous-selection"
        || services
            .iter()
            .filter(|service| service.metadata.selection_rank.is_none())
            .count()
            >= 2
    {
        warnings.push(Warning {
            code: "runtime_ambiguity".into(),
            message: "Multiple capabilities can satisfy the same role without deterministic selection.".into(),
        });
    }

    if choices.state_model == "duplicated-projections" {
        warnings.push(Warning {
            code: "state_drift".into(),
            message: "State is being duplicated without a clear capability boundary or projection purpose.".into(),
        });
    }

    let verdict = if warnings.is_empty() {
        "coherent"
    } else if warnings.iter().any(|warning| warning.code == "over_orchestrated") {
        "fragile"
    } else if warnings.iter().any(|warning| warning.code == "runtime_ambiguity") {
        "ambiguous"
    } else {
        "needs-attention"
    };

    Assessment {
        verdict: verdict.to_string(),
        warnings,
    }
}

pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn list_labs(root_dir: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    for entry in fs::read_dir(root_dir.join("scenarios")).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if entry.file_type().map_err(|err| err.to_string())?.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

pub fn load_report(root_dir: &Path, lab: &str) -> Result<ScenarioReport, String> {
    let labs = list_labs(root_dir)?;
    if !labs.iter().any(|name| name == lab) {
        return Err(format!(
            "unknown lab \"{lab}\". Available labs: {}",
            labs.join(", ")
        ));
    }
    let path = root_dir.join("scenarios").join(lab).join("scenario.json");
    let raw: RawScenario =
        serde_json::from_str(&fs::read_to_string(path).map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?;
    validate_scenario(raw, lab)
}

pub fn format_report(report: &ScenarioReport) -> String {
    let mut out = String::new();
    writeln!(&mut out, "Scenario: {}", report.scenario).unwrap();
    writeln!(&mut out, "Title: {}", report.title).unwrap();
    writeln!(&mut out, "Summary: {}", report.summary).unwrap();
    writeln!(&mut out, "Verdict: {}", report.assessment.verdict).unwrap();
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Architectural Choices").unwrap();
    writeln!(&mut out, "- granularity: {}", report.choices.granularity).unwrap();
    writeln!(
        &mut out,
        "- event semantics: {}",
        report.choices.event_semantics
    )
    .unwrap();
    writeln!(
        &mut out,
        "- metadata quality: {}",
        report.choices.metadata_quality
    )
    .unwrap();
    writeln!(&mut out, "- orchestration: {}", report.choices.orchestration).unwrap();
    writeln!(
        &mut out,
        "- runtime placement: {}",
        report.choices.runtime_placement
    )
    .unwrap();
    writeln!(&mut out, "- state model: {}", report.choices.state_model).unwrap();
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Services").unwrap();
    for service in &report.services {
        writeln!(&mut out, "- {} ({})", service.id, service.capability).unwrap();
        writeln!(&mut out, "  summary: {}", service.summary).unwrap();
        writeln!(&mut out, "  placements: {}", service.placements.join(", ")).unwrap();
        if !service.consumes.is_empty() {
            writeln!(&mut out, "  consumes: {}", service.consumes.join(", ")).unwrap();
        }
        if !service.emits.is_empty() {
            writeln!(&mut out, "  emits: {}", service.emits.join(", ")).unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Interaction Flow").unwrap();
    if report.interactions.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for interaction in &report.interactions {
            writeln!(
                &mut out,
                "- {} -> {} -> {}",
                interaction.from, interaction.event, interaction.to
            )
            .unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Warnings").unwrap();
    if report.assessment.warnings.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for warning in &report.assessment.warnings {
            writeln!(&mut out, "- {}: {}", warning.code, warning.message).unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Reader Value").unwrap();
    for value in &report.expectations {
        writeln!(&mut out, "- {}", value).unwrap();
    }

    out
}

pub fn diff_reports(from: &ScenarioReport, to: &ScenarioReport) -> ScenarioDiff {
    let left: BTreeSet<String> = from
        .assessment
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect();
    let right: BTreeSet<String> = to
        .assessment
        .warnings
        .iter()
        .map(|warning| warning.code.clone())
        .collect();
    let mut changed_axes = Vec::new();

    let axes = [
        (
            "granularity",
            &from.choices.granularity,
            &to.choices.granularity,
        ),
        (
            "event semantics",
            &from.choices.event_semantics,
            &to.choices.event_semantics,
        ),
        (
            "metadata quality",
            &from.choices.metadata_quality,
            &to.choices.metadata_quality,
        ),
        (
            "orchestration",
            &from.choices.orchestration,
            &to.choices.orchestration,
        ),
        (
            "runtime placement",
            &from.choices.runtime_placement,
            &to.choices.runtime_placement,
        ),
        ("state model", &from.choices.state_model, &to.choices.state_model),
    ];

    for (label, left_value, right_value) in axes {
        if left_value != right_value {
            changed_axes.push(format!("{label}: {left_value} -> {right_value}"));
        }
    }

    ScenarioDiff {
        from: from.scenario.clone(),
        to: to.scenario.clone(),
        verdict_change: format!(
            "{} -> {}",
            from.assessment.verdict, to.assessment.verdict
        ),
        added_warnings: right.difference(&left).cloned().collect(),
        removed_warnings: left.difference(&right).cloned().collect(),
        changed_axes,
    }
}

pub fn format_diff(diff: &ScenarioDiff) -> String {
    let mut out = String::new();
    writeln!(&mut out, "Architecture diff: {} -> {}", diff.from, diff.to).unwrap();
    writeln!(&mut out, "Verdict: {}", diff.verdict_change).unwrap();
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Changed axes").unwrap();
    if diff.changed_axes.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for axis in &diff.changed_axes {
            writeln!(&mut out, "- {}", axis).unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Added warnings").unwrap();
    if diff.added_warnings.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for warning in &diff.added_warnings {
            writeln!(&mut out, "- {}", warning).unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Removed warnings").unwrap();
    if diff.removed_warnings.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for warning in &diff.removed_warnings {
            writeln!(&mut out, "- {}", warning).unwrap();
        }
    }

    out
}

pub fn validate_all(root_dir: &Path) -> Result<Vec<String>, String> {
    let labs = list_labs(root_dir)?;
    let mut summaries = Vec::new();
    for lab in labs {
        let report = load_report(root_dir, &lab)?;
        summaries.push(format!(
            "Validated {}: {} services, {} interactions, verdict={}",
            report.scenario,
            report.services.len(),
            report.interactions.len(),
            report.assessment.verdict
        ));
    }
    Ok(summaries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_is_coherent() {
        let report = load_report(&project_root(), "lab1-baseline").unwrap();
        assert_eq!(report.assessment.verdict, "coherent");
        assert!(report.assessment.warnings.is_empty());
    }

    #[test]
    fn over_granular_lab_warns() {
        let report = load_report(&project_root(), "lab2-over-granular").unwrap();
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "over_granular"));
    }

    #[test]
    fn ambiguity_lab_has_runtime_warning() {
        let report = load_report(&project_root(), "lab4-runtime-ambiguity").unwrap();
        assert_eq!(report.assessment.verdict, "ambiguous");
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "runtime_ambiguity"));
    }

    #[test]
    fn diff_detects_changed_axes() {
        let from = load_report(&project_root(), "lab1-baseline").unwrap();
        let to = load_report(&project_root(), "lab5-over-orchestrated").unwrap();
        let diff = diff_reports(&from, &to);
        assert!(diff
            .changed_axes
            .iter()
            .any(|item| item.contains("orchestration")));
        assert!(diff.added_warnings.iter().any(|warning| warning == "over_orchestrated"));
    }

    #[test]
    fn over_orchestrated_lab_stays_focused_on_orchestration() {
        let report = load_report(&project_root(), "lab5-over-orchestrated").unwrap();
        let warning_codes: BTreeSet<&str> = report
            .assessment
            .warnings
            .iter()
            .map(|warning| warning.code.as_str())
            .collect();

        assert!(warning_codes.contains("over_orchestrated"));
        assert!(warning_codes.contains("metadata_bloat"));
        assert!(warning_codes.contains("state_drift"));
        assert!(!warning_codes.contains("over_granular"));
    }

    #[test]
    fn unknown_lab_error_lists_available_options() {
        let error = load_report(&project_root(), "does-not-exist").unwrap_err();
        assert!(error.contains("unknown lab"));
        assert!(error.contains("lab1-baseline"));
    }
}
