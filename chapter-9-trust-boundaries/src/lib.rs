use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawPolicy {
    kind: String,
    #[serde(rename = "specVersion")]
    spec_version: String,
    #[serde(rename = "trustedPublishers")]
    trusted_publishers: Vec<String>,
    #[serde(rename = "allowedTrustTiers")]
    allowed_trust_tiers: Vec<String>,
    #[serde(rename = "placementRules")]
    placement_rules: serde_json::Value,
    #[serde(rename = "eventRules")]
    event_rules: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct RawScenarioPlan {
    placement: String,
    executions: Vec<ExecutionRequest>,
    communications: Vec<CommunicationRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionRequest {
    service: String,
    #[serde(rename = "requestedPermissions")]
    requested_permissions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommunicationRequest {
    from: String,
    event: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct RawContract {
    kind: String,
    #[serde(rename = "specVersion")]
    spec_version: String,
    service: RawService,
    capabilities: Vec<RawCapability>,
    permissions: Vec<String>,
    dependencies: Vec<Dependency>,
    events: RawEvents,
    io: RawIo,
}

#[derive(Debug, Deserialize)]
struct RawService {
    id: String,
    version: String,
    summary: String,
    publisher: String,
    #[serde(rename = "trustTier")]
    trust_tier: String,
    placements: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct Dependency {
    name: String,
    version: String,
    provenance: String,
    checksum: String,
}

#[derive(Debug, Deserialize)]
struct RawCapability {
    id: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CapabilityVersion {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct RawEvents {
    consumes: Vec<RawEventContract>,
    emits: Vec<RawEventContract>,
}

#[derive(Debug, Deserialize, Clone)]
struct RawEventContract {
    name: String,
    schema: String,
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

#[derive(Debug, Deserialize)]
struct RawIo {
    #[serde(rename = "inputSchema")]
    input_schema: String,
    #[serde(rename = "outputSchema")]
    output_schema: String,
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
    pub version: String,
    pub summary: String,
    pub publisher: String,
    #[serde(rename = "trustTier")]
    pub trust_tier: String,
    pub placements: Vec<String>,
    pub permissions: Vec<String>,
    pub capabilities: Vec<CapabilityVersion>,
    pub dependencies: Vec<Dependency>,
    pub consumes: Vec<String>,
    pub emits: Vec<String>,
    #[serde(rename = "eventSchemas")]
    pub event_schemas: EventSchemas,
    pub io: IoSchemas,
}

#[derive(Debug, Clone)]
pub struct Scenario {
    pub name: String,
    pub placement: String,
    pub executions: Vec<ExecutionRequest>,
    pub communications: Vec<CommunicationRequest>,
    pub services: Vec<ServiceContract>,
    policy: Policy,
}

#[derive(Debug, Clone)]
struct Policy {
    trusted_publishers: Vec<String>,
    allowed_trust_tiers: Vec<String>,
    placement_rules: serde_json::Value,
    event_rules: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AuditEntry {
    pub kind: String,
    pub subject: String,
    pub decision: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TrustReport {
    pub scenario: String,
    pub placement: String,
    pub outcome: String,
    pub services: Vec<ServiceContract>,
    #[serde(rename = "auditLog")]
    pub audit_log: Vec<AuditEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustDiff {
    pub from_scenario: String,
    pub to_scenario: String,
    pub from_outcome: String,
    pub to_outcome: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
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

fn ensure_schema_exists(root_dir: &Path, schema: &str, label: &str) -> Result<(), String> {
    if !root_dir.join(schema).exists() {
        return Err(format!("{label}: schema points to missing file \"{schema}\""));
    }
    Ok(())
}

fn validate_service(raw: RawContract, label: &str, root_dir: &Path) -> Result<ServiceContract, String> {
    ensure_non_empty(&raw.kind, "kind", label)?;
    ensure_non_empty(&raw.spec_version, "specVersion", label)?;
    if raw.kind != "uma.trusted-service-contract" {
        return Err(format!(
            "{label}: \"kind\" must be \"uma.trusted-service-contract\""
        ));
    }

    ensure_non_empty(&raw.service.id, "service.id", label)?;
    ensure_non_empty(&raw.service.version, "service.version", label)?;
    ensure_non_empty(&raw.service.summary, "service.summary", label)?;
    ensure_non_empty(&raw.service.publisher, "service.publisher", label)?;
    ensure_non_empty(&raw.service.trust_tier, "service.trustTier", label)?;
    ensure_string_list(&raw.service.placements, "service.placements", label)?;
    ensure_string_list(&raw.permissions, "permissions", label)?;

    let mut capability_seen = BTreeSet::new();
    for capability in &raw.capabilities {
        ensure_non_empty(&capability.id, "capabilities[].id", label)?;
        ensure_non_empty(&capability.version, "capabilities[].version", label)?;
        let key = format!("{}@{}", capability.id, capability.version);
        if !capability_seen.insert(key.clone()) {
            return Err(format!("{label}: duplicate capability \"{key}\""));
        }
    }

    for dependency in &raw.dependencies {
        ensure_non_empty(&dependency.name, "dependencies[].name", label)?;
        ensure_non_empty(&dependency.version, "dependencies[].version", label)?;
        ensure_non_empty(&dependency.provenance, "dependencies[].provenance", label)?;
        ensure_non_empty(&dependency.checksum, "dependencies[].checksum", label)?;
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
        version: raw.service.version,
        summary: raw.service.summary,
        publisher: raw.service.publisher,
        trust_tier: raw.service.trust_tier,
        placements: raw.service.placements,
        permissions: raw.permissions,
        capabilities: raw
            .capabilities
            .into_iter()
            .map(|item| CapabilityVersion {
                id: item.id,
                version: item.version,
            })
            .collect(),
        dependencies: raw.dependencies,
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

fn load_policy(root_dir: &Path) -> Result<Policy, String> {
    let raw: RawPolicy = serde_json::from_str(
        &fs::read_to_string(root_dir.join("contracts").join("policies").join("runtime-policy.json"))
            .map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())?;
    if raw.kind != "uma.runtime-trust-policy" {
        return Err("runtime-policy.json: invalid policy kind".to_string());
    }
    ensure_string_list(&raw.trusted_publishers, "trustedPublishers", "runtime-policy.json")?;
    ensure_string_list(&raw.allowed_trust_tiers, "allowedTrustTiers", "runtime-policy.json")?;
    let _ = raw.spec_version;
    Ok(Policy {
        trusted_publishers: raw.trusted_publishers,
        allowed_trust_tiers: raw.allowed_trust_tiers,
        placement_rules: raw.placement_rules,
        event_rules: raw.event_rules,
    })
}

pub fn load_scenario(root_dir: &Path, scenario_name: &str) -> Result<Scenario, String> {
    let scenarios = list_scenarios(root_dir)?;
    if !scenarios.iter().any(|name| name == scenario_name) {
        return Err(format!(
            "unknown scenario \"{scenario_name}\". Available scenarios: {}",
            scenarios.join(", ")
        ));
    }

    let scenario_dir = root_dir.join("scenarios").join(scenario_name);
    let plan: RawScenarioPlan = serde_json::from_str(
        &fs::read_to_string(scenario_dir.join("runtime.json")).map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())?;
    ensure_non_empty(&plan.placement, "placement", "runtime.json")?;

    let mut services = Vec::new();
    let mut seen_ids = BTreeSet::new();
    for entry in fs::read_dir(scenario_dir.join("services")).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if !entry.file_type().map_err(|err| err.to_string())?.is_file() {
            continue;
        }
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&scenario_dir)
            .map_err(|err| err.to_string())?
            .display()
            .to_string();
        let raw: RawContract =
            serde_json::from_str(&fs::read_to_string(entry.path()).map_err(|err| err.to_string())?)
                .map_err(|err| format!("{relative}: {err}"))?;
        let service = validate_service(raw, &relative, root_dir)?;
        if !seen_ids.insert(service.id.clone()) {
            return Err(format!(
                "duplicate service id \"{}\" in scenarios/{scenario_name}",
                service.id
            ));
        }
        services.push(service);
    }
    services.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(Scenario {
        name: scenario_name.to_string(),
        placement: plan.placement,
        executions: plan.executions,
        communications: plan.communications,
        services,
        policy: load_policy(root_dir)?,
    })
}

fn allow_permission(policy: &Policy, placement: &str, permission: &str) -> bool {
    policy
        .placement_rules
        .get(placement)
        .and_then(|value| value.get("allowedPermissions"))
        .and_then(|value| value.as_array())
        .map(|values| values.iter().any(|value| value.as_str() == Some(permission)))
        .unwrap_or(false)
}

fn allow_consumer_tier(policy: &Policy, event: &str, trust_tier: &str) -> bool {
    policy
        .event_rules
        .get(event)
        .and_then(|value| value.get("allowedConsumerTiers"))
        .and_then(|value| value.as_array())
        .map(|values| values.iter().any(|value| value.as_str() == Some(trust_tier)))
        .unwrap_or(false)
}

fn audit_entry(kind: &str, subject: &str, decision: &str, reason: &str) -> AuditEntry {
    AuditEntry {
        kind: kind.to_string(),
        subject: subject.to_string(),
        decision: decision.to_string(),
        reason: reason.to_string(),
    }
}

fn evaluate_execution(service: &ServiceContract, request: &ExecutionRequest, scenario: &Scenario) -> AuditEntry {
    if !service.placements.iter().any(|value| value == &scenario.placement) {
        return audit_entry("execution", &service.id, "deny", "placement.forbidden");
    }
    if !scenario
        .policy
        .trusted_publishers
        .iter()
        .any(|value| value == &service.publisher)
    {
        return audit_entry("execution", &service.id, "deny", "publisher.untrusted");
    }
    if !scenario
        .policy
        .allowed_trust_tiers
        .iter()
        .any(|value| value == &service.trust_tier)
    {
        return audit_entry("execution", &service.id, "deny", "trust_tier.blocked");
    }
    for dependency in &service.dependencies {
        if dependency.provenance != "verified" {
            return audit_entry(
                "execution",
                &service.id,
                "deny",
                "dependency.provenance.untrusted",
            );
        }
        if dependency.checksum.trim().is_empty() {
            return audit_entry("execution", &service.id, "deny", "dependency.checksum.missing");
        }
    }
    for permission in &request.requested_permissions {
        if !service.permissions.iter().any(|value| value == permission) {
            return audit_entry("execution", &service.id, "deny", "permission.undeclared");
        }
        if !allow_permission(&scenario.policy, &scenario.placement, permission) {
            return audit_entry("execution", &service.id, "deny", "permission.forbidden");
        }
    }
    audit_entry("execution", &service.id, "allow", "execution.trusted")
}

fn evaluate_communication(
    source: &ServiceContract,
    target: &ServiceContract,
    request: &CommunicationRequest,
    execution_log: &[AuditEntry],
    scenario: &Scenario,
) -> AuditEntry {
    if !source.emits.iter().any(|value| value == &request.event) {
        return audit_entry("communication", &format!("{}->{}", source.id, target.id), "deny", "event.not_emitted");
    }
    if !target.consumes.iter().any(|value| value == &request.event) {
        return audit_entry("communication", &format!("{}->{}", source.id, target.id), "deny", "event.not_consumed");
    }

    let source_allowed = execution_log
        .iter()
        .any(|entry| entry.subject == source.id && entry.decision == "allow");
    let target_allowed = execution_log
        .iter()
        .any(|entry| entry.subject == target.id && entry.decision == "allow");
    if !source_allowed || !target_allowed {
        return audit_entry(
            "communication",
            &format!("{}->{}", source.id, target.id),
            "deny",
            "execution.not_trusted",
        );
    }

    if !allow_consumer_tier(&scenario.policy, &request.event, &target.trust_tier) {
        return audit_entry(
            "communication",
            &format!("{}->{}", source.id, target.id),
            "deny",
            "communication.forbidden",
        );
    }

    audit_entry(
        "communication",
        &format!("{}->{}", source.id, target.id),
        "allow",
        "communication.trusted",
    )
}

pub fn evaluate_trust(scenario: &Scenario) -> TrustReport {
    let mut audit_log = Vec::new();

    for request in &scenario.executions {
        match scenario.services.iter().find(|service| service.id == request.service) {
            Some(service) => audit_log.push(evaluate_execution(service, request, scenario)),
            None => audit_log.push(audit_entry(
                "execution",
                &request.service,
                "deny",
                "service.not_found",
            )),
        }
    }

    let execution_log = audit_log.clone();
    for request in &scenario.communications {
        let source = scenario.services.iter().find(|service| service.id == request.from);
        let target = scenario.services.iter().find(|service| service.id == request.to);
        match (source, target) {
            (Some(source), Some(target)) => {
                audit_log.push(evaluate_communication(source, target, request, &execution_log, scenario))
            }
            _ => audit_log.push(audit_entry(
                "communication",
                &format!("{}->{}", request.from, request.to),
                "deny",
                "service.not_found",
            )),
        }
    }

    let outcome = if audit_log.iter().any(|entry| entry.decision == "deny") {
        "deny"
    } else {
        "allow"
    };

    TrustReport {
        scenario: scenario.name.clone(),
        placement: scenario.placement.clone(),
        outcome: outcome.to_string(),
        services: scenario.services.clone(),
        audit_log,
    }
}

pub fn format_report(report: &TrustReport) -> String {
    let mut out = String::new();
    writeln!(&mut out, "Scenario: {}", report.scenario).unwrap();
    writeln!(&mut out, "Placement: {}", report.placement).unwrap();
    writeln!(&mut out, "Outcome: {}", report.outcome).unwrap();
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Services").unwrap();
    for service in &report.services {
        writeln!(&mut out, "- {} v{}", service.id, service.version).unwrap();
        writeln!(&mut out, "  publisher: {}", service.publisher).unwrap();
        writeln!(&mut out, "  trust tier: {}", service.trust_tier).unwrap();
        writeln!(&mut out, "  placements: {}", service.placements.join(", ")).unwrap();
        writeln!(&mut out, "  permissions: {}", service.permissions.join(", ")).unwrap();
    }
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Audit Log").unwrap();
    for entry in &report.audit_log {
        writeln!(
            &mut out,
            "- [{}] {} {}: {}",
            entry.decision, entry.kind, entry.subject, entry.reason
        )
        .unwrap();
    }
    out
}

pub fn diff_reports(from: &TrustReport, to: &TrustReport) -> TrustDiff {
    let left: BTreeSet<String> = from
        .audit_log
        .iter()
        .map(|entry| format!("{}:{}:{}:{}", entry.decision, entry.kind, entry.subject, entry.reason))
        .collect();
    let right: BTreeSet<String> = to
        .audit_log
        .iter()
        .map(|entry| format!("{}:{}:{}:{}", entry.decision, entry.kind, entry.subject, entry.reason))
        .collect();
    TrustDiff {
        from_scenario: from.scenario.clone(),
        to_scenario: to.scenario.clone(),
        from_outcome: from.outcome.clone(),
        to_outcome: to.outcome.clone(),
        added: right.difference(&left).cloned().collect(),
        removed: left.difference(&right).cloned().collect(),
    }
}

pub fn format_trust_diff(diff: &TrustDiff) -> String {
    let mut out = String::new();
    writeln!(
        &mut out,
        "Trust diff: {} -> {}",
        diff.from_scenario, diff.to_scenario
    )
    .unwrap();
    writeln!(
        &mut out,
        "Outcome: {} -> {}",
        diff.from_outcome, diff.to_outcome
    )
    .unwrap();
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Added trust decisions").unwrap();
    if diff.added.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for value in &diff.added {
            writeln!(&mut out, "- {value}").unwrap();
        }
    }
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "Removed trust decisions").unwrap();
    if diff.removed.is_empty() {
        writeln!(&mut out, "- none").unwrap();
    } else {
        for value in &diff.removed {
            writeln!(&mut out, "- {value}").unwrap();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("chapter9-tests-{nanos}"));
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
    fn lab1_is_allowed() {
        let scenario = load_scenario(&project_root(), "lab1-trusted-service").unwrap();
        let report = evaluate_trust(&scenario);
        assert_eq!(report.outcome, "allow");
        assert_eq!(report.audit_log[0].reason, "execution.trusted");
    }

    #[test]
    fn lab4_to_lab5_diff_restores_communication() {
        let from = evaluate_trust(&load_scenario(&project_root(), "lab4-forbidden-communication").unwrap());
        let to = evaluate_trust(&load_scenario(&project_root(), "lab5-restored-compliance").unwrap());
        let diff = diff_reports(&from, &to);
        assert_eq!(diff.from_outcome, "deny");
        assert_eq!(diff.to_outcome, "allow");
        assert!(diff
            .added
            .iter()
            .any(|item| item.contains("communication:upload-bridge->internal-audit-sink:communication.trusted")));
    }

    #[test]
    fn scenario_list_is_stable() {
        assert_eq!(
            list_scenarios(&project_root()).unwrap(),
            vec![
                "lab1-trusted-service",
                "lab2-undeclared-permission",
                "lab3-untrusted-dependency",
                "lab4-forbidden-communication",
                "lab5-restored-compliance"
            ]
        );
    }

    #[test]
    fn undeclared_permission_is_denied() {
        let report = evaluate_trust(&load_scenario(&project_root(), "lab2-undeclared-permission").unwrap());
        assert_eq!(report.outcome, "deny");
        assert!(report
            .audit_log
            .iter()
            .any(|entry| entry.reason == "permission.undeclared"));
    }

    #[test]
    fn forbidden_partner_communication_is_denied() {
        let report = evaluate_trust(&load_scenario(&project_root(), "lab4-forbidden-communication").unwrap());
        assert!(report.audit_log.iter().any(|entry| {
            entry.kind == "communication"
                && entry.subject == "upload-bridge->partner-audit-sink"
                && entry.reason == "communication.forbidden"
        }));
    }

    #[test]
    fn missing_schema_is_rejected_while_loading_service() {
        let root = temp_root();
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "uma.runtime-trust-policy",
  "specVersion": "1.0",
  "trustedPublishers": ["uma.book.team"],
  "allowedTrustTiers": ["internal"],
  "placementRules": {"cloud": {"allowedPermissions": ["events.publish"]}},
  "eventRules": {}
}"#,
        );
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");
        write_file(
            &root.join("scenarios/lab1/services/service.json"),
            r#"{
  "kind": "uma.trusted-service-contract",
  "specVersion": "1.0",
  "service": {
    "id": "case-redactor",
    "version": "1.0.0",
    "summary": "demo",
    "publisher": "uma.book.team",
    "trustTier": "internal",
    "placements": ["cloud"]
  },
  "capabilities": [{"id": "events.publish", "version": "1.0"}],
  "permissions": ["events.publish"],
  "dependencies": [{"name":"dep","version":"1.0.0","provenance":"verified","checksum":"abc"}],
  "events": {
    "consumes": [],
    "emits": [{"name": "case.redacted", "schema": "contracts/schemas/missing.json"}]
  },
  "io": {
    "inputSchema": "contracts/schemas/input.json",
    "outputSchema": "contracts/schemas/output.json"
  }
}"#,
        );

        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(err.contains("missing file"));
        assert!(err.contains("contracts/schemas/missing.json"));
    }

    #[test]
    fn duplicate_service_ids_are_rejected() {
        let root = temp_root();
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "uma.runtime-trust-policy",
  "specVersion": "1.0",
  "trustedPublishers": ["uma.book.team"],
  "allowedTrustTiers": ["internal"],
  "placementRules": {"cloud": {"allowedPermissions": ["events.publish"]}},
  "eventRules": {}
}"#,
        );
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");
        write_file(&root.join("contracts/schemas/event.json"), "{}");
        let service = r#"{
  "kind": "uma.trusted-service-contract",
  "specVersion": "1.0",
  "service": {
    "id": "case-redactor",
    "version": "1.0.0",
    "summary": "demo",
    "publisher": "uma.book.team",
    "trustTier": "internal",
    "placements": ["cloud"]
  },
  "capabilities": [{"id": "events.publish", "version": "1.0"}],
  "permissions": ["events.publish"],
  "dependencies": [{"name":"dep","version":"1.0.0","provenance":"verified","checksum":"abc"}],
  "events": {
    "consumes": [],
    "emits": [{"name": "case.redacted", "schema": "contracts/schemas/event.json"}]
  },
  "io": {
    "inputSchema": "contracts/schemas/input.json",
    "outputSchema": "contracts/schemas/output.json"
  }
}"#;
        write_file(&root.join("scenarios/lab1/services/one.json"), service);
        write_file(&root.join("scenarios/lab1/services/two.json"), service);

        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(err.contains("duplicate service id"));
    }
}
