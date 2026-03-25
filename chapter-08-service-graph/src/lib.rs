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

fn err_string<E: ToString>(err: E) -> String {
    err.to_string()
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

fn resolve_project_root(cwd: Option<PathBuf>, manifest_dir: PathBuf) -> PathBuf {
    match cwd {
        Some(cwd) if cwd.join("scenarios").exists() && cwd.join("contracts").exists() => cwd,
        _ if manifest_dir.join("scenarios").exists() && manifest_dir.join("contracts").exists() => {
            manifest_dir
        }
        _ => manifest_dir,
    }
}

pub fn project_root() -> PathBuf {
    resolve_project_root(
        std::env::current_dir().ok(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
}

pub fn list_scenarios(root_dir: &Path) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(root_dir.join("scenarios")).map_err(err_string)?;
    let mut scenarios = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
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
    let entries = fs::read_dir(&service_dir).map_err(err_string)?;
    for entry in entries.filter_map(Result::ok) {
        if !entry.path().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let relative = format!("services/{}", entry.file_name().to_string_lossy());
        let raw_contents = fs::read_to_string(entry.path()).map_err(err_string)?;
        let raw: RawContract = match serde_json::from_str(&raw_contents) {
            Ok(raw) => raw,
            Err(err) => return Err(format!("{relative}: {err}")),
        };
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
    use std::os::unix::fs::PermissionsExt;
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
        let parent = path.parent().expect("write_file paths should have a parent");
        fs::create_dir_all(parent).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn raw_contract() -> RawContract {
        RawContract {
            kind: "uma.service-contract".to_string(),
            spec_version: "1.0".to_string(),
            service: RawService {
                id: "image-tagger".to_string(),
                version: "1.0.0".to_string(),
                summary: "tags images".to_string(),
                placements: vec!["edge".to_string(), "cloud".to_string()],
            },
            capabilities: vec![RawCapability {
                id: "media.tag".to_string(),
                version: "1.0".to_string(),
            }],
            events: RawEvents {
                consumes: vec![RawEventContract {
                    name: "image.uploaded".to_string(),
                    schema: "contracts/schemas/input.json".to_string(),
                }],
                emits: vec![RawEventContract {
                    name: "image.tagged".to_string(),
                    schema: "contracts/schemas/event.json".to_string(),
                }],
            },
            io: RawIo {
                input_schema: "contracts/schemas/input.json".to_string(),
                output_schema: "contracts/schemas/output.json".to_string(),
            },
        }
    }

    fn write_schema_set(root: &Path) {
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");
        write_file(&root.join("contracts/schemas/event.json"), "{}");
    }

    fn write_service(root: &Path, scenario: &str, name: &str, contents: &str) {
        write_file(
            &root.join("scenarios").join(scenario).join("services").join(name),
            contents,
        );
    }

    fn write_contract_json(root: &Path, scenario: &str, name: &str, raw: &RawContract) {
        let capabilities = raw
            .capabilities
            .iter()
            .map(|item| {
                format!(
                    r#"{{"id":"{}","version":"{}"}}"#,
                    item.id, item.version
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let consumes = raw
            .events
            .consumes
            .iter()
            .map(|item| {
                format!(
                    r#"{{"name":"{}","schema":"{}"}}"#,
                    item.name, item.schema
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let emits = raw
            .events
            .emits
            .iter()
            .map(|item| {
                format!(
                    r#"{{"name":"{}","schema":"{}"}}"#,
                    item.name, item.schema
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let placements = raw
            .service
            .placements
            .iter()
            .map(|item| format!(r#""{}""#, item))
            .collect::<Vec<_>>()
            .join(",");

        let json = format!(
            r#"{{
  "kind": "{}",
  "specVersion": "{}",
  "service": {{
    "id": "{}",
    "version": "{}",
    "summary": "{}",
    "placements": [{}]
  }},
  "capabilities": [{}],
  "events": {{
    "consumes": [{}],
    "emits": [{}]
  }},
  "io": {{
    "inputSchema": "{}",
    "outputSchema": "{}"
  }}
}}"#,
            raw.kind,
            raw.spec_version,
            raw.service.id,
            raw.service.version,
            raw.service.summary,
            placements,
            capabilities,
            consumes,
            emits,
            raw.io.input_schema,
            raw.io.output_schema
        );
        write_service(root, scenario, name, &json);
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

    #[test]
    fn validate_contract_rejects_blank_fields_and_duplicates() {
        let root = temp_root();
        write_schema_set(&root);

        let service_cases: [(&str, fn(&mut RawContract), &str); 8] = [
            ("kind", |raw| raw.kind = " ".into(), "\"kind\""),
            (
                "specVersion",
                |raw| raw.spec_version = " ".into(),
                "\"specVersion\"",
            ),
            ("service.id", |raw| raw.service.id = " ".into(), "\"service.id\""),
            (
                "service.version",
                |raw| raw.service.version = " ".into(),
                "\"service.version\"",
            ),
            (
                "service.summary",
                |raw| raw.service.summary = " ".into(),
                "\"service.summary\"",
            ),
            (
                "service.placements",
                |raw| raw.service.placements[0] = " ".into(),
                "\"service.placements\"",
            ),
            (
                "io.inputSchema",
                |raw| raw.io.input_schema = " ".into(),
                "\"io.inputSchema\"",
            ),
            (
                "io.outputSchema",
                |raw| raw.io.output_schema = " ".into(),
                "\"io.outputSchema\"",
            ),
        ];
        for (label, mutate, expected) in service_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_contract(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let mut wrong_kind = raw_contract();
        wrong_kind.kind = "wrong.kind".into();
        assert!(validate_contract(wrong_kind, "service.json", &root)
            .unwrap_err()
            .contains("\"kind\" must be \"uma.service-contract\""));

        let mut duplicate_capability = raw_contract();
        duplicate_capability.capabilities.push(RawCapability {
            id: "media.tag".into(),
            version: "1.0".into(),
        });
        assert!(validate_contract(duplicate_capability, "service.json", &root)
            .unwrap_err()
            .contains("duplicate capability"));

        let mut duplicate_consume = raw_contract();
        duplicate_consume.events.consumes.push(RawEventContract {
            name: "image.uploaded".into(),
            schema: "contracts/schemas/input.json".into(),
        });
        assert!(validate_contract(duplicate_consume, "service.json", &root)
            .unwrap_err()
            .contains("duplicate event"));

        let mut duplicate_emit = raw_contract();
        duplicate_emit.events.emits.push(RawEventContract {
            name: "image.tagged".into(),
            schema: "contracts/schemas/event.json".into(),
        });
        assert!(validate_contract(duplicate_emit, "service.json", &root)
            .unwrap_err()
            .contains("duplicate event"));

        let mut blank_capability_id = raw_contract();
        blank_capability_id.capabilities[0].id = " ".into();
        assert!(validate_contract(blank_capability_id, "service.json", &root)
            .unwrap_err()
            .contains("\"capabilities[].id\""));

        let mut blank_capability_version = raw_contract();
        blank_capability_version.capabilities[0].version = " ".into();
        assert!(validate_contract(blank_capability_version, "service.json", &root)
            .unwrap_err()
            .contains("\"capabilities[].version\""));
    }

    #[test]
    fn validate_contract_rejects_remaining_required_event_fields_and_missing_schemas() {
        let root = temp_root();
        write_schema_set(&root);

        let event_cases: [(&str, fn(&mut RawContract), &str); 4] = [
            (
                "events.consumes[].name",
                |raw| raw.events.consumes[0].name = " ".into(),
                "\"events.consumes[].name\"",
            ),
            (
                "events.consumes[].schema",
                |raw| raw.events.consumes[0].schema = " ".into(),
                "\"events.consumes[].schema\"",
            ),
            (
                "events.emits[].name",
                |raw| raw.events.emits[0].name = " ".into(),
                "\"events.emits[].name\"",
            ),
            (
                "events.emits[].schema",
                |raw| raw.events.emits[0].schema = " ".into(),
                "\"events.emits[].schema\"",
            ),
        ];
        for (label, mutate, expected) in event_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_contract(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let mut missing_consume_schema = raw_contract();
        missing_consume_schema.events.consumes[0].schema = "contracts/schemas/missing-consume.json".into();
        assert!(validate_contract(missing_consume_schema, "service.json", &root)
            .unwrap_err()
            .contains("missing file"));

        let mut missing_emit_schema = raw_contract();
        missing_emit_schema.events.emits[0].schema = "contracts/schemas/missing-emit.json".into();
        assert!(validate_contract(missing_emit_schema, "service.json", &root)
            .unwrap_err()
            .contains("missing file"));

        let mut missing_input = raw_contract();
        missing_input.io.input_schema = "contracts/schemas/missing-input.json".into();
        assert!(validate_contract(missing_input, "service.json", &root)
            .unwrap_err()
            .contains("missing file"));

        let mut missing_output = raw_contract();
        missing_output.io.output_schema = "contracts/schemas/missing-output.json".into();
        assert!(validate_contract(missing_output, "service.json", &root)
            .unwrap_err()
            .contains("missing file"));
    }

    #[test]
    fn resolve_project_root_covers_all_branches() {
        let cwd_root = temp_root();
        fs::create_dir_all(cwd_root.join("scenarios")).unwrap();
        fs::create_dir_all(cwd_root.join("contracts")).unwrap();
        assert_eq!(
            resolve_project_root(Some(cwd_root.clone()), PathBuf::from("/unused")),
            cwd_root
        );

        let manifest_root = temp_root();
        fs::create_dir_all(manifest_root.join("scenarios")).unwrap();
        fs::create_dir_all(manifest_root.join("contracts")).unwrap();
        assert_eq!(
            resolve_project_root(Some(temp_root()), manifest_root.clone()),
            manifest_root
        );

        let fallback = temp_root();
        assert_eq!(
            resolve_project_root(Some(temp_root()), fallback.clone()),
            fallback
        );
    }

    #[test]
    fn list_scenarios_rejects_missing_scenarios_root() {
        let root = temp_root();
        let error = list_scenarios(&root).unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn load_services_covers_unknown_missing_directory_invalid_json_and_non_json_filtering() {
        let root = temp_root();
        let error = load_services(&root, "missing").unwrap_err();
        assert!(!error.is_empty());

        fs::create_dir_all(root.join("scenarios/lab1")).unwrap();
        let error = load_services(&root, "lab1").unwrap_err();
        assert!(!error.is_empty());

        let root = temp_root();
        write_schema_set(&root);
        write_service(&root, "lab1", "broken.json", "{ invalid");
        let error = load_services(&root, "lab1").unwrap_err();
        assert!(error.contains("services/broken.json"));

        let root = temp_root();
        write_schema_set(&root);
        fs::create_dir_all(root.join("scenarios/lab1/services/archive")).unwrap();
        write_service(&root, "lab1", "readme.txt", "ignore");
        write_contract_json(&root, "lab1", "service.json", &raw_contract());
        let services = load_services(&root, "lab1").unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].id, "image-tagger");

        let root = temp_root();
        write_schema_set(&root);
        write_contract_json(&root, "lab1", "service.json", &raw_contract());
        let unreadable = root.join("scenarios/lab1/services/service.json");
        let mut permissions = fs::metadata(&unreadable).unwrap().permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(&unreadable, permissions).unwrap();
        let error = load_services(&root, "lab1").unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn format_graph_covers_full_and_empty_sections() {
        let full = load_scenario_graph(&project_root(), "lab3-indexer").unwrap();
        let rendered = format_graph(&full);
        assert!(rendered.contains("Scenario: lab3-indexer"));
        assert!(rendered.contains("Services"));
        assert!(rendered.contains("capability:"));
        assert!(rendered.contains("Edges"));
        assert!(rendered.contains("Waiting Consumers"));

        let empty = ScenarioGraph {
            scenario: "empty".into(),
            graph: Graph {
                services: vec![ServiceContract {
                    kind: "uma.service-contract".into(),
                    spec_version: "1.0".into(),
                    id: "single".into(),
                    service_version: "1.0.0".into(),
                    summary: "summary".into(),
                    placements: vec!["cloud".into()],
                    capabilities: vec!["cap".into()],
                    capability_versions: vec![CapabilityVersion {
                        id: "cap".into(),
                        version: "1.0".into(),
                    }],
                    consumes: Vec::new(),
                    emits: Vec::new(),
                    event_schemas: EventSchemas {
                        consumes: Vec::new(),
                        emits: Vec::new(),
                    },
                    io: IoSchemas {
                        input_schema: "contracts/schemas/input.json".into(),
                        output_schema: "contracts/schemas/output.json".into(),
                    },
                }],
                edges: Vec::new(),
                waiting: Vec::new(),
            },
        };
        let rendered = format_graph(&empty);
        assert!(rendered.contains("Edges\n- none"));
        assert!(rendered.contains("Waiting Consumers\n- none"));

        let waiting = load_scenario_graph(&project_root(), "lab4-broken-compat").unwrap();
        let rendered = format_graph(&waiting);
        assert!(rendered.contains("metadata-indexer waiting for image.tagged"));
    }

    #[test]
    fn build_graph_and_diff_cover_empty_and_removed_sections() {
        let service = ServiceContract {
            kind: "uma.service-contract".into(),
            spec_version: "1.0".into(),
            id: "single".into(),
            service_version: "1.0.0".into(),
            summary: "summary".into(),
            placements: vec!["cloud".into()],
            capabilities: vec!["cap".into()],
            capability_versions: vec![CapabilityVersion {
                id: "cap".into(),
                version: "1.0".into(),
            }],
            consumes: vec!["missing.event".into()],
            emits: Vec::new(),
            event_schemas: EventSchemas {
                consumes: vec![EventSchema {
                    name: "missing.event".into(),
                    schema: "contracts/schemas/event.json".into(),
                }],
                emits: Vec::new(),
            },
            io: IoSchemas {
                input_schema: "contracts/schemas/input.json".into(),
                output_schema: "contracts/schemas/output.json".into(),
            },
        };
        let graph = build_graph(&[service]);
        assert!(graph.edges.is_empty());
        assert_eq!(
            graph.waiting,
            vec![WaitingConsumer {
                service: "single".into(),
                event: "missing.event".into()
            }]
        );

        let stable = load_scenario_graph(&project_root(), "lab3-indexer").unwrap();
        let upload_only = load_scenario_graph(&project_root(), "lab1-upload-only").unwrap();
        let diff = diff_graphs(&stable, &upload_only);
        let rendered = format_graph_diff(&diff);
        assert!(rendered.contains("Removed services"));
        assert!(rendered.contains("Removed edges"));

        let empty = GraphDiff {
            from_scenario: "a".into(),
            to_scenario: "b".into(),
            added_services: Vec::new(),
            removed_services: Vec::new(),
            added_edges: Vec::new(),
            removed_edges: Vec::new(),
            added_waiting: Vec::new(),
            removed_waiting: Vec::new(),
        };
        let rendered = format_graph_diff(&empty);
        assert!(rendered.contains("Added services\n- none"));
        assert!(rendered.contains("Removed waiting consumers\n- none"));

        let waiting_services = vec![
            ServiceContract {
                kind: "uma.service-contract".into(),
                spec_version: "1.0".into(),
                id: "consumer-b".into(),
                service_version: "1.0.0".into(),
                summary: "summary".into(),
                placements: vec!["cloud".into()],
                capabilities: vec!["cap".into()],
                capability_versions: vec![CapabilityVersion {
                    id: "cap".into(),
                    version: "1.0".into(),
                }],
                consumes: vec!["missing.two".into()],
                emits: Vec::new(),
                event_schemas: EventSchemas {
                    consumes: vec![EventSchema {
                        name: "missing.two".into(),
                        schema: "contracts/schemas/event.json".into(),
                    }],
                    emits: Vec::new(),
                },
                io: IoSchemas {
                    input_schema: "contracts/schemas/input.json".into(),
                    output_schema: "contracts/schemas/output.json".into(),
                },
            },
            ServiceContract {
                kind: "uma.service-contract".into(),
                spec_version: "1.0".into(),
                id: "consumer-a".into(),
                service_version: "1.0.0".into(),
                summary: "summary".into(),
                placements: vec!["cloud".into()],
                capabilities: vec!["cap".into()],
                capability_versions: vec![CapabilityVersion {
                    id: "cap".into(),
                    version: "1.0".into(),
                }],
                consumes: vec!["missing.one".into()],
                emits: Vec::new(),
                event_schemas: EventSchemas {
                    consumes: vec![EventSchema {
                        name: "missing.one".into(),
                        schema: "contracts/schemas/event.json".into(),
                    }],
                    emits: Vec::new(),
                },
                io: IoSchemas {
                    input_schema: "contracts/schemas/input.json".into(),
                    output_schema: "contracts/schemas/output.json".into(),
                },
            },
        ];
        let multi_waiting = build_graph(&waiting_services);
        assert_eq!(
            multi_waiting
                .waiting
                .iter()
                .map(|item| format!("{} waiting for {}", item.service, item.event))
                .collect::<Vec<_>>(),
            vec![
                "consumer-a waiting for missing.one".to_string(),
                "consumer-b waiting for missing.two".to_string(),
            ]
        );

        let waiting_from = ScenarioGraph {
            scenario: "with-waiting".into(),
            graph: multi_waiting,
        };
        let waiting_to = ScenarioGraph {
            scenario: "no-waiting".into(),
            graph: Graph {
                services: waiting_services,
                edges: Vec::new(),
                waiting: Vec::new(),
            },
        };
        let rendered = format_graph_diff(&diff_graphs(&waiting_from, &waiting_to));
        assert!(rendered.contains("Removed waiting consumers"));
        assert!(rendered.contains("consumer-a waiting for missing.one"));
    }
}
