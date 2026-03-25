use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct RawScenario {
    title: String,
    summary: String,
    services: Vec<RawService>,
    interactions: Vec<RawInteraction>,
    choices: RawChoices,
    expectations: Vec<String>,
    #[serde(default)]
    runtime_decisions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawService {
    id: String,
    capability: String,
    version: String,
    summary: String,
    placements: Vec<String>,
    #[serde(default)]
    consumers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawInteraction {
    from: String,
    to: String,
    relation: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawChoices {
    contract_anchor: String,
    versioning: String,
    runtime_governance: String,
    duplication: String,
    event_semantics: String,
    adoption_mode: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Service {
    pub id: String,
    pub capability: String,
    pub version: String,
    pub summary: String,
    pub placements: Vec<String>,
    pub consumers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Interaction {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Choices {
    #[serde(rename = "contractAnchor")]
    pub contract_anchor: String,
    pub versioning: String,
    #[serde(rename = "runtimeGovernance")]
    pub runtime_governance: String,
    pub duplication: String,
    #[serde(rename = "eventSemantics")]
    pub event_semantics: String,
    #[serde(rename = "adoptionMode")]
    pub adoption_mode: String,
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
    ensure_non_empty(&raw.choices.contract_anchor, "choices.contract_anchor", label)?;
    ensure_non_empty(&raw.choices.versioning, "choices.versioning", label)?;
    ensure_non_empty(
        &raw.choices.runtime_governance,
        "choices.runtime_governance",
        label,
    )?;
    ensure_non_empty(&raw.choices.duplication, "choices.duplication", label)?;
    ensure_non_empty(&raw.choices.event_semantics, "choices.event_semantics", label)?;
    ensure_non_empty(&raw.choices.adoption_mode, "choices.adoption_mode", label)?;
    ensure_list(&raw.expectations, "expectations", label)?;
    ensure_list(&raw.runtime_decisions, "runtime_decisions", label)?;

    let mut service_ids = BTreeSet::new();
    let mut services = Vec::new();
    for service in raw.services {
        ensure_non_empty(&service.id, "services[].id", label)?;
        ensure_non_empty(&service.capability, "services[].capability", label)?;
        ensure_non_empty(&service.version, "services[].version", label)?;
        ensure_non_empty(&service.summary, "services[].summary", label)?;
        ensure_list(&service.placements, "services[].placements", label)?;
        ensure_list(&service.consumers, "services[].consumers", label)?;
        if !service_ids.insert(service.id.clone()) {
            return Err(format!("{label}: duplicate service id \"{}\"", service.id));
        }
        services.push(Service {
            id: service.id,
            capability: service.capability,
            version: service.version,
            summary: service.summary,
            placements: service.placements,
            consumers: service.consumers,
        });
    }
    services.sort_by(|a, b| a.id.cmp(&b.id));

    let mut interactions = Vec::new();
    for interaction in raw.interactions {
        ensure_non_empty(&interaction.from, "interactions[].from", label)?;
        ensure_non_empty(&interaction.to, "interactions[].to", label)?;
        ensure_non_empty(&interaction.relation, "interactions[].relation", label)?;
        interactions.push(Interaction {
            from: interaction.from,
            to: interaction.to,
            relation: interaction.relation,
        });
    }
    interactions.sort_by(|a, b| {
        a.from
            .cmp(&b.from)
            .then(a.relation.cmp(&b.relation))
            .then(a.to.cmp(&b.to))
    });

    let choices = Choices {
        contract_anchor: raw.choices.contract_anchor,
        versioning: raw.choices.versioning,
        runtime_governance: raw.choices.runtime_governance,
        duplication: raw.choices.duplication,
        event_semantics: raw.choices.event_semantics,
        adoption_mode: raw.choices.adoption_mode,
    };

    let assessment = assess(&choices);

    Ok(ScenarioReport {
        scenario: label.to_string(),
        title: raw.title,
        summary: raw.summary,
        services,
        interactions,
        choices,
        expectations: raw.expectations,
        runtime_decisions: raw.runtime_decisions,
        assessment,
    })
}

fn assess(choices: &Choices) -> Assessment {
    let mut warnings = Vec::new();

    if choices.contract_anchor == "drifting" {
        warnings.push(Warning {
            code: "behavioral_drift".into(),
            message: "The same contract is being stretched across incompatible behavioral expectations."
                .into(),
        });
    }

    if choices.duplication == "cross-environment" {
        warnings.push(Warning {
            code: "duplicate_behavior".into(),
            message: "The same conceptual capability now exists in multiple implementations that can drift apart."
                .into(),
        });
    }

    if choices.versioning == "uncontrolled-proliferation" {
        warnings.push(Warning {
            code: "version_fragmentation".into(),
            message:
                "Multiple versions coexist without clear compatibility or lifecycle rules.".into(),
        });
    }

    if choices.event_semantics == "unstable" {
        warnings.push(Warning {
            code: "semantic_instability".into(),
            message:
                "Events remain structurally valid but their meaning is no longer stable for all consumers."
                    .into(),
        });
    }

    if choices.runtime_governance == "manual-only" {
        warnings.push(Warning {
            code: "manual_governance_limit".into(),
            message:
                "Human coordination is carrying behavior alignment instead of runtime enforcement."
                    .into(),
        });
    }

    if choices.adoption_mode == "hybrid" && choices.runtime_governance != "runtime-enforced" {
        warnings.push(Warning {
            code: "hybrid_boundary_risk".into(),
            message:
                "Hybrid adoption without strong boundary enforcement leaves legacy assumptions exposed."
                    .into(),
        });
    }

    let governed_shape = choices.runtime_governance == "runtime-enforced"
        && (choices.versioning == "controlled-coexistence" || choices.adoption_mode == "hybrid");

    let verdict = if warnings
        .iter()
        .any(|warning| warning.code == "version_fragmentation")
    {
        "fragmented"
    } else if governed_shape {
        "governed"
    } else if warnings.is_empty() {
        "coherent"
    } else {
        "at-risk"
    };

    Assessment {
        verdict: verdict.to_string(),
        warnings,
    }
}

pub fn project_root() -> PathBuf {
    let cwd = std::env::current_dir().ok();
    if let Some(cwd) = cwd {
        if cwd.join("scenarios").exists() {
            return cwd;
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(parent) = manifest_dir.parent() {
        if parent.join("scenarios").exists() {
            return parent.to_path_buf();
        }
    }

    manifest_dir
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

    writeln!(&mut out, "Evolution Axes").unwrap();
    writeln!(
        &mut out,
        "- contract anchor: {}",
        report.choices.contract_anchor
    )
    .unwrap();
    writeln!(&mut out, "- versioning: {}", report.choices.versioning).unwrap();
    writeln!(
        &mut out,
        "- runtime governance: {}",
        report.choices.runtime_governance
    )
    .unwrap();
    writeln!(&mut out, "- duplication: {}", report.choices.duplication).unwrap();
    writeln!(
        &mut out,
        "- event semantics: {}",
        report.choices.event_semantics
    )
    .unwrap();
    writeln!(&mut out, "- adoption mode: {}", report.choices.adoption_mode).unwrap();
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Capabilities").unwrap();
    for service in &report.services {
        writeln!(
            &mut out,
            "- {} {} ({})",
            service.id, service.version, service.capability
        )
        .unwrap();
        writeln!(&mut out, "  summary: {}", service.summary).unwrap();
        writeln!(&mut out, "  placements: {}", service.placements.join(", ")).unwrap();
        if !service.consumers.is_empty() {
            writeln!(&mut out, "  consumed by: {}", service.consumers.join(", ")).unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Interaction Surface").unwrap();
    if report.interactions.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for interaction in &report.interactions {
            writeln!(
                &mut out,
                "- {} -> {} -> {}",
                interaction.from, interaction.relation, interaction.to
            )
            .unwrap();
        }
    }
    writeln!(&mut out).unwrap();

    writeln!(&mut out, "Runtime Decisions").unwrap();
    if report.runtime_decisions.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for decision in &report.runtime_decisions {
            writeln!(&mut out, "- {}", decision).unwrap();
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
            "contract anchor",
            &from.choices.contract_anchor,
            &to.choices.contract_anchor,
        ),
        ("versioning", &from.choices.versioning, &to.choices.versioning),
        (
            "runtime governance",
            &from.choices.runtime_governance,
            &to.choices.runtime_governance,
        ),
        ("duplication", &from.choices.duplication, &to.choices.duplication),
        (
            "event semantics",
            &from.choices.event_semantics,
            &to.choices.event_semantics,
        ),
        (
            "adoption mode",
            &from.choices.adoption_mode,
            &to.choices.adoption_mode,
        ),
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
    writeln!(&mut out, "Evolution diff: {} -> {}", diff.from, diff.to).unwrap();
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
            "Validated {}: {} capabilities, {} interactions, verdict={}",
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

    fn sample_choices() -> RawChoices {
        RawChoices {
            contract_anchor: "stable".into(),
            versioning: "single-contract-line".into(),
            runtime_governance: "runtime-enforced".into(),
            duplication: "none".into(),
            event_semantics: "stable".into(),
            adoption_mode: "governed".into(),
        }
    }

    fn sample_scenario() -> RawScenario {
        RawScenario {
            title: "Sample".into(),
            summary: "Scenario summary".into(),
            services: vec![RawService {
                id: "cap-a".into(),
                capability: "domain.capability".into(),
                version: "1.0.0".into(),
                summary: "Capability summary".into(),
                placements: vec!["cloud".into(), "edge".into()],
                consumers: vec!["consumer-a".into()],
            }],
            interactions: vec![RawInteraction {
                from: "cap-a".into(),
                to: "consumer-a".into(),
                relation: "emits domain.event".into(),
            }],
            choices: sample_choices(),
            expectations: vec!["One stable behavior should remain visible.".into()],
            runtime_decisions: vec!["Validated one governed path.".into()],
        }
    }

    fn temp_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("chapter11-tests-{stamp}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn baseline_is_coherent() {
        let report = load_report(&project_root(), "lab1-contract-anchor").unwrap();
        assert_eq!(report.assessment.verdict, "coherent");
        assert!(report.assessment.warnings.is_empty());
    }

    #[test]
    fn drift_lab_reports_behavioral_drift() {
        let report = load_report(&project_root(), "lab2-behavioral-drift").unwrap();
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "behavioral_drift"));
    }

    #[test]
    fn version_sprawl_lab_is_fragmented() {
        let report = load_report(&project_root(), "lab4-version-sprawl").unwrap();
        assert_eq!(report.assessment.verdict, "fragmented");
        assert!(report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "version_fragmentation"));
    }

    #[test]
    fn runtime_governed_lab_is_governed() {
        let report = load_report(&project_root(), "lab5-runtime-governed-coexistence").unwrap();
        assert_eq!(report.assessment.verdict, "governed");
        assert!(!report
            .assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "version_fragmentation"));
    }

    #[test]
    fn diff_reports_controlled_versioning_shift() {
        let from = load_report(&project_root(), "lab4-version-sprawl").unwrap();
        let to = load_report(&project_root(), "lab5-runtime-governed-coexistence").unwrap();
        let diff = diff_reports(&from, &to);
        assert!(diff
            .changed_axes
            .iter()
            .any(|item| item.contains("versioning")));
        assert!(diff
            .removed_warnings
            .iter()
            .any(|warning| warning == "version_fragmentation"));
    }

    #[test]
    fn unknown_lab_error_lists_available_options() {
        let error = load_report(&project_root(), "does-not-exist").unwrap_err();
        assert!(error.contains("unknown lab"));
        assert!(error.contains("lab1-contract-anchor"));
    }

    #[test]
    fn validate_scenario_rejects_empty_title() {
        let mut raw = sample_scenario();
        raw.title = "   ".into();
        let error = validate_scenario(raw, "sample").unwrap_err();
        assert!(error.contains("\"title\" must be a non-empty string"));
    }

    #[test]
    fn validate_scenario_rejects_duplicate_service_ids() {
        let mut raw = sample_scenario();
        raw.services.push(RawService {
            id: "cap-a".into(),
            capability: "domain.other".into(),
            version: "1.0.1".into(),
            summary: "Duplicate".into(),
            placements: vec!["cloud".into()],
            consumers: Vec::new(),
        });
        let error = validate_scenario(raw, "sample").unwrap_err();
        assert!(error.contains("duplicate service id"));
    }

    #[test]
    fn validate_scenario_rejects_blank_list_entries() {
        let mut raw = sample_scenario();
        raw.expectations = vec!["".into()];
        let error = validate_scenario(raw, "sample").unwrap_err();
        assert!(error.contains("\"expectations\" must be a non-empty string"));
    }

    #[test]
    fn assess_adds_hybrid_boundary_risk_when_not_runtime_enforced() {
        let choices = Choices {
            contract_anchor: "stable".into(),
            versioning: "controlled-coexistence".into(),
            runtime_governance: "manual-only".into(),
            duplication: "none".into(),
            event_semantics: "stable".into(),
            adoption_mode: "hybrid".into(),
        };
        let assessment = assess(&choices);
        assert_eq!(assessment.verdict, "at-risk");
        assert!(assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "hybrid_boundary_risk"));
        assert!(assessment
            .warnings
            .iter()
            .any(|warning| warning.code == "manual_governance_limit"));
    }

    #[test]
    fn assess_marks_runtime_governed_hybrid_as_governed() {
        let choices = Choices {
            contract_anchor: "stable".into(),
            versioning: "single-contract-line".into(),
            runtime_governance: "runtime-enforced".into(),
            duplication: "none".into(),
            event_semantics: "stable".into(),
            adoption_mode: "hybrid".into(),
        };
        let assessment = assess(&choices);
        assert_eq!(assessment.verdict, "governed");
        assert!(assessment.warnings.is_empty());
    }

    #[test]
    fn format_report_renders_optional_empty_sections() {
        let report = ScenarioReport {
            scenario: "sample".into(),
            title: "No optional sections".into(),
            summary: "summary".into(),
            services: vec![Service {
                id: "svc".into(),
                capability: "cap".into(),
                version: "1.0.0".into(),
                summary: "summary".into(),
                placements: vec!["cloud".into()],
                consumers: Vec::new(),
            }],
            interactions: Vec::new(),
            choices: Choices {
                contract_anchor: "stable".into(),
                versioning: "single-contract-line".into(),
                runtime_governance: "runtime-enforced".into(),
                duplication: "none".into(),
                event_semantics: "stable".into(),
                adoption_mode: "governed".into(),
            },
            expectations: vec!["expectation".into()],
            runtime_decisions: Vec::new(),
            assessment: Assessment {
                verdict: "coherent".into(),
                warnings: Vec::new(),
            },
        };
        let rendered = format_report(&report);
        assert!(rendered.contains("Interaction Surface\n- none"));
        assert!(rendered.contains("Runtime Decisions\n- none"));
        assert!(rendered.contains("Warnings\n- none"));
        assert!(!rendered.contains("consumed by"));
    }

    #[test]
    fn format_diff_renders_empty_sections() {
        let diff = ScenarioDiff {
            from: "a".into(),
            to: "b".into(),
            verdict_change: "coherent -> coherent".into(),
            added_warnings: Vec::new(),
            removed_warnings: Vec::new(),
            changed_axes: Vec::new(),
        };
        let rendered = format_diff(&diff);
        assert!(rendered.contains("Changed axes\n- none"));
        assert!(rendered.contains("Added warnings\n- none"));
        assert!(rendered.contains("Removed warnings\n- none"));
    }

    #[test]
    fn list_labs_and_validate_all_work_from_temp_root() {
        let root = temp_root();
        let scenarios = root.join("scenarios");
        fs::create_dir_all(scenarios.join("lab-b")).unwrap();
        fs::create_dir_all(scenarios.join("lab-a")).unwrap();
        fs::write(
            scenarios.join("lab-a").join("scenario.json"),
            serde_json::to_string_pretty(&sample_scenario()).unwrap(),
        )
        .unwrap();
        fs::write(
            scenarios.join("lab-b").join("scenario.json"),
            serde_json::to_string_pretty(&sample_scenario()).unwrap(),
        )
        .unwrap();

        let labs = list_labs(&root).unwrap();
        assert_eq!(labs, vec!["lab-a".to_string(), "lab-b".to_string()]);

        let summaries = validate_all(&root).unwrap();
        assert_eq!(summaries.len(), 2);
        assert!(summaries[0].contains("Validated lab-a"));
        assert!(summaries[1].contains("Validated lab-b"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn project_root_resolves_repo_root_from_test_context() {
        let root = project_root();
        assert!(root.join("scenarios").exists());
    }

    #[test]
    fn load_report_rejects_invalid_json() {
        let root = temp_root();
        let lab_dir = root.join("scenarios").join("lab-json");
        fs::create_dir_all(&lab_dir).unwrap();
        fs::write(lab_dir.join("scenario.json"), "{not valid json").unwrap();

        let error = load_report(&root, "lab-json").unwrap_err();

        fs::remove_dir_all(root).unwrap();
        assert!(!error.is_empty());
    }

    #[test]
    fn format_report_renders_full_sections() {
        let report = load_report(&project_root(), "lab5-runtime-governed-coexistence").unwrap();
        let rendered = format_report(&report);
        assert!(rendered.contains("Capabilities"));
        assert!(rendered.contains("consumed by: legacy-sync"));
        assert!(rendered.contains("Interaction Surface"));
        assert!(rendered.contains("Runtime Decisions"));
        assert!(rendered.contains("Selected v1 only for consumers"));
        assert!(rendered.contains("Reader Value"));
    }

    #[test]
    fn format_diff_renders_added_and_removed_warnings() {
        let from = load_report(&project_root(), "lab5-runtime-governed-coexistence").unwrap();
        let to = load_report(&project_root(), "lab4-version-sprawl").unwrap();
        let diff = diff_reports(&from, &to);
        let rendered = format_diff(&diff);
        assert!(rendered.contains("Added warnings"));
        assert!(rendered.contains("version_fragmentation"));
        assert!(rendered.contains("Changed axes"));
        assert!(rendered.contains("contract anchor: stable -> drifting"));
    }
}
