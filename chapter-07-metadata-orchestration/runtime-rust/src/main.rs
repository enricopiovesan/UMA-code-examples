use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

static EVENT_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Deserialize)]
struct Contract {
    name: String,
    version: String,
    #[serde(default)]
    events: ContractEvents,
    #[serde(default)]
    constraints: ContractConstraints,
}

#[derive(Debug, Default, Deserialize)]
struct ContractEvents {
    #[serde(default)]
    emits: Vec<EventSpec>,
    #[serde(default)]
    subscribes: Vec<SubscribeSpec>,
}

#[derive(Debug, Deserialize)]
struct EventSpec {
    name: String,
    #[allow(dead_code)]
    schema: String,
}

#[derive(Debug, Deserialize)]
struct SubscribeSpec {
    pattern: String,
}

#[derive(Debug, Default, Deserialize)]
struct ContractConstraints {
    #[serde(default)]
    placement: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PolicyFile {
    #[serde(default)]
    deny: Vec<PolicyRule>,
}

#[derive(Debug, Deserialize)]
struct PolicyRule {
    rule: String,
    #[serde(rename = "if")]
    condition: PolicyCondition,
}

#[derive(Debug, Deserialize)]
struct PolicyCondition {
    service: String,
    placement: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TaggerInput {
    id: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImageAnalyzed {
    id: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ValidationStatus {
    source: String,
    event: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct EvaluatorOutput {
    id: String,
    score: f64,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    ensure_log_dirs()?;
    let root = project_root()?;

    let policy_digest = sha256_file(root.join("contracts/schemas/policy.standard.v1.json"))?;
    println!("[info] policy.digest {policy_digest}");

    let tagger = load_contract(root.join("contracts/image.tagger.contract.yaml"))?;
    let logger = load_contract(root.join("contracts/telemetry.logger.contract.yaml"))?;
    let edge_cache = load_contract(root.join("contracts/edge.cache.contract.yaml"))?;
    let evaluator = load_contract(root.join("contracts/ai.model.evaluator.contract.yaml"))?;

    let policy_check = enforce_policy(&root, &evaluator)?;
    let fail_mode = std::env::var("POLICY_FAIL_MODE").unwrap_or_else(|_| "closed".to_string());
    if let Some(reason) = policy_check {
        if fail_mode == "closed" {
            eprintln!("[error] policy.violation {reason}");
            std::process::exit(4);
        }
        println!("[warn] policy.violation {reason} continuing due to fail-open");
    }

    print_binding(&tagger, &logger);
    print_binding(&tagger, &edge_cache);
    print_binding(&tagger, &evaluator);

    let input = TaggerInput {
        id: "img-001".to_string(),
        bytes: (0..8).collect(),
    };

    let tagger_output: ImageAnalyzed = run_wasmtime(
        root.join("services/image.tagger/target/wasm32-wasip1/release/image_tagger.wasm"),
        &input,
    )?;

    validate_image_analyzed(&tagger_output)?;
    println!("[info] validation.passed event_schema=image.analyzed.v1");
    write_event_envelope(
        "image.analyzed.v1",
        &serde_json::to_value(&tagger_output)?,
        &tagger.name,
        &tagger.version,
    )?;

    let telemetry = validate_telemetry(&tagger_output);
    println!(
        "[info] telemetry.{} {}",
        if telemetry.status == "passed" { "ok" } else { "error" },
        serde_json::to_string(&telemetry)?
    );
    write_event_envelope(
        "telemetry.validation.v1",
        &serde_json::to_value(&telemetry)?,
        &logger.name,
        &logger.version,
    )?;

    let cache_output: ValidationStatus = run_wasmtime(
        root.join("services/edge.cache/target/wasm32-wasip1/release/edge_cache.wasm"),
        &tagger_output,
    )?;
    println!(
        "[info] cache.{} {}",
        if cache_output.status == "passed" { "ok" } else { "error" },
        serde_json::to_string(&cache_output)?
    );
    write_event_envelope(
        "cache.persisted.v1",
        &serde_json::to_value(&cache_output)?,
        &edge_cache.name,
        &edge_cache.version,
    )?;

    let evaluator_output = evaluate(&tagger_output);
    println!("[info] evaluator.ok {}", serde_json::to_string(&evaluator_output)?);
    write_event_envelope(
        "inference.completed.v1",
        &serde_json::to_value(&evaluator_output)?,
        &evaluator.name,
        &evaluator.version,
    )?;

    Ok(())
}

fn ensure_log_dirs() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("logs/events")?;
    Ok(())
}

fn project_root() -> Result<PathBuf, Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    if cwd.join("contracts").exists() {
        return Ok(cwd);
    }

    let manifest_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("unable to resolve project root")?
        .to_path_buf();
    if manifest_root.join("contracts").exists() {
        return Ok(manifest_root);
    }

    Err("unable to locate Chapter 7 project root".into())
}

fn sha256_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let raw = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(raw);
    let digest = hasher.finalize();
    Ok(digest.iter().map(|b| format!("{b:02x}")).collect())
}

fn load_contract(path: PathBuf) -> Result<Contract, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&raw)?)
}

fn match_pattern(pattern: &str, event_name: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix(".*") {
        event_name.starts_with(prefix)
    } else {
        pattern == event_name
    }
}

fn bindings<'a>(publisher: &'a Contract, subscriber: &'a Contract) -> Vec<&'a str> {
    let mut matched = Vec::new();
    for emit in &publisher.events.emits {
        for subscribe in &subscriber.events.subscribes {
            if match_pattern(&subscribe.pattern, &emit.name) {
                matched.push(emit.name.as_str());
            }
        }
    }
    matched
}

fn print_binding(publisher: &Contract, subscriber: &Contract) {
    let matched = bindings(publisher, subscriber);
    if matched.is_empty() {
        println!("[warn] no binding created for {}", subscriber.name);
    } else {
        println!(
            "[info] binding.created {} → {}",
            matched.join(", "),
            subscriber.name
        );
    }
}

fn enforce_policy(root: &Path, evaluator: &Contract) -> Result<Option<String>, Box<dyn Error>> {
    let raw = fs::read_to_string(root.join("contracts/policies/org.telemetry.standard.json"))?;
    let policy: PolicyFile = serde_json::from_str(&raw)?;
    let contains_browser = evaluator
        .constraints
        .placement
        .iter()
        .any(|placement| placement == "browser");

    if contains_browser {
        for rule in policy.deny {
            if rule.condition.service == "ai.model.evaluator"
                && rule.condition.placement == "browser"
            {
                return Ok(Some(format!("policy.deny {}", rule.rule)));
            }
        }
    }
    Ok(None)
}

fn run_wasmtime<TInput, TOutput>(wasm_path: PathBuf, input: &TInput) -> Result<TOutput, Box<dyn Error>>
where
    TInput: Serialize,
    TOutput: for<'de> Deserialize<'de>,
{
    let start = Instant::now();
    let input_json = serde_json::to_vec(input)?;
    let output = Command::new("wasmtime")
        .args(["run", "--dir=.", wasm_path.to_string_lossy().as_ref()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(&input_json)?;
            }
            child.wait_with_output()
        })?;

    log_telemetry(json!({
        "metric": "uma.qos.latency.ms",
        "value": start.elapsed().as_millis()
    }))?;

    if !output.status.success() {
        return Err(format!(
            "wasmtime failed for {}: {}",
            wasm_path.display(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

fn validate_image_analyzed(event: &ImageAnalyzed) -> Result<(), Box<dyn Error>> {
    if event.id.trim().is_empty() {
        return Err("validation.failed id must be a non-empty string".into());
    }
    if event.tags.is_empty() || event.tags.iter().any(|tag| tag.trim().is_empty()) {
        return Err("validation.failed tags must be a non-empty string array".into());
    }
    Ok(())
}

fn validate_telemetry(event: &ImageAnalyzed) -> ValidationStatus {
    match validate_image_analyzed(event) {
        Ok(()) => ValidationStatus {
            source: "telemetry.logger".to_string(),
            event: "image.analyzed.v1".to_string(),
            status: "passed".to_string(),
            reason: None,
        },
        Err(err) => ValidationStatus {
            source: "telemetry.logger".to_string(),
            event: "image.analyzed.v1".to_string(),
            status: "failed".to_string(),
            reason: Some(err.to_string()),
        },
    }
}

fn evaluate(event: &ImageAnalyzed) -> EvaluatorOutput {
    let score = if event.tags.iter().any(|tag| tag == "even") {
        0.7
    } else {
        0.3
    };
    EvaluatorOutput {
        id: event.id.clone(),
        score,
    }
}

fn log_telemetry(payload: Value) -> Result<(), Box<dyn Error>> {
    ensure_log_dirs()?;
    let line = serde_json::to_string(&payload)?;
    fs::write(
        "logs/telemetry.jsonl",
        format!(
            "{}{}",
            fs::read_to_string("logs/telemetry.jsonl").unwrap_or_default(),
            line + "\n"
        ),
    )?;
    Ok(())
}

fn write_event_envelope(
    event_type: &str,
    data: &Value,
    service_id: &str,
    contract_version: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    ensure_log_dirs()?;
    let id = next_event_id();
    let mut envelope = Map::new();
    envelope.insert("specversion".into(), Value::String("1.0".into()));
    envelope.insert("id".into(), Value::String(id.clone()));
    envelope.insert("source".into(), Value::String(service_id.to_string()));
    envelope.insert("type".into(), Value::String(event_type.to_string()));
    envelope.insert("time".into(), Value::String(iso_timestamp()));
    envelope.insert(
        "datacontenttype".into(),
        Value::String("application/json".into()),
    );
    envelope.insert("data".into(), data.clone());
    envelope.insert("umaserviceid".into(), Value::String(service_id.to_string()));
    envelope.insert(
        "umacontractversion".into(),
        Value::String(contract_version.to_string()),
    );
    envelope.insert("umaruntimeid".into(), Value::String("cloud-runner".into()));
    envelope.insert("phase".into(), Value::String("normal".into()));
    envelope.insert("reasonCode".into(), Value::String("OK".into()));

    let path = Path::new("logs/events").join(format!("{id}.json"));
    fs::write(&path, serde_json::to_string_pretty(&Value::Object(envelope))?)?;
    Ok(path)
}

fn next_event_id() -> String {
    let count = EVENT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("evt-{now}-{count}")
}

fn iso_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_binding_matches_image_events() {
        let publisher = Contract {
            name: "image.tagger".into(),
            version: "1.1.0".into(),
            events: ContractEvents {
                emits: vec![EventSpec {
                    name: "image.analyzed.v1".into(),
                    schema: "contracts/schemas/image.analyzed.v1.json".into(),
                }],
                subscribes: Vec::new(),
            },
            constraints: ContractConstraints::default(),
        };
        let subscriber = Contract {
            name: "telemetry.logger".into(),
            version: "1.0.0".into(),
            events: ContractEvents {
                emits: Vec::new(),
                subscribes: vec![SubscribeSpec {
                    pattern: "image.*".into(),
                }],
            },
            constraints: ContractConstraints::default(),
        };

        assert_eq!(bindings(&publisher, &subscriber), vec!["image.analyzed.v1"]);
    }

    #[test]
    fn browser_placement_triggers_policy_violation() {
        let root = project_root().unwrap();
        let contract_path = root.join("contracts/ai.model.evaluator.contract.yaml");
        let evaluator = load_contract(contract_path).unwrap();
        let reason = enforce_policy(&root, &evaluator).unwrap();
        assert_eq!(reason.as_deref(), Some("policy.deny forbid_evaluator_in_browser"));
    }

    #[test]
    fn image_analyzed_validation_rejects_empty_tags() {
        let invalid = ImageAnalyzed {
            id: "img-001".into(),
            tags: vec![],
        };
        assert!(validate_image_analyzed(&invalid).is_err());
    }

    #[test]
    fn exact_pattern_match_requires_exact_event_name() {
        assert!(match_pattern("image.analyzed.v1", "image.analyzed.v1"));
        assert!(!match_pattern("image.analyzed.v1", "image.analyzed.v2"));
    }

    #[test]
    fn telemetry_validation_returns_failed_status_for_invalid_event() {
        let invalid = ImageAnalyzed {
            id: "".into(),
            tags: vec!["even".into()],
        };
        let result = validate_telemetry(&invalid);
        assert_eq!(result.status, "failed");
        assert!(result.reason.unwrap().contains("non-empty string"));
    }

    #[test]
    fn odd_tags_produce_lower_evaluator_score() {
        let event = ImageAnalyzed {
            id: "img-001".into(),
            tags: vec!["odd".into(), "low-entropy".into()],
        };
        let output = evaluate(&event);
        assert_eq!(output.score, 0.3);
    }

    #[test]
    fn event_envelope_contains_expected_metadata() {
        let tmp = std::env::temp_dir().join(format!(
            "chapter7-envelope-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(tmp.join("logs/events")).unwrap();
        let previous = std::env::current_dir().unwrap();
        std::env::set_current_dir(&tmp).unwrap();

        let path = write_event_envelope(
            "image.analyzed.v1",
            &json!({"id":"img-001","tags":["even"]}),
            "image.tagger",
            "1.1.0",
        )
        .unwrap();
        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

        std::env::set_current_dir(previous).unwrap();

        assert_eq!(written.get("type").and_then(Value::as_str), Some("image.analyzed.v1"));
        assert_eq!(written.get("umaserviceid").and_then(Value::as_str), Some("image.tagger"));
        assert_eq!(written.get("umacontractversion").and_then(Value::as_str), Some("1.1.0"));
        assert_eq!(
            written.get("data").and_then(|v| v.get("id")).and_then(Value::as_str),
            Some("img-001")
        );
    }
}
