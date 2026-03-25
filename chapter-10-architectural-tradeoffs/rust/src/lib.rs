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

fn resolve_project_root(cwd: Option<PathBuf>, manifest_dir: PathBuf) -> PathBuf {
    match cwd {
        Some(cwd) if cwd.join("scenarios").exists() => cwd,
        _ => match manifest_dir.parent() {
            Some(parent) if parent.join("scenarios").exists() => parent.to_path_buf(),
            _ => manifest_dir,
        },
    }
}

pub fn project_root() -> PathBuf {
    resolve_project_root(
        std::env::current_dir().ok(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
}

pub fn list_labs(root_dir: &Path) -> Result<Vec<String>, String> {
    let entries = match fs::read_dir(root_dir.join("scenarios")) {
        Ok(entries) => entries,
        Err(err) => return Err(err.to_string()),
    };
    let mut names = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
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
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => return Err(err.to_string()),
    };
    let raw: RawScenario = match serde_json::from_str(&contents) {
        Ok(raw) => raw,
        Err(err) => return Err(err.to_string()),
    };
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
    use std::time::{SystemTime, UNIX_EPOCH};

    fn raw_scenario() -> RawScenario {
        RawScenario {
            title: "Test scenario".into(),
            summary: "A coherent test scenario.".into(),
            services: vec![
                RawService {
                    id: "b-service".into(),
                    capability: "media.index".into(),
                    summary: "Indexes analysis.".into(),
                    placements: vec!["cloud".into()],
                    events: RawEvents {
                        consumes: vec!["image.analyzed".into()],
                        emits: vec!["metadata.indexed".into()],
                    },
                    metadata: MetadataProfile {
                        fields: vec!["capability".into(), "placements".into()],
                        selection_rank: Some(20),
                    },
                },
                RawService {
                    id: "a-service".into(),
                    capability: "media.analyze".into(),
                    summary: "Analyzes uploads.".into(),
                    placements: vec!["edge".into(), "cloud".into()],
                    events: RawEvents {
                        consumes: vec!["image.uploaded".into()],
                        emits: vec!["image.analyzed".into()],
                    },
                    metadata: MetadataProfile {
                        fields: vec!["capability".into(), "latencyTargetMs".into()],
                        selection_rank: Some(10),
                    },
                },
            ],
            interactions: vec![
                RawInteraction {
                    from: "b-service".into(),
                    event: "metadata.indexed".into(),
                    to: "client".into(),
                },
                RawInteraction {
                    from: "a-service".into(),
                    event: "image.analyzed".into(),
                    to: "b-service".into(),
                },
            ],
            choices: RawChoices {
                granularity: "capability-aligned".into(),
                event_semantics: "stable-domain-facts".into(),
                metadata_quality: "concise".into(),
                orchestration: "distributed-event-progression".into(),
                runtime_placement: "constraint-driven".into(),
                state_model: "local-projections".into(),
            },
            expectations: vec![
                "Expectation one".into(),
                "Expectation two".into(),
            ],
        }
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("chapter10-{prefix}-{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_scenario(root: &Path, lab: &str, raw: &RawScenario) {
        let path = root.join("scenarios").join(lab);
        fs::create_dir_all(&path).unwrap();
        let services = raw
            .services
            .iter()
            .map(|service| {
                serde_json::json!({
                    "id": service.id,
                    "capability": service.capability,
                    "summary": service.summary,
                    "placements": service.placements,
                    "events": {
                        "consumes": service.events.consumes,
                        "emits": service.events.emits,
                    },
                    "metadata": {
                        "fields": service.metadata.fields,
                        "selection_rank": service.metadata.selection_rank,
                    }
                })
            })
            .collect::<Vec<_>>();
        let interactions = raw
            .interactions
            .iter()
            .map(|interaction| {
                serde_json::json!({
                    "from": interaction.from,
                    "event": interaction.event,
                    "to": interaction.to,
                })
            })
            .collect::<Vec<_>>();
        fs::write(
            path.join("scenario.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "title": raw.title,
                "summary": raw.summary,
                "services": services,
                "interactions": interactions,
                "choices": {
                    "granularity": raw.choices.granularity,
                    "event_semantics": raw.choices.event_semantics,
                    "metadata_quality": raw.choices.metadata_quality,
                    "orchestration": raw.choices.orchestration,
                    "runtime_placement": raw.choices.runtime_placement,
                    "state_model": raw.choices.state_model,
                },
                "expectations": raw.expectations,
            }))
            .unwrap(),
        )
        .unwrap();
    }

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

    #[test]
    fn validate_scenario_sorts_and_normalizes_entities() {
        let report = validate_scenario(raw_scenario(), "test-lab").unwrap();
        assert_eq!(report.services[0].id, "a-service");
        assert_eq!(report.services[1].id, "b-service");
        assert_eq!(report.interactions[0].from, "a-service");
        assert_eq!(report.interactions[1].from, "b-service");
    }

    #[test]
    fn validate_scenario_rejects_empty_strings_and_duplicate_service_ids() {
        let mut empty_title = raw_scenario();
        empty_title.title = "   ".into();
        let error = validate_scenario(empty_title, "empty-title").unwrap_err();
        assert!(error.contains("\"title\" must be a non-empty string"));

        let mut empty_expectation = raw_scenario();
        empty_expectation.expectations.push(" ".into());
        let error = validate_scenario(empty_expectation, "empty-expectation").unwrap_err();
        assert!(error.contains("\"expectations\" must be a non-empty string"));

        let mut duplicate_ids = raw_scenario();
        duplicate_ids.services[1].id = duplicate_ids.services[0].id.clone();
        let error = validate_scenario(duplicate_ids, "duplicate-ids").unwrap_err();
        assert!(error.contains("duplicate service id"));
    }

    #[test]
    fn validate_scenario_covers_remaining_field_error_paths() {
        let scenario_cases: [(&str, fn(&mut RawScenario), &str); 7] = [
            ("summary", |raw| raw.summary = " ".into(), "\"summary\""),
            (
                "choices.granularity",
                |raw| raw.choices.granularity = " ".into(),
                "\"choices.granularity\"",
            ),
            (
                "choices.event_semantics",
                |raw| raw.choices.event_semantics = " ".into(),
                "\"choices.event_semantics\"",
            ),
            (
                "choices.metadata_quality",
                |raw| raw.choices.metadata_quality = " ".into(),
                "\"choices.metadata_quality\"",
            ),
            (
                "choices.orchestration",
                |raw| raw.choices.orchestration = " ".into(),
                "\"choices.orchestration\"",
            ),
            (
                "choices.runtime_placement",
                |raw| raw.choices.runtime_placement = " ".into(),
                "\"choices.runtime_placement\"",
            ),
            (
                "choices.state_model",
                |raw| raw.choices.state_model = " ".into(),
                "\"choices.state_model\"",
            ),
        ];
        for (label, mutate, expected) in scenario_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let service_cases: [(&str, fn(&mut RawScenario), &str); 7] = [
            (
                "services[].id",
                |raw| raw.services[0].id = " ".into(),
                "\"services[].id\"",
            ),
            (
                "services[].capability",
                |raw| raw.services[0].capability = " ".into(),
                "\"services[].capability\"",
            ),
            (
                "services[].summary",
                |raw| raw.services[0].summary = " ".into(),
                "\"services[].summary\"",
            ),
            (
                "services[].placements",
                |raw| raw.services[0].placements[0] = " ".into(),
                "\"services[].placements\"",
            ),
            (
                "services[].events.consumes",
                |raw| raw.services[0].events.consumes = vec![" ".into()],
                "\"services[].events.consumes\"",
            ),
            (
                "services[].events.emits",
                |raw| raw.services[0].events.emits[0] = " ".into(),
                "\"services[].events.emits\"",
            ),
            (
                "services[].metadata.fields",
                |raw| raw.services[0].metadata.fields[0] = " ".into(),
                "\"services[].metadata.fields\"",
            ),
        ];
        for (label, mutate, expected) in service_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let interaction_cases: [(&str, fn(&mut RawScenario), &str); 3] = [
            (
                "interactions[].from",
                |raw| raw.interactions[0].from = " ".into(),
                "\"interactions[].from\"",
            ),
            (
                "interactions[].event",
                |raw| raw.interactions[0].event = " ".into(),
                "\"interactions[].event\"",
            ),
            (
                "interactions[].to",
                |raw| raw.interactions[0].to = " ".into(),
                "\"interactions[].to\"",
            ),
        ];
        for (label, mutate, expected) in interaction_cases {
            let mut raw = raw_scenario();
            mutate(&mut raw);
            let error = validate_scenario(raw, label).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }
    }

    #[test]
    fn infer_over_granular_detects_staged_pipeline_without_explicit_flag() {
        let services = (0..5)
            .map(|index| Service {
                id: format!("service-{index}"),
                capability: format!("capability-{index}"),
                summary: "step".into(),
                placements: vec!["cloud".into()],
                consumes: Vec::new(),
                emits: Vec::new(),
                metadata: MetadataProfile {
                    fields: vec!["capability".into()],
                    selection_rank: Some(index),
                },
            })
            .collect::<Vec<_>>();
        let interactions = (0..4)
            .map(|index| Interaction {
                from: format!("service-{index}"),
                event: format!("event-{index}"),
                to: format!("service-{}", index + 1),
            })
            .collect::<Vec<_>>();
        let choices = Choices {
            granularity: "capability-aligned".into(),
            event_semantics: "stable-domain-facts".into(),
            metadata_quality: "concise".into(),
            orchestration: "distributed-event-progression".into(),
            runtime_placement: "constraint-driven".into(),
            state_model: "local-projections".into(),
        };

        assert!(infer_over_granular(&services, &interactions, &choices));
    }

    #[test]
    fn assess_covers_all_warning_and_verdict_paths() {
        let mut report = validate_scenario(raw_scenario(), "assessment").unwrap();
        report.choices.event_semantics = "ambiguous".into();
        report.choices.metadata_quality = "bloated".into();
        report.choices.orchestration = "rigid-centralized".into();
        report.choices.runtime_placement = "ambiguous-selection".into();
        report.choices.state_model = "duplicated-projections".into();

        let assessment = assess(&report.services, &report.interactions, &report.choices);
        let codes = assessment
            .warnings
            .iter()
            .map(|warning| warning.code.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(assessment.verdict, "fragile");
        assert!(codes.contains("hidden_event_coupling"));
        assert!(codes.contains("metadata_bloat"));
        assert!(codes.contains("over_orchestrated"));
        assert!(codes.contains("runtime_ambiguity"));
        assert!(codes.contains("state_drift"));

        let ambiguous_choices = Choices {
            granularity: "capability-aligned".into(),
            event_semantics: "stable-domain-facts".into(),
            metadata_quality: "concise".into(),
            orchestration: "distributed-event-progression".into(),
            runtime_placement: "ambiguous-selection".into(),
            state_model: "local-projections".into(),
        };
        let ambiguous = assess(&report.services, &report.interactions, &ambiguous_choices);
        assert_eq!(ambiguous.verdict, "ambiguous");

        let needs_attention_choices = Choices {
            granularity: "capability-aligned".into(),
            event_semantics: "ambiguous".into(),
            metadata_quality: "concise".into(),
            orchestration: "distributed-event-progression".into(),
            runtime_placement: "constraint-driven".into(),
            state_model: "local-projections".into(),
        };
        let needs_attention = assess(&report.services, &report.interactions, &needs_attention_choices);
        assert_eq!(needs_attention.verdict, "needs-attention");
    }

    #[test]
    fn resolve_project_root_covers_all_branches() {
        let cwd_root = unique_temp_dir("cwd-root");
        fs::create_dir_all(cwd_root.join("scenarios")).unwrap();
        assert_eq!(
            resolve_project_root(Some(cwd_root.clone()), PathBuf::from("/unused/manifest")),
            cwd_root
        );

        let manifest_parent = unique_temp_dir("manifest-parent");
        let manifest_dir = manifest_parent.join("rust");
        fs::create_dir_all(manifest_parent.join("scenarios")).unwrap();
        fs::create_dir_all(&manifest_dir).unwrap();
        assert_eq!(
            resolve_project_root(None, manifest_dir.clone()),
            manifest_parent
        );

        let fallback_base = unique_temp_dir("fallback-manifest");
        let fallback_manifest = fallback_base.join("nested").join("manifest");
        fs::create_dir_all(&fallback_manifest).unwrap();
        assert_eq!(
            resolve_project_root(None, fallback_manifest.clone()),
            fallback_manifest
        );

        let relative_manifest = PathBuf::from("manifest-without-parent");
        assert_eq!(
            resolve_project_root(None, relative_manifest.clone()),
            relative_manifest
        );
    }

    #[test]
    fn list_labs_sorts_directories_and_ignores_files() {
        let root = unique_temp_dir("list-labs");
        fs::create_dir_all(root.join("scenarios/z-lab")).unwrap();
        fs::create_dir_all(root.join("scenarios/a-lab")).unwrap();
        fs::write(root.join("scenarios/README.txt"), "ignore").unwrap();

        let labs = list_labs(&root).unwrap();
        assert_eq!(labs, vec!["a-lab".to_string(), "z-lab".to_string()]);
    }

    #[test]
    fn list_labs_errors_when_scenarios_dir_is_missing() {
        let root = unique_temp_dir("missing-scenarios");
        let error = list_labs(&root).unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn load_report_rejects_invalid_json() {
        let root = unique_temp_dir("invalid-json");
        let lab_path = root.join("scenarios").join("broken-lab");
        fs::create_dir_all(&lab_path).unwrap();
        fs::write(lab_path.join("scenario.json"), "{ not valid json").unwrap();

        let error = load_report(&root, "broken-lab").unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn load_report_errors_when_scenario_file_is_missing() {
        let root = unique_temp_dir("missing-scenario-file");
        fs::create_dir_all(root.join("scenarios").join("lab-without-file")).unwrap();

        let error = load_report(&root, "lab-without-file").unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn load_report_errors_when_scenarios_directory_is_missing() {
        let root = unique_temp_dir("missing-scenarios-load-report");
        let error = load_report(&root, "any-lab").unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn format_report_covers_populated_and_empty_sections() {
        let populated = load_report(&project_root(), "lab5-over-orchestrated").unwrap();
        let rendered = format_report(&populated);
        assert!(rendered.contains("Scenario: lab5-over-orchestrated"));
        assert!(rendered.contains("Architectural Choices"));
        assert!(rendered.contains("Services"));
        assert!(rendered.contains("consumes:"));
        assert!(rendered.contains("emits:"));
        assert!(rendered.contains("Interaction Flow"));
        assert!(rendered.contains("Warnings"));
        assert!(rendered.contains("Reader Value"));

        let empty = ScenarioReport {
            scenario: "empty".into(),
            title: "Empty".into(),
            summary: "Minimal".into(),
            services: vec![Service {
                id: "single".into(),
                capability: "capability".into(),
                summary: "summary".into(),
                placements: vec!["cloud".into()],
                consumes: Vec::new(),
                emits: Vec::new(),
                metadata: MetadataProfile {
                    fields: vec!["capability".into()],
                    selection_rank: Some(1),
                },
            }],
            interactions: Vec::new(),
            choices: Choices {
                granularity: "capability-aligned".into(),
                event_semantics: "stable-domain-facts".into(),
                metadata_quality: "concise".into(),
                orchestration: "distributed-event-progression".into(),
                runtime_placement: "constraint-driven".into(),
                state_model: "local-projections".into(),
            },
            expectations: vec!["One value".into()],
            assessment: Assessment {
                verdict: "coherent".into(),
                warnings: Vec::new(),
            },
        };
        let rendered = format_report(&empty);
        assert!(rendered.contains("Interaction Flow\n- none"));
        assert!(rendered.contains("Warnings\n- none"));
    }

    #[test]
    fn format_diff_covers_changed_and_empty_sections() {
        let from = load_report(&project_root(), "lab1-baseline").unwrap();
        let to = load_report(&project_root(), "lab5-over-orchestrated").unwrap();
        let changed = format_diff(&diff_reports(&from, &to));
        assert!(changed.contains("Architecture diff: lab1-baseline -> lab5-over-orchestrated"));
        assert!(changed.contains("Changed axes"));
        assert!(changed.contains("Added warnings"));
        assert!(changed.contains("Removed warnings"));

        let removed = format_diff(&diff_reports(&to, &from));
        assert!(removed.contains("Removed warnings\n- metadata_bloat"));

        let empty = ScenarioDiff {
            from: "a".into(),
            to: "b".into(),
            verdict_change: "coherent -> coherent".into(),
            added_warnings: Vec::new(),
            removed_warnings: Vec::new(),
            changed_axes: Vec::new(),
        };
        let rendered = format_diff(&empty);
        assert!(rendered.contains("Changed axes\n- none"));
        assert!(rendered.contains("Added warnings\n- none"));
        assert!(rendered.contains("Removed warnings\n- none"));
    }

    #[test]
    fn validate_all_summarizes_each_lab() {
        let root = unique_temp_dir("validate-all");
        write_scenario(&root, "b-lab", &raw_scenario());
        let mut second = raw_scenario();
        second.title = "Second scenario".into();
        second.choices.event_semantics = "ambiguous".into();
        write_scenario(&root, "a-lab", &second);

        let summaries = validate_all(&root).unwrap();
        assert_eq!(summaries.len(), 2);
        assert!(summaries[0].contains("Validated a-lab"));
        assert!(summaries[1].contains("Validated b-lab"));
    }

    #[test]
    fn validate_all_surfaces_listing_and_load_errors() {
        let missing_root = unique_temp_dir("validate-all-missing");
        let error = validate_all(&missing_root).unwrap_err();
        assert!(!error.is_empty());

        let broken_root = unique_temp_dir("validate-all-broken");
        fs::create_dir_all(broken_root.join("scenarios").join("broken-lab")).unwrap();
        let error = validate_all(&broken_root).unwrap_err();
        assert!(!error.is_empty());
    }
}
