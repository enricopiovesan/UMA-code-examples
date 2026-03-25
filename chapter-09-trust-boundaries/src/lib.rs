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

fn err_string<E: ToString>(err: E) -> String {
    err.to_string()
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

fn resolve_project_root(cwd: Option<PathBuf>, manifest_dir: PathBuf) -> PathBuf {
    match cwd {
        Some(cwd) if cwd.join("scenarios").exists() && cwd.join("contracts").exists() => cwd,
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

fn load_policy(root_dir: &Path) -> Result<Policy, String> {
    let raw_contents = match fs::read_to_string(
        root_dir.join("contracts").join("policies").join("runtime-policy.json"),
    ) {
        Ok(contents) => contents,
        Err(err) => return Err(err_string(err)),
    };
    let raw: RawPolicy = match serde_json::from_str(&raw_contents) {
        Ok(policy) => policy,
        Err(err) => return Err(err_string(err)),
    };
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
    let mut found = false;
    for name in &scenarios {
        if name == scenario_name {
            found = true;
            break;
        }
    }
    if !found {
        return Err(format!(
            "unknown scenario \"{scenario_name}\". Available scenarios: {}",
            scenarios.join(", ")
        ));
    }

    let scenario_dir = root_dir.join("scenarios").join(scenario_name);
    let runtime_contents = match fs::read_to_string(scenario_dir.join("runtime.json")) {
        Ok(contents) => contents,
        Err(err) => return Err(err_string(err)),
    };
    let plan: RawScenarioPlan = match serde_json::from_str(&runtime_contents) {
        Ok(plan) => plan,
        Err(err) => return Err(err_string(err)),
    };
    ensure_non_empty(&plan.placement, "placement", "runtime.json")?;

    let mut services = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let entries = match fs::read_dir(scenario_dir.join("services")) {
        Ok(entries) => entries,
        Err(err) => return Err(err_string(err)),
    };
    for entry in entries.filter_map(Result::ok) {
        if !entry.path().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let relative = format!("services/{}", entry.file_name().to_string_lossy());
        let raw_contents = match fs::read_to_string(entry.path()) {
            Ok(contents) => contents,
            Err(err) => return Err(err_string(err)),
        };
        let raw: RawContract = match serde_json::from_str(&raw_contents) {
            Ok(value) => value,
            Err(err) => return Err(format!("{relative}: {err}")),
        };
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
    use serde_json::json;
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
        let dir = std::env::temp_dir().join(format!("chapter9-tests-{pid}-{nanos}-{counter}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_file(path: &Path, contents: &str) {
        let parent = path.parent().expect("write_file paths should have a parent");
        fs::create_dir_all(parent).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn service_contract(id: &str) -> ServiceContract {
        ServiceContract {
            kind: "uma.trusted-service-contract".to_string(),
            spec_version: "1.0".to_string(),
            id: id.to_string(),
            version: "1.0.0".to_string(),
            summary: "demo".to_string(),
            publisher: "uma.book.team".to_string(),
            trust_tier: "internal".to_string(),
            placements: vec!["cloud".to_string()],
            permissions: vec!["events.publish".to_string()],
            capabilities: vec![CapabilityVersion {
                id: "events.publish".to_string(),
                version: "1.0".to_string(),
            }],
            dependencies: vec![Dependency {
                name: "dep".to_string(),
                version: "1.0.0".to_string(),
                provenance: "verified".to_string(),
                checksum: "abc".to_string(),
            }],
            consumes: vec!["case.redacted".to_string()],
            emits: vec!["case.redacted".to_string()],
            event_schemas: EventSchemas {
                consumes: vec![EventSchema {
                    name: "case.redacted".to_string(),
                    schema: "contracts/schemas/event.json".to_string(),
                }],
                emits: vec![EventSchema {
                    name: "case.redacted".to_string(),
                    schema: "contracts/schemas/event.json".to_string(),
                }],
            },
            io: IoSchemas {
                input_schema: "contracts/schemas/input.json".to_string(),
                output_schema: "contracts/schemas/output.json".to_string(),
            },
        }
    }

    fn policy() -> Policy {
        Policy {
            trusted_publishers: vec!["uma.book.team".to_string()],
            allowed_trust_tiers: vec!["internal".to_string()],
            placement_rules: json!({
                "cloud": {"allowedPermissions": ["events.publish"]}
            }),
            event_rules: json!({
                "case.redacted": {"allowedConsumerTiers": ["internal"]}
            }),
        }
    }

    fn scenario_with_services(services: Vec<ServiceContract>) -> Scenario {
        Scenario {
            name: "test-scenario".to_string(),
            placement: "cloud".to_string(),
            executions: Vec::new(),
            communications: Vec::new(),
            services,
            policy: policy(),
        }
    }

    fn raw_contract() -> RawContract {
        RawContract {
            kind: "uma.trusted-service-contract".to_string(),
            spec_version: "1.0".to_string(),
            service: RawService {
                id: "case-redactor".to_string(),
                version: "1.0.0".to_string(),
                summary: "demo".to_string(),
                publisher: "uma.book.team".to_string(),
                trust_tier: "internal".to_string(),
                placements: vec!["cloud".to_string()],
            },
            capabilities: vec![RawCapability {
                id: "events.publish".to_string(),
                version: "1.0".to_string(),
            }],
            permissions: vec!["events.publish".to_string()],
            dependencies: vec![Dependency {
                name: "dep".to_string(),
                version: "1.0.0".to_string(),
                provenance: "verified".to_string(),
                checksum: "abc".to_string(),
            }],
            events: RawEvents {
                consumes: vec![RawEventContract {
                    name: "case.received".to_string(),
                    schema: "contracts/schemas/input.json".to_string(),
                }],
                emits: vec![RawEventContract {
                    name: "case.redacted".to_string(),
                    schema: "contracts/schemas/event.json".to_string(),
                }],
            },
            io: RawIo {
                input_schema: "contracts/schemas/input.json".to_string(),
                output_schema: "contracts/schemas/output.json".to_string(),
            },
        }
    }

    fn write_valid_runtime_policy(root: &Path) {
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "uma.runtime-trust-policy",
  "specVersion": "1.0",
  "trustedPublishers": ["uma.book.team"],
  "allowedTrustTiers": ["internal"],
  "placementRules": {"cloud": {"allowedPermissions": ["events.publish"]}},
  "eventRules": {"case.redacted": {"allowedConsumerTiers": ["internal"]}}
}"#,
        );
    }

    fn write_valid_schema_set(root: &Path) {
        write_file(&root.join("contracts/schemas/input.json"), "{}");
        write_file(&root.join("contracts/schemas/output.json"), "{}");
        write_file(&root.join("contracts/schemas/event.json"), "{}");
    }

    fn valid_service_json() -> &'static str {
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
    "consumes": [{"name": "case.received", "schema": "contracts/schemas/input.json"}],
    "emits": [{"name": "case.redacted", "schema": "contracts/schemas/event.json"}]
  },
  "io": {
    "inputSchema": "contracts/schemas/input.json",
    "outputSchema": "contracts/schemas/output.json"
  }
}"#
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
        let from =
            evaluate_trust(&load_scenario(&project_root(), "lab4-forbidden-communication").unwrap());
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
        let report =
            evaluate_trust(&load_scenario(&project_root(), "lab4-forbidden-communication").unwrap());
        assert!(report.audit_log.iter().any(|entry| {
            entry.kind == "communication"
                && entry.subject == "upload-bridge->partner-audit-sink"
                && entry.reason == "communication.forbidden"
        }));
    }

    #[test]
    fn project_root_prefers_current_directory_when_layout_matches() {
        let root = std::env::temp_dir().join("chapter9-cwd-layout");
        std::fs::create_dir_all(root.join("scenarios")).unwrap();
        std::fs::create_dir_all(root.join("contracts")).unwrap();
        assert_eq!(
            resolve_project_root(
                Some(root.clone()),
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            )
            .canonicalize()
            .unwrap(),
            root.canonicalize().unwrap()
        );
    }

    #[test]
    fn project_root_falls_back_to_manifest_directory() {
        let root = std::env::temp_dir().join("chapter9-cwd-fallback");
        std::fs::create_dir_all(&root).unwrap();
        assert_eq!(
            resolve_project_root(
                Some(root),
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            )
            .canonicalize()
            .unwrap(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap()
        );
    }

    #[test]
    fn missing_schema_is_rejected_while_loading_service() {
        let root = temp_root();
        write_valid_runtime_policy(&root);
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
        write_valid_runtime_policy(&root);
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        write_valid_schema_set(&root);
        let service = valid_service_json();
        write_file(&root.join("scenarios/lab1/services/one.json"), service);
        write_file(&root.join("scenarios/lab1/services/two.json"), service);

        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(err.contains("duplicate service id"));
    }

    #[test]
    fn helper_validation_rejects_empty_values_and_missing_schema() {
        let err = ensure_non_empty("   ", "kind", "service.json").unwrap_err();
        assert!(err.contains("\"kind\" must be a non-empty string"));

        let err = ensure_string_list(&["ok".to_string(), "".to_string()], "permissions", "service.json")
            .unwrap_err();
        assert!(err.contains("\"permissions\" must be a non-empty string"));

        let root = temp_root();
        let err = ensure_schema_exists(&root, "contracts/schemas/missing.json", "service.json").unwrap_err();
        assert!(err.contains("schema points to missing file"));
    }

    #[test]
    fn err_string_covers_supported_error_types() {
        let io_err = std::io::Error::other("io failure");
        assert!(err_string(io_err).contains("io failure"));

        let json_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        assert!(!err_string(json_err).is_empty());

        let strip_err = Path::new("/tmp").strip_prefix("/other").unwrap_err();
        assert!(!err_string(strip_err).is_empty());
    }

    #[test]
    fn validate_service_rejects_invalid_shapes() {
        let root = temp_root();
        write_valid_schema_set(&root);

        let mut wrong_kind = raw_contract();
        wrong_kind.kind = "wrong.kind".to_string();
        assert!(validate_service(wrong_kind, "service.json", &root)
            .unwrap_err()
            .contains("\"kind\" must be \"uma.trusted-service-contract\""));

        let mut duplicate_capability = raw_contract();
        duplicate_capability.capabilities.push(RawCapability {
            id: "events.publish".to_string(),
            version: "1.0".to_string(),
        });
        assert!(validate_service(duplicate_capability, "service.json", &root)
            .unwrap_err()
            .contains("duplicate capability"));

        let mut duplicate_consume = raw_contract();
        duplicate_consume.events.consumes.push(RawEventContract {
            name: "case.received".to_string(),
            schema: "contracts/schemas/input.json".to_string(),
        });
        assert!(validate_service(duplicate_consume, "service.json", &root)
            .unwrap_err()
            .contains("duplicate event"));

        let mut duplicate_emit = raw_contract();
        duplicate_emit.events.emits.push(RawEventContract {
            name: "case.redacted".to_string(),
            schema: "contracts/schemas/event.json".to_string(),
        });
        assert!(validate_service(duplicate_emit, "service.json", &root)
            .unwrap_err()
            .contains("duplicate event"));
    }

    #[test]
    fn validate_service_rejects_remaining_required_blank_fields() {
        let root = temp_root();
        write_valid_schema_set(&root);

        let service_cases: [(&str, fn(&mut RawContract), &str); 9] = [
            ("kind", |raw| raw.kind = " ".to_string(), "\"kind\""),
            (
                "specVersion",
                |raw| raw.spec_version = " ".to_string(),
                "\"specVersion\"",
            ),
            (
                "service.id",
                |raw| raw.service.id = " ".to_string(),
                "\"service.id\"",
            ),
            (
                "service.version",
                |raw| raw.service.version = " ".to_string(),
                "\"service.version\"",
            ),
            (
                "service.summary",
                |raw| raw.service.summary = " ".to_string(),
                "\"service.summary\"",
            ),
            (
                "service.publisher",
                |raw| raw.service.publisher = " ".to_string(),
                "\"service.publisher\"",
            ),
            (
                "service.trustTier",
                |raw| raw.service.trust_tier = " ".to_string(),
                "\"service.trustTier\"",
            ),
            (
                "service.placements",
                |raw| raw.service.placements[0] = " ".to_string(),
                "\"service.placements\"",
            ),
            (
                "permissions",
                |raw| raw.permissions[0] = " ".to_string(),
                "\"permissions\"",
            ),
        ];
        for (label, mutate, expected) in service_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let capability_cases: [(&str, fn(&mut RawContract), &str); 2] = [
            (
                "capabilities[].id",
                |raw| raw.capabilities[0].id = " ".to_string(),
                "\"capabilities[].id\"",
            ),
            (
                "capabilities[].version",
                |raw| raw.capabilities[0].version = " ".to_string(),
                "\"capabilities[].version\"",
            ),
        ];
        for (label, mutate, expected) in capability_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let dependency_cases: [(&str, fn(&mut RawContract), &str); 4] = [
            (
                "dependencies[].name",
                |raw| raw.dependencies[0].name = " ".to_string(),
                "\"dependencies[].name\"",
            ),
            (
                "dependencies[].version",
                |raw| raw.dependencies[0].version = " ".to_string(),
                "\"dependencies[].version\"",
            ),
            (
                "dependencies[].provenance",
                |raw| raw.dependencies[0].provenance = " ".to_string(),
                "\"dependencies[].provenance\"",
            ),
            (
                "dependencies[].checksum",
                |raw| raw.dependencies[0].checksum = " ".to_string(),
                "\"dependencies[].checksum\"",
            ),
        ];
        for (label, mutate, expected) in dependency_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let consume_cases: [(&str, fn(&mut RawContract), &str); 2] = [
            (
                "events.consumes[].name",
                |raw| raw.events.consumes[0].name = " ".to_string(),
                "\"events.consumes[].name\"",
            ),
            (
                "events.consumes[].schema",
                |raw| raw.events.consumes[0].schema = " ".to_string(),
                "\"events.consumes[].schema\"",
            ),
        ];
        for (label, mutate, expected) in consume_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let emit_cases: [(&str, fn(&mut RawContract), &str); 2] = [
            (
                "events.emits[].name",
                |raw| raw.events.emits[0].name = " ".to_string(),
                "\"events.emits[].name\"",
            ),
            (
                "events.emits[].schema",
                |raw| raw.events.emits[0].schema = " ".to_string(),
                "\"events.emits[].schema\"",
            ),
        ];
        for (label, mutate, expected) in emit_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let mut missing_consume_schema = raw_contract();
        missing_consume_schema.events.consumes[0].schema =
            "contracts/schemas/missing-consume.json".to_string();
        let error = validate_service(missing_consume_schema, "service.json", &root).unwrap_err();
        assert!(error.contains("missing file"));

        let io_cases: [(&str, fn(&mut RawContract), &str); 2] = [
            (
                "io.inputSchema",
                |raw| raw.io.input_schema = " ".to_string(),
                "\"io.inputSchema\"",
            ),
            (
                "io.outputSchema",
                |raw| raw.io.output_schema = " ".to_string(),
                "\"io.outputSchema\"",
            ),
        ];
        for (label, mutate, expected) in io_cases {
            let mut raw = raw_contract();
            mutate(&mut raw);
            let error = validate_service(raw, "service.json", &root).unwrap_err();
            assert!(error.contains(expected), "{label}: {error}");
        }

        let mut missing_input_schema = raw_contract();
        missing_input_schema.io.input_schema = "contracts/schemas/missing-input.json".to_string();
        let error = validate_service(missing_input_schema, "service.json", &root).unwrap_err();
        assert!(error.contains("missing file"));

        let mut missing_output_schema = raw_contract();
        missing_output_schema.io.output_schema = "contracts/schemas/missing-output.json".to_string();
        let error = validate_service(missing_output_schema, "service.json", &root).unwrap_err();
        assert!(error.contains("missing file"));
    }

    #[test]
    fn list_scenarios_ignores_files_and_unknown_scenario_is_reported() {
        let root = temp_root();
        fs::create_dir_all(root.join("scenarios/alpha")).unwrap();
        fs::create_dir_all(root.join("scenarios/beta")).unwrap();
        write_file(&root.join("scenarios/readme.txt"), "ignore");

        let listed = list_scenarios(&root).unwrap();
        assert_eq!(listed, vec!["alpha", "beta"]);

        let err = load_scenario(&root, "missing").unwrap_err();
        assert!(err.contains("unknown scenario"));
        assert!(err.contains("alpha, beta"));
    }

    #[test]
    fn load_policy_rejects_blank_lists_and_missing_file() {
        let root = temp_root();
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "uma.runtime-trust-policy",
  "specVersion": "1.0",
  "trustedPublishers": [""],
  "allowedTrustTiers": ["internal"],
  "placementRules": {},
  "eventRules": {}
}"#,
        );
        let error = load_policy(&root).unwrap_err();
        assert!(error.contains("\"trustedPublishers\""));

        let root = temp_root();
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "uma.runtime-trust-policy",
  "specVersion": "1.0",
  "trustedPublishers": ["uma.book.team"],
  "allowedTrustTiers": [""],
  "placementRules": {},
  "eventRules": {}
}"#,
        );
        let error = load_policy(&root).unwrap_err();
        assert!(error.contains("\"allowedTrustTiers\""));

        let missing_root = temp_root();
        let error = load_policy(&missing_root).unwrap_err();
        assert!(!error.is_empty());

        let invalid_json_root = temp_root();
        write_file(
            &invalid_json_root.join("contracts/policies/runtime-policy.json"),
            "{ invalid",
        );
        let error = load_policy(&invalid_json_root).unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn load_scenario_ignores_service_directories_and_non_json_files() {
        let root = temp_root();
        write_valid_runtime_policy(&root);
        write_valid_schema_set(&root);
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        fs::create_dir_all(root.join("scenarios/lab1/services/archive")).unwrap();
        write_file(&root.join("scenarios/lab1/services/readme.txt"), "ignore");
        write_file(
            &root.join("scenarios/lab1/services/service.json"),
            valid_service_json(),
        );

        let scenario = load_scenario(&root, "lab1").unwrap();
        assert_eq!(scenario.services.len(), 1);
        assert_eq!(scenario.services[0].id, "case-redactor");
    }

    #[test]
    fn load_scenario_reports_invalid_service_json_with_relative_path() {
        let root = temp_root();
        write_valid_runtime_policy(&root);
        write_valid_schema_set(&root);
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        write_file(
            &root.join("scenarios/lab1/services/broken.json"),
            "{ invalid json",
        );

        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(err.contains("services/broken.json"));
    }

    #[test]
    fn load_scenario_covers_runtime_and_policy_error_paths() {
        let root = temp_root();
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());

        let root = temp_root();
        fs::create_dir_all(root.join("scenarios/lab1")).unwrap();
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());

        let root = temp_root();
        write_file(&root.join("scenarios/lab1/runtime.json"), "{ invalid");
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());

        let root = temp_root();
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":" ","executions":[],"communications":[]}"#,
        );
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(err.contains("\"placement\""));

        let root = temp_root();
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());

        let root = temp_root();
        write_valid_schema_set(&root);
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        write_file(
            &root.join("scenarios/lab1/services/service.json"),
            valid_service_json(),
        );
        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn load_scenario_errors_when_service_file_is_unreadable() {
        let root = temp_root();
        write_valid_runtime_policy(&root);
        write_valid_schema_set(&root);
        write_file(
            &root.join("scenarios/lab1/runtime.json"),
            r#"{"placement":"cloud","executions":[],"communications":[]}"#,
        );
        let service_path = root.join("scenarios/lab1/services/service.json");
        write_file(&service_path, valid_service_json());

        let mut permissions = fs::metadata(&service_path).unwrap().permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(&service_path, permissions).unwrap();

        let err = load_scenario(&root, "lab1").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn load_policy_rejects_invalid_kind() {
        let root = temp_root();
        write_file(
            &root.join("contracts/policies/runtime-policy.json"),
            r#"{
  "kind": "wrong.kind",
  "specVersion": "1.0",
  "trustedPublishers": ["uma.book.team"],
  "allowedTrustTiers": ["internal"],
  "placementRules": {},
  "eventRules": {}
}"#,
        );

        let err = load_policy(&root).unwrap_err();
        assert!(err.contains("invalid policy kind"));
    }

    #[test]
    fn permission_and_tier_rules_default_to_false_when_missing() {
        let policy = policy();
        assert!(allow_permission(&policy, "cloud", "events.publish"));
        assert!(!allow_permission(&policy, "edge", "events.publish"));
        assert!(!allow_permission(&policy, "cloud", "db.write"));

        assert!(allow_consumer_tier(&policy, "case.redacted", "internal"));
        assert!(!allow_consumer_tier(&policy, "case.redacted", "partner"));
        assert!(!allow_consumer_tier(&policy, "missing.event", "internal"));
    }

    #[test]
    fn evaluate_execution_covers_all_denial_reasons() {
        let request = ExecutionRequest {
            service: "case-redactor".to_string(),
            requested_permissions: vec!["events.publish".to_string()],
        };

        let mut wrong_placement = service_contract("case-redactor");
        wrong_placement.placements = vec!["edge".to_string()];
        assert_eq!(
            evaluate_execution(&wrong_placement, &request, &scenario_with_services(vec![])).reason,
            "placement.forbidden"
        );

        let mut untrusted_publisher = service_contract("case-redactor");
        untrusted_publisher.publisher = "other.publisher".to_string();
        assert_eq!(
            evaluate_execution(&untrusted_publisher, &request, &scenario_with_services(vec![])).reason,
            "publisher.untrusted"
        );

        let mut blocked_tier = service_contract("case-redactor");
        blocked_tier.trust_tier = "partner".to_string();
        assert_eq!(
            evaluate_execution(&blocked_tier, &request, &scenario_with_services(vec![])).reason,
            "trust_tier.blocked"
        );

        let mut bad_provenance = service_contract("case-redactor");
        bad_provenance.dependencies[0].provenance = "unverified".to_string();
        assert_eq!(
            evaluate_execution(&bad_provenance, &request, &scenario_with_services(vec![])).reason,
            "dependency.provenance.untrusted"
        );

        let mut missing_checksum = service_contract("case-redactor");
        missing_checksum.dependencies[0].checksum = "   ".to_string();
        assert_eq!(
            evaluate_execution(&missing_checksum, &request, &scenario_with_services(vec![])).reason,
            "dependency.checksum.missing"
        );

        let undeclared = ExecutionRequest {
            service: "case-redactor".to_string(),
            requested_permissions: vec!["db.write".to_string()],
        };
        assert_eq!(
            evaluate_execution(&service_contract("case-redactor"), &undeclared, &scenario_with_services(vec![]))
                .reason,
            "permission.undeclared"
        );

        let forbidden = ExecutionRequest {
            service: "case-redactor".to_string(),
            requested_permissions: vec!["events.publish".to_string(), "db.write".to_string()],
        };
        let mut service = service_contract("case-redactor");
        service.permissions.push("db.write".to_string());
        assert_eq!(
            evaluate_execution(&service, &forbidden, &scenario_with_services(vec![])).reason,
            "permission.forbidden"
        );

        assert_eq!(
            evaluate_execution(&service_contract("case-redactor"), &request, &scenario_with_services(vec![])).reason,
            "execution.trusted"
        );
    }

    #[test]
    fn evaluate_communication_covers_all_denial_reasons() {
        let request = CommunicationRequest {
            from: "source".to_string(),
            event: "case.redacted".to_string(),
            to: "target".to_string(),
        };
        let mut source = service_contract("source");
        let target = service_contract("target");

        source.emits.clear();
        assert_eq!(
            evaluate_communication(&source, &target, &request, &[], &scenario_with_services(vec![])).reason,
            "event.not_emitted"
        );

        let source = service_contract("source");
        let mut target_not_consuming = service_contract("target");
        target_not_consuming.consumes.clear();
        assert_eq!(
            evaluate_communication(&source, &target_not_consuming, &request, &[], &scenario_with_services(vec![]))
                .reason,
            "event.not_consumed"
        );

        let source = service_contract("source");
        let target = service_contract("target");
        let execution_log = vec![audit_entry("execution", "source", "allow", "execution.trusted")];
        assert_eq!(
            evaluate_communication(&source, &target, &request, &execution_log, &scenario_with_services(vec![])).reason,
            "execution.not_trusted"
        );

        let source = service_contract("source");
        let mut target_blocked = service_contract("target");
        target_blocked.trust_tier = "partner".to_string();
        let execution_log = vec![
            audit_entry("execution", "source", "allow", "execution.trusted"),
            audit_entry("execution", "target", "allow", "execution.trusted"),
        ];
        assert_eq!(
            evaluate_communication(
                &source,
                &target_blocked,
                &request,
                &execution_log,
                &scenario_with_services(vec![])
            )
            .reason,
            "communication.forbidden"
        );

        let source = service_contract("source");
        let target = service_contract("target");
        assert_eq!(
            evaluate_communication(&source, &target, &request, &execution_log, &scenario_with_services(vec![])).reason,
            "communication.trusted"
        );
    }

    #[test]
    fn evaluate_trust_handles_missing_services_and_formats_reports() {
        let scenario = Scenario {
            name: "manual".to_string(),
            placement: "cloud".to_string(),
            executions: vec![ExecutionRequest {
                service: "missing-service".to_string(),
                requested_permissions: vec![],
            }],
            communications: vec![CommunicationRequest {
                from: "missing-source".to_string(),
                event: "case.redacted".to_string(),
                to: "missing-target".to_string(),
            }],
            services: vec![service_contract("existing")],
            policy: policy(),
        };

        let report = evaluate_trust(&scenario);
        assert_eq!(report.outcome, "deny");
        assert!(report.audit_log.iter().any(|entry| entry.reason == "service.not_found"));

        let formatted = format_report(&report);
        assert!(formatted.contains("Scenario: manual"));
        assert!(formatted.contains("Audit Log"));
        assert!(formatted.contains("publisher: uma.book.team"));
    }

    #[test]
    fn format_trust_diff_covers_empty_and_non_empty_sections() {
        let empty = TrustDiff {
            from_scenario: "a".to_string(),
            to_scenario: "b".to_string(),
            from_outcome: "allow".to_string(),
            to_outcome: "allow".to_string(),
            added: Vec::new(),
            removed: Vec::new(),
        };
        let empty_text = format_trust_diff(&empty);
        assert!(empty_text.contains("Added trust decisions"));
        assert!(empty_text.contains("- none"));

        let populated = TrustDiff {
            from_scenario: "a".to_string(),
            to_scenario: "b".to_string(),
            from_outcome: "deny".to_string(),
            to_outcome: "allow".to_string(),
            added: vec!["allow:execution:svc:execution.trusted".to_string()],
            removed: vec!["deny:execution:svc:publisher.untrusted".to_string()],
        };
        let populated_text = format_trust_diff(&populated);
        assert!(populated_text.contains("allow:execution:svc:execution.trusted"));
        assert!(populated_text.contains("deny:execution:svc:publisher.untrusted"));
    }
}
