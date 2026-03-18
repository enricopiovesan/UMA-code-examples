use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawContract {
    kind: String,
    #[serde(rename = "specVersion")]
    spec_version: String,
    service: RawService,
    capabilities: Vec<RawCapability>,
    events: RawEvents,
    io: RawIo,
}

#[derive(Debug, Deserialize)]
struct RawService {
    id: String,
    version: String,
    summary: String,
    placements: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawCapability {
    id: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct RawEvents {
    consumes: Vec<RawEventContract>,
    emits: Vec<RawEventContract>,
}

#[derive(Debug, Deserialize)]
struct RawEventContract {
    name: String,
    schema: String,
}

#[derive(Debug, Deserialize)]
struct RawIo {
    #[serde(rename = "inputSchema")]
    input_schema: String,
    #[serde(rename = "outputSchema")]
    output_schema: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CapabilityVersion {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventSchema {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventSchemas {
    pub consumes: Vec<EventSchema>,
    pub emits: Vec<EventSchema>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IoSchemas {
    #[serde(rename = "inputSchema")]
    pub input_schema: String,
    #[serde(rename = "outputSchema")]
    pub output_schema: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ServiceContract {
    pub kind: String,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    pub id: String,
    #[serde(rename = "serviceVersion")]
    pub service_version: String,
    pub summary: String,
    pub placements: Vec<String>,
    pub capabilities: Vec<String>,
    #[serde(rename = "capabilityVersions")]
    pub capability_versions: Vec<CapabilityVersion>,
    pub consumes: Vec<String>,
    pub emits: Vec<String>,
    #[serde(rename = "eventSchemas")]
    pub event_schemas: EventSchemas,
    pub io: IoSchemas,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub event: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WaitingConsumer {
    pub service: String,
    pub event: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Graph {
    pub services: Vec<ServiceContract>,
    pub edges: Vec<Edge>,
    pub waiting: Vec<WaitingConsumer>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ScenarioGraph {
    pub scenario: String,
    #[serde(flatten)]
    pub graph: Graph,
}

fn ensure_non_empty(value: &str, field: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{label}: \"{field}\" must be a non-empty string"));
    }
    Ok(())
}

fn ensure_string_list(values: &[String], field: &str, label: &str) -> Result<(), String> {
    for value in values {
        ensure_non_empty(value, field, label)?;
    }
    Ok(())
}

fn ensure_schema_exists(root_dir: &Path, schema_path: &str, label: &str) -> Result<(), String> {
    let full = root_dir.join(schema_path);
    if !full.exists() {
        return Err(format!(
            "{label}: schema points to missing file \"{schema_path}\""
        ));
    }
    Ok(())
}

fn validate_contract(raw: RawContract, label: &str, root_dir: &Path) -> Result<ServiceContract, String> {
    ensure_non_empty(&raw.kind, "kind", label)?;
    ensure_non_empty(&raw.spec_version, "specVersion", label)?;
    if raw.kind != "uma.service-contract" {
        return Err(format!(
            "{label}: \"kind\" must be \"uma.service-contract\""
        ));
    }

    ensure_non_empty(&raw.service.id, "service.id", label)?;
    ensure_non_empty(&raw.service.version, "service.version", label)?;
    ensure_non_empty(&raw.service.summary, "service.summary", label)?;
    ensure_string_list(&raw.service.placements, "service.placements", label)?;

    let mut capability_seen = BTreeSet::new();
    for capability in &raw.capabilities {
        ensure_non_empty(&capability.id, "capabilities[].id", label)?;
        ensure_non_empty(&capability.version, "capabilities[].version", label)?;
        let key = format!("{}@{}", capability.id, capability.version);
        if !capability_seen.insert(key.clone()) {
            return Err(format!("{label}: duplicate capability \"{key}\""));
        }
    }

    let mut consume_seen = BTreeSet::new();
    for event in &raw.events.consumes {
        ensure_non_empty(&event.name, "events.consumes[].name", label)?;
        ensure_non_empty(&event.schema, "events.consumes[].schema", label)?;
        ensure_schema_exists(root_dir, &event.schema, label)?;
        if !consume_seen.insert(event.name.clone()) {
            return Err(format!(
                "{label}: duplicate event \"{}\" in \"events.consumes\"",
                event.name
            ));
        }
    }

    let mut emit_seen = BTreeSet::new();
    for event in &raw.events.emits {
        ensure_non_empty(&event.name, "events.emits[].name", label)?;
        ensure_non_empty(&event.schema, "events.emits[].schema", label)?;
        ensure_schema_exists(root_dir, &event.schema, label)?;
        if !emit_seen.insert(event.name.clone()) {
            return Err(format!(
                "{label}: duplicate event \"{}\" in \"events.emits\"",
                event.name
            ));
        }
    }

    ensure_non_empty(&raw.io.input_schema, "io.inputSchema", label)?;
    ensure_non_empty(&raw.io.output_schema, "io.outputSchema", label)?;
    ensure_schema_exists(root_dir, &raw.io.input_schema, label)?;
    ensure_schema_exists(root_dir, &raw.io.output_schema, label)?;

    Ok(ServiceContract {
        kind: raw.kind,
        spec_version: raw.spec_version,
        id: raw.service.id,
        service_version: raw.service.version,
        summary: raw.service.summary,
        placements: raw.service.placements,
        capabilities: raw.capabilities.iter().map(|item| item.id.clone()).collect(),
        capability_versions: raw
            .capabilities
            .into_iter()
            .map(|item| CapabilityVersion {
                id: item.id,
                version: item.version,
            })
            .collect(),
        consumes: raw.events.consumes.iter().map(|item| item.name.clone()).collect(),
        emits: raw.events.emits.iter().map(|item| item.name.clone()).collect(),
        event_schemas: EventSchemas {
            consumes: raw
                .events
                .consumes
                .into_iter()
                .map(|item| EventSchema {
                    name: item.name,
                    schema: item.schema,
                })
                .collect(),
            emits: raw
                .events
                .emits
                .into_iter()
                .map(|item| EventSchema {
                    name: item.name,
                    schema: item.schema,
                })
                .collect(),
        },
        io: IoSchemas {
            input_schema: raw.io.input_schema,
            output_schema: raw.io.output_schema,
        },
    })
}

pub fn project_root() -> PathBuf {
    let cwd = std::env::current_dir().ok();
    if let Some(cwd) = cwd {
        if cwd.join("scenarios").exists() && cwd.join("contracts").exists() {
            return cwd;
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest_dir.join("scenarios").exists() && manifest_dir.join("contracts").exists() {
        return manifest_dir;
    }

    manifest_dir
}

pub fn list_scenarios(root_dir: &Path) -> Result<Vec<String>, String> {
    let mut scenarios = Vec::new();
    for entry in fs::read_dir(root_dir.join("scenarios")).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if entry.file_type().map_err(|err| err.to_string())?.is_dir() {
            scenarios.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    scenarios.sort();
    Ok(scenarios)
}

pub fn load_services(root_dir: &Path, scenario_name: &str) -> Result<Vec<ServiceContract>, String> {
    let scenarios = list_scenarios(root_dir)?;
    if !scenarios.iter().any(|name| name == scenario_name) {
        return Err(format!(
            "unknown scenario \"{scenario_name}\". Available scenarios: {}",
            scenarios.join(", ")
        ));
    }

    let service_dir = root_dir.join("scenarios").join(scenario_name).join("services");
    let mut services = Vec::new();
    let mut service_ids = BTreeSet::new();
    for entry in fs::read_dir(&service_dir).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if !entry.file_type().map_err(|err| err.to_string())?.is_file() {
            continue;
        }
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let relative = entry
            .path()
            .strip_prefix(root_dir.join("scenarios").join(scenario_name))
            .map_err(|err| err.to_string())?
            .display()
            .to_string();
        let raw: RawContract =
            serde_json::from_str(&fs::read_to_string(entry.path()).map_err(|err| err.to_string())?)
                .map_err(|err| format!("{relative}: {err}"))?;
        let contract = validate_contract(raw, &relative, root_dir)?;
        if !service_ids.insert(contract.id.clone()) {
            return Err(format!(
                "duplicate service id \"{}\" in scenarios/{scenario_name}",
                contract.id
            ));
        }
        services.push(contract);
    }
    services.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(services)
}

pub fn build_graph(services: &[ServiceContract]) -> Graph {
    let mut emitters: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for service in services {
        for event_name in &service.emits {
            emitters
                .entry(event_name.clone())
                .or_default()
                .push(service.id.clone());
        }
    }

    let mut edges = Vec::new();
    let mut waiting = Vec::new();

    for service in services {
        for event_name in &service.consumes {
            if let Some(producers) = emitters.get(event_name) {
                for producer in producers {
                    edges.push(Edge {
                        from: producer.clone(),
                        event: event_name.clone(),
                        to: service.id.clone(),
                    });
                }
            } else {
                waiting.push(WaitingConsumer {
                    service: service.id.clone(),
                    event: event_name.clone(),
                });
            }
        }
    }

    edges.sort_by(|left, right| {
        left.from
            .cmp(&right.from)
            .then(left.event.cmp(&right.event))
            .then(left.to.cmp(&right.to))
    });
    waiting.sort_by(|left, right| left.service.cmp(&right.service).then(left.event.cmp(&right.event)));

    Graph {
        services: services.to_vec(),
        edges,
        waiting,
    }
}

pub fn load_scenario_graph(root_dir: &Path, scenario_name: &str) -> Result<ScenarioGraph, String> {
    let services = load_services(root_dir, scenario_name)?;
    Ok(ScenarioGraph {
        scenario: scenario_name.to_string(),
        graph: build_graph(&services),
    })
}

pub fn format_graph(report: &ScenarioGraph) -> String {
    let mut out = String::new();
    writeln!(&mut out, "Scenario: {}", report.scenario).unwrap();
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Services").unwrap();

    for service in &report.graph.services {
        writeln!(&mut out, "- {} v{}", service.id, service.service_version).unwrap();
        writeln!(&mut out, "  summary: {}", service.summary).unwrap();
        writeln!(
            &mut out,
            "  placements: {}",
            service.placements.join(", ")
        )
        .unwrap();
        for capability in &service.capability_versions {
            writeln!(
                &mut out,
                "  capability: {}@{}",
                capability.id, capability.version
            )
            .unwrap();
        }
        for event in &service.event_schemas.consumes {
            writeln!(&mut out, "  consumes: {} ({})", event.name, event.schema).unwrap();
        }
        for event in &service.event_schemas.emits {
            writeln!(&mut out, "  emits: {} ({})", event.name, event.schema).unwrap();
        }
    }

    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Edges").unwrap();
    if report.graph.edges.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for edge in &report.graph.edges {
            writeln!(&mut out, "- {} -> {} -> {}", edge.from, edge.event, edge.to).unwrap();
        }
    }

    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Waiting Consumers").unwrap();
    if report.graph.waiting.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for entry in &report.graph.waiting {
            writeln!(&mut out, "- {} waiting for {}", entry.service, entry.event).unwrap();
        }
    }

    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDiff {
    pub from_scenario: String,
    pub to_scenario: String,
    pub added_services: Vec<String>,
    pub removed_services: Vec<String>,
    pub added_edges: Vec<String>,
    pub removed_edges: Vec<String>,
    pub added_waiting: Vec<String>,
    pub removed_waiting: Vec<String>,
}

fn diff_vec(left: &[String], right: &[String]) -> (Vec<String>, Vec<String>) {
    let left_set: BTreeSet<_> = left.iter().cloned().collect();
    let right_set: BTreeSet<_> = right.iter().cloned().collect();
    let added = right_set.difference(&left_set).cloned().collect();
    let removed = left_set.difference(&right_set).cloned().collect();
    (added, removed)
}

pub fn diff_graphs(from: &ScenarioGraph, to: &ScenarioGraph) -> GraphDiff {
    let left_services: Vec<String> = from.graph.services.iter().map(|item| item.id.clone()).collect();
    let right_services: Vec<String> = to.graph.services.iter().map(|item| item.id.clone()).collect();
    let left_edges: Vec<String> = from
        .graph
        .edges
        .iter()
        .map(|edge| format!("{} -> {} -> {}", edge.from, edge.event, edge.to))
        .collect();
    let right_edges: Vec<String> = to
        .graph
        .edges
        .iter()
        .map(|edge| format!("{} -> {} -> {}", edge.from, edge.event, edge.to))
        .collect();
    let left_waiting: Vec<String> = from
        .graph
        .waiting
        .iter()
        .map(|item| format!("{} waiting for {}", item.service, item.event))
        .collect();
    let right_waiting: Vec<String> = to
        .graph
        .waiting
        .iter()
        .map(|item| format!("{} waiting for {}", item.service, item.event))
        .collect();

    let (added_services, removed_services) = diff_vec(&left_services, &right_services);
    let (added_edges, removed_edges) = diff_vec(&left_edges, &right_edges);
    let (added_waiting, removed_waiting) = diff_vec(&left_waiting, &right_waiting);

    GraphDiff {
        from_scenario: from.scenario.clone(),
        to_scenario: to.scenario.clone(),
        added_services,
        removed_services,
        added_edges,
        removed_edges,
        added_waiting,
        removed_waiting,
    }
}

pub fn format_graph_diff(diff: &GraphDiff) -> String {
    let mut out = String::new();
    writeln!(
        &mut out,
        "Graph diff: {} -> {}",
        diff.from_scenario, diff.to_scenario
    )
    .unwrap();
    writeln!(&mut out).unwrap();

    let sections = [
        ("Added services", &diff.added_services),
        ("Removed services", &diff.removed_services),
        ("Added edges", &diff.added_edges),
        ("Removed edges", &diff.removed_edges),
        ("Added waiting consumers", &diff.added_waiting),
        ("Removed waiting consumers", &diff.removed_waiting),
    ];

    for (index, (title, values)) in sections.iter().enumerate() {
        writeln!(&mut out, "{title}").unwrap();
        if values.is_empty() {
            writeln!(&mut out, "- none").unwrap();
        } else {
            for value in *values {
                writeln!(&mut out, "- {value}").unwrap();
            }
        }
        if index + 1 != sections.len() {
            writeln!(&mut out).unwrap();
        }
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
        let dir = std::env::temp_dir().join(format!("chapter8-tests-{pid}-{nanos}-{counter}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn list_returns_expected_order() {
        let scenarios = list_scenarios(&project_root()).unwrap();
        assert_eq!(
            scenarios,
            vec![
                "lab1-upload-only",
                "lab2-image-tagger",
                "lab3-indexer",
                "lab4-broken-compat",
                "lab5-fixed-compat"
            ]
        );
    }

    #[test]
    fn lab3_has_expected_edges() {
        let report = load_scenario_graph(&project_root(), "lab3-indexer").unwrap();
        assert_eq!(report.graph.services.len(), 3);
        assert_eq!(
            report.graph.edges,
            vec![
                Edge {
                    from: "image-tagger".into(),
                    event: "image.tagged".into(),
                    to: "metadata-indexer".into()
                },
                Edge {
                    from: "upload-service".into(),
                    event: "image.uploaded".into(),
                    to: "image-tagger".into()
                }
            ]
        );
        assert!(report.graph.waiting.is_empty());
    }

    #[test]
    fn broken_compatibility_creates_waiting_consumer() {
        let report = load_scenario_graph(&project_root(), "lab4-broken-compat").unwrap();
        assert_eq!(
            report.graph.waiting,
            vec![WaitingConsumer {
                service: "metadata-indexer".into(),
                event: "image.tagged".into()
            }]
        );
    }

    #[test]
    fn unknown_scenario_is_helpful() {
        let error = load_scenario_graph(&project_root(), "missing").unwrap_err();
        assert!(error.contains("unknown scenario"));
    }

    #[test]
    fn missing_schema_is_rejected_during_load() {
        let root = temp_root();
        write_file(
            &root.join("scenarios/lab1/services/upload.json"),
            r#"{
  "kind": "uma.service-contract",
  "specVersion": "1.0",
  "service": {
    "id": "upload-service",
    "version": "1.0.0",
    "summary": "demo",
    "placements": ["cloud"]
  },
  "capabilities": [{"id": "media.ingest", "version": "1.0"}],
  "events": {
    "consumes": [],
    "emits": [{"name": "image.uploaded", "schema": "contracts/schemas/missing.json"}]
  },
  "io": {
    "inputSchema": "contracts/schemas/input.json",
    "outputSchema": "contracts/schemas/output.json"
  }
}"#,
        );
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");

        let err = load_services(&root, "lab1").unwrap_err();
        assert!(err.contains("missing file"));
        assert!(err.contains("contracts/schemas/missing.json"));
    }

    #[test]
    fn duplicate_service_ids_are_rejected() {
        let root = temp_root();
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");
        write_file(&root.join("contracts/schemas/event.json"), "{}");
        let service = r#"{
  "kind": "uma.service-contract",
  "specVersion": "1.0",
  "service": {
    "id": "dup-service",
    "version": "1.0.0",
    "summary": "demo",
    "placements": ["cloud"]
  },
  "capabilities": [{"id": "media.ingest", "version": "1.0"}],
  "events": {
    "consumes": [],
    "emits": [{"name": "image.uploaded", "schema": "contracts/schemas/event.json"}]
  },
  "io": {
    "inputSchema": "contracts/schemas/input.json",
    "outputSchema": "contracts/schemas/output.json"
  }
}"#;
        write_file(&root.join("scenarios/lab1/services/one.json"), service);
        write_file(&root.join("scenarios/lab1/services/two.json"), service);

        let err = load_services(&root, "lab1").unwrap_err();
        assert!(err.contains("duplicate service id"));
    }

    #[test]
    fn graph_diff_reports_added_waiting_consumers() {
        let stable = load_scenario_graph(&project_root(), "lab3-indexer").unwrap();
        let broken = load_scenario_graph(&project_root(), "lab4-broken-compat").unwrap();
        let diff = diff_graphs(&stable, &broken);

        assert_eq!(diff.added_waiting, vec!["metadata-indexer waiting for image.tagged"]);
        assert_eq!(diff.removed_edges, vec!["image-tagger -> image.tagged -> metadata-indexer"]);
    }
}
