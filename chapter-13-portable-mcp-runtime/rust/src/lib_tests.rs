use super::*;
use crate::storage::contract_fixtures;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_root() -> PathBuf {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("ch13-runtime-tests-{nanos}-{counter}"));
    fs::create_dir_all(&path).unwrap();
    path
}

fn write_file(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("parent")).unwrap();
    fs::write(path, contents).unwrap();
}

fn install_fake_runtime_hosted_environment(repo_root: &Path, planner_proposal: &str) -> PathBuf {
    let chapter_root = repo_root.join("chapter13");
    for path in [
        "planner-ai-wasi/target/wasm32-wasip1/debug/chapter13_planner_ai_wasi.wasm",
        "summarizer-ai-wasi/target/wasm32-wasip1/debug/chapter13_summarizer_ai_wasi.wasm",
        "translator-ai-wasi/target/wasm32-wasip1/debug/chapter13_translator_ai_wasi.wasm",
        "models/planner/model_quantized.onnx",
        "models/planner/manifest.json",
        "models/model_quantized.onnx",
        "models/manifest.json",
        "models/translator/model_quantized.onnx",
        "models/translator/manifest.json",
    ] {
        write_file(&chapter_root.join(path), "stub");
    }

    let wasmtime = repo_root.join(".bin/wasmtime-v39.0.0-aarch64-macos/wasmtime");
    write_file(
        &wasmtime,
        &format!(
            "#!/bin/sh\nlast=\"\"\nfor arg in \"$@\"; do last=\"$arg\"; done\ncat >/dev/null\ncase \"$last\" in\n  *planner_ai_wasi.wasm)\n    printf '%s' '{}' ;;\n  *summarizer_ai_wasi.wasm)\n    printf '%s' '{}' ;;\n  *translator_ai_wasi.wasm)\n    printf '%s' '{}' ;;\n  *)\n    echo unknown module >&2\n    exit 1 ;;\nesac\n",
            serde_json::json!({
                "proposal": planner_proposal,
                "provider": "chapter13-planner-ai-wasi",
                "mode": "runtime-hosted-ranking",
                "model_id": "planner",
                "model_revision": "1",
                "model_checksum": "abc"
            }),
            serde_json::json!({
                "summary": "Hosted summary",
                "provider": "chapter13-summarizer-ai-wasi",
                "mode": "runtime-hosted-extractive",
                "model_id": "summarizer",
                "model_revision": "1",
                "model_checksum": "abc"
            }),
            serde_json::json!({
                "translated_text": "Rapport en francais: Hosted summary",
                "translated_facts": ["Fait 1", "Fait 2"],
                "provider": "chapter13-translator-ai-wasi",
                "mode": "runtime-hosted-translation",
                "model_id": "translator",
                "model_revision": "1",
                "model_checksum": "abc"
            }),
        ),
    );
    let mut perms = fs::metadata(&wasmtime).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&wasmtime, perms).unwrap();
    chapter_root
}

fn sample_goal() -> GoalSpec {
    GoalSpec {
        target: "brief".into(),
        target_language: "en".into(),
        prefer_ai: false,
        allow_degraded: false,
        local_only: false,
    }
}

fn sample_context() -> ContextSeed {
    ContextSeed {
        project_name: "UMA Runtime Atlas".into(),
        source_fragments: vec!["Browser telemetry remains healthy.".into()],
        available_capabilities: vec![
            "DataProviderLocal".into(),
            "InsightEnricher".into(),
            "SummarizerBasic".into(),
            "Formatter".into(),
        ],
        ai_available: false,
    }
}

fn sample_scenario() -> Scenario {
    Scenario {
        id: "sample".into(),
        title: "Sample".into(),
        summary: "Sample summary".into(),
        goal: sample_goal(),
        context: sample_context(),
    }
}

fn sample_state() -> RuntimeState {
    RuntimeState {
        source_loaded: false,
        source_fragments: vec!["Browser telemetry remains healthy.".into()],
        structured_facts: Vec::new(),
        summary: None,
        translated_summary: None,
        translated_facts: Vec::new(),
        report: None,
        degraded: false,
    }
}

#[test]
fn basic_report_uses_deterministic_path() {
    let report = run_scenario(&project_root(), "use-case-1-basic-report").unwrap();
    assert_eq!(report.status, "complete");
    assert_eq!(
        report.selected_path,
        vec!["DataProviderLocal", "InsightEnricher", "SummarizerBasic", "Formatter"]
    );
    assert_eq!(report.final_language, "en");
}

#[test]
fn french_report_inserts_translation() {
    let report = run_scenario(&project_root(), "use-case-3-french-report").unwrap();
    assert!(report.selected_path.iter().any(|item| item == "TranslatorFr"));
    assert_eq!(report.final_language, "fr");
    assert!(report.final_output.contains("Langue: fr"));
    assert!(
        report.translator_ai_mode == "runtime-hosted-translation" || report.translator_ai_mode == "fallback"
    );
}

#[test]
fn agent_proposal_can_be_rejected_by_runtime() {
    let report = run_scenario(&project_root(), "use-case-5-agent-validation").unwrap();
    assert_eq!(report.planner_provider, "deterministic-local-planner");
    assert_eq!(report.planner_mode, "deterministic");
    assert!(report
        .rejected_capabilities
        .iter()
        .any(|item| item.capability == "SummarizerAI"));
    assert!(report.selected_path.iter().any(|item| item == "SummarizerBasic"));
    assert!(!report.selected_path.iter().any(|item| item == "SummarizerAI"));
}

#[test]
fn runtime_can_degrade_when_translation_is_unavailable() {
    let report = run_scenario(&project_root(), "use-case-4-runtime-adapts").unwrap();
    assert_eq!(report.status, "degraded");
    assert_eq!(report.final_language, "en (degraded)");
}

#[test]
fn ai_report_prefers_ai_when_constraints_allow_it() {
    let report = run_scenario(&project_root(), "use-case-2-ai-report").unwrap();
    assert!(
        report.planner_provider == "chapter13-planner-ai-wasi" || report.planner_provider == "deterministic-local-planner"
    );
    assert!(report.planner_mode == "runtime-hosted-ranking" || report.planner_mode == "fallback");
    assert_eq!(
        report.planner_provider == "chapter13-planner-ai-wasi",
        report.planner_mode == "runtime-hosted-ranking"
    );
    assert_eq!(
        report.planner_fallback_reason.is_none(),
        report.planner_mode == "runtime-hosted-ranking"
    );
    assert!(report.selected_path.iter().any(|item| item == "SummarizerAI"));
    assert!(!report.selected_path.iter().any(|item| item == "SummarizerBasic"));
    assert!(report.summarizer_ai_mode == "fallback" || report.summarizer_ai_mode == "runtime-hosted-extractive");
    assert_eq!(
        report.summarizer_ai_provider == "chapter13-summarizer-ai-wasi",
        report.summarizer_ai_mode == "runtime-hosted-extractive"
    );
    assert_eq!(
        report.summarizer_ai_fallback_reason.is_none(),
        report.summarizer_ai_mode == "runtime-hosted-extractive"
    );
    let ai_step = report
        .steps
        .iter()
        .find(|step| step.selected_capability == "SummarizerAI")
        .unwrap();
    assert_eq!(ai_step.agent_mode, report.planner_mode);
    assert_eq!(ai_step.execution_mode, report.summarizer_ai_mode);
    let translator_step = report
        .steps
        .iter()
        .find(|step| step.selected_capability == "TranslatorFr")
        .unwrap();
    assert_eq!(translator_step.execution_mode, report.translator_ai_mode);
    assert_eq!(report.final_language, "fr");
}

#[test]
fn ai_executive_briefing_uses_real_ai_path_without_translation() {
    let report = run_scenario(&project_root(), "use-case-6-ai-executive-briefing").unwrap();
    assert!(
        report.planner_provider == "chapter13-planner-ai-wasi" || report.planner_provider == "deterministic-local-planner"
    );
    assert!(report.planner_mode == "runtime-hosted-ranking" || report.planner_mode == "fallback");
    assert!(report.selected_path.iter().any(|item| item == "SummarizerAI"));
    assert!(!report.selected_path.iter().any(|item| item == "TranslatorFr"));
    assert!(!report.selected_path.iter().any(|item| item == "SummarizerBasic"));
    assert_eq!(report.final_language, "en");
    let ai_step = report
        .steps
        .iter()
        .find(|step| step.selected_capability == "SummarizerAI")
        .unwrap();
    assert_eq!(ai_step.execution_mode, report.summarizer_ai_mode);
}

#[test]
fn runtime_hosted_success_paths_are_used_when_fake_wasmtime_is_available() {
    let repo_root = temp_root();
    let chapter_root = install_fake_runtime_hosted_environment(&repo_root, "SummarizerAI");
    write_file(
        &chapter_root.join("examples/ai-report/scenario.json"),
        include_str!("../../examples/ai-report/scenario.json"),
    );
    write_file(
        &chapter_root.join("examples/french-report/scenario.json"),
        include_str!("../../examples/french-report/scenario.json"),
    );

    let ai_report = run_scenario(&chapter_root, "use-case-2-ai-report").unwrap();
    assert_eq!(ai_report.planner_provider, "chapter13-planner-ai-wasi");
    assert_eq!(ai_report.planner_mode, "runtime-hosted-ranking");
    assert_eq!(ai_report.summarizer_ai_provider, "chapter13-summarizer-ai-wasi");
    assert_eq!(ai_report.summarizer_ai_mode, "runtime-hosted-extractive");
    assert!(ai_report
        .steps
        .iter()
        .any(|step| step.selected_capability == "SummarizerAI" && step.output_preview == "Hosted summary"));

    let french_report = run_scenario(&chapter_root, "use-case-3-french-report").unwrap();
    assert_eq!(french_report.translator_ai_provider, "chapter13-translator-ai-wasi");
    assert_eq!(french_report.translator_ai_mode, "runtime-hosted-translation");
    assert!(french_report.final_output.contains("Fait 1"));
}

#[test]
fn planner_unknown_proposal_and_degraded_selection_paths_are_covered() {
    let repo_root = temp_root();
    let chapter_root = install_fake_runtime_hosted_environment(&repo_root, "UnknownCapability");
    write_file(
        &chapter_root.join("examples/ai-report/scenario.json"),
        include_str!("../../examples/ai-report/scenario.json"),
    );
    let report = run_scenario(&chapter_root, "use-case-2-ai-report").unwrap();
    assert!(report.selected_path.iter().any(|item| item == "SummarizerBasic" || item == "SummarizerAI"));

    write_file(
        &chapter_root.join("examples/degraded-no-capability/scenario.json"),
        r#"{
          "id":"degraded-no-capability",
          "title":"Degraded no capability",
          "summary":"Degraded path",
          "goal":{"target":"report","targetLanguage":"en","preferAI":false,"allowDegraded":true,"localOnly":false},
          "context":{
            "projectName":"UMA",
            "sourceFragments":["One source."],
            "availableCapabilities":["DataProviderLocal"],
            "aiAvailable":false
          }
        }"#,
    );
    let error = run_scenario(&chapter_root, "degraded-no-capability").unwrap_err();
    assert!(error.contains("degraded path required"));
}

#[test]
fn scenario_loading_and_capability_filtering_cover_error_paths() {
    let root = temp_root();
    let error = list_scenarios(&root).unwrap_err();
    assert!(!error.is_empty());

    write_file(&root.join("examples/ignore.txt"), "noop");
    write_file(
        &root.join("examples/use-case-1-basic-report/scenario.json"),
        include_str!("../../examples/basic-report/scenario.json"),
    );
    write_file(&root.join("examples/bad/scenario.json"), "{ invalid");

    let error = list_scenarios(&root).unwrap_err();
    assert!(!error.is_empty());

    fs::remove_dir_all(root.join("examples/bad")).unwrap();
    let scenarios = list_scenarios(&root).unwrap();
    assert_eq!(scenarios.len(), 1);
    assert_eq!(load_scenario(&root, "missing").unwrap_err(), "unknown scenario \"missing\"");

    let caps = available_capabilities_for_scenario(&root, "use-case-1-basic-report").unwrap();
    assert_eq!(caps.len(), 5);
    assert!(capability_descriptors().iter().any(|item| item.name == "PlannerAI"));
}

#[test]
fn need_and_input_helpers_cover_all_branches() {
    let mut goal = sample_goal();
    let mut state = sample_state();
    assert_eq!(current_need(&goal, &state), "provide-source-fragments");
    assert!(!has_input("source_fragments", &state));
    assert!(!has_input("missing", &state));

    state.source_loaded = true;
    assert_eq!(current_need(&goal, &state), "derive-structured-insights");
    state.structured_facts = vec!["Fact: Browser telemetry remains healthy.".into()];
    assert_eq!(current_need(&goal, &state), "generate-summary");
    assert!(has_input("structured_facts", &state));

    state.summary = Some("Summary".into());
    goal.target_language = "fr".into();
    assert_eq!(current_need(&goal, &state), "translate-to-target-language");
    assert!(has_input("summary", &state));
    assert!(has_input("summary_or_translation", &state));

    state.translated_summary = Some("Resume".into());
    state.degraded = true;
    assert_eq!(current_need(&goal, &state), "format-structured-report");
    assert!(has_input("translated_summary", &state));
    state.report = Some("done".into());
    assert_eq!(current_need(&goal, &state), "goal-satisfied");
}

#[test]
fn agent_selection_helpers_cover_direct_deterministic_and_fallback_modes() {
    let mut scenario = sample_scenario();
    let visible = vec![contract_fixtures().into_iter().find(|item| item.name == "Formatter").unwrap()];
    let direct =
        propose_with_agent(&temp_root(), AgentProviderKind::DeterministicLocal, "format-structured-report", &scenario, &visible);
    assert_eq!(direct.provider, "runtime-direct-selection");
    assert_eq!(direct.mode, "direct");
    assert_eq!(direct.proposal.as_deref(), Some("Formatter"));

    let visible = contract_fixtures()
        .into_iter()
        .filter(|item| item.name == "SummarizerBasic" || item.name == "SummarizerAI")
        .collect::<Vec<_>>();
    let deterministic =
        propose_with_agent(&temp_root(), AgentProviderKind::DeterministicLocal, "generate-summary", &scenario, &visible);
    assert_eq!(deterministic.provider, "deterministic-local-planner");
    assert_eq!(deterministic.mode, "deterministic");
    assert_eq!(deterministic.proposal.as_deref(), Some("SummarizerBasic"));

    scenario.goal.prefer_ai = true;
    scenario.context.ai_available = true;
    scenario.context.available_capabilities.push("PlannerAI".into());
    assert!(matches!(agent_provider_for(&scenario), AgentProviderKind::PlannerAI));
    let fallback = propose_with_agent(&temp_root(), AgentProviderKind::PlannerAI, "generate-summary", &scenario, &visible);
    assert_eq!(fallback.provider, "deterministic-local-planner");
    assert_eq!(fallback.mode, "fallback");
    assert!(fallback.fallback_reason.as_deref().unwrap().contains("PlannerAI module"));

    assert_eq!(AgentProviderKind::PlannerAI.label(), "PlannerAI");
    assert_eq!(AgentProviderKind::DeterministicLocal.label(), "deterministic-local-planner");
    let without_ai = deterministic_agent_proposal("generate-summary", &scenario, &[visible[0].clone()]);
    assert_eq!(without_ai.as_deref(), Some("SummarizerBasic"));
    assert!(deterministic_agent_proposal("other", &scenario, &[]).is_none());
}

#[test]
fn planner_and_runtime_hosted_invocations_cover_missing_artifacts() {
    let root = temp_root();
    let scenario = sample_scenario();
    let visible = contract_fixtures()
        .into_iter()
        .filter(|item| item.name == "SummarizerAI")
        .collect::<Vec<_>>();
    let error = hosted::invoke_runtime_hosted_planner(&root, "generate-summary", &scenario, &visible).unwrap_err();
    assert!(error.contains("module is not built yet"));

    write_file(
        &root.join("planner-ai-wasi/target/wasm32-wasip1/debug/chapter13_planner_ai_wasi.wasm"),
        "wasm",
    );
    let error = hosted::invoke_runtime_hosted_planner(&root, "generate-summary", &scenario, &visible).unwrap_err();
    assert!(error.contains("model artifacts are not installed"));

    let error = hosted::invoke_runtime_hosted_summarizer(&root, "UMA Runtime Atlas", &["Fact".into()]).unwrap_err();
    assert!(error.contains("module is not built yet"));
    write_file(
        &root.join("summarizer-ai-wasi/target/wasm32-wasip1/debug/chapter13_summarizer_ai_wasi.wasm"),
        "wasm",
    );
    let error = hosted::invoke_runtime_hosted_summarizer(&root, "UMA Runtime Atlas", &["Fact".into()]).unwrap_err();
    assert!(error.contains("model artifacts are not installed"));

    let error = hosted::invoke_runtime_hosted_translator(&root, "Summary", &["Fact".into()]).unwrap_err();
    assert!(error.contains("module is not built yet"));
    write_file(
        &root.join("translator-ai-wasi/target/wasm32-wasip1/debug/chapter13_translator_ai_wasi.wasm"),
        "wasm",
    );
    let error = hosted::invoke_runtime_hosted_translator(&root, "Summary", &["Fact".into()]).unwrap_err();
    assert!(error.contains("model artifacts are not installed"));
}

#[test]
fn contract_validation_and_discovery_cover_all_rejection_reasons() {
    let mut scenario = sample_scenario();
    let mut state = sample_state();
    state.source_loaded = true;
    let contract = contract_fixtures().into_iter().find(|item| item.name == "SummarizerAI").unwrap();
    let rejected = validate_contract(&contract, &scenario, &state);
    assert_eq!(rejected.status, "rejected");
    assert!(rejected.reasons.iter().any(|item| item.contains("missing input: structured_facts")));
    assert!(rejected.reasons.iter().any(|item| item.contains("AI capability is unavailable")));

    let mut local_contract = contract_fixtures().into_iter().find(|item| item.name == "DataProviderLocal").unwrap();
    local_contract.constraints.requires = vec!["local".into()];
    local_contract.constraints.runtime = vec!["cloud".into()];
    scenario.goal.local_only = true;
    local_contract.constraints.excludes = vec!["local-only".into()];
    let local_rejected = validate_contract(&local_contract, &scenario, &state);
    assert!(local_rejected.reasons.iter().any(|item| item.contains("violates local-only")));
    assert!(local_rejected
        .reasons
        .iter()
        .any(|item| item.contains("cannot satisfy local execution placement")));

    let translator = contract_fixtures().into_iter().find(|item| item.name == "TranslatorFr").unwrap();
    let translator_rejected = validate_contract(&translator, &scenario, &state);
    assert!(translator_rejected.reasons.iter().any(|item| item.contains("translator is unavailable")));

    let (visible, discovery) = discover_candidates(&scenario, &state, "format-structured-report");
    assert!(visible.iter().any(|item| item.name == "Formatter"));
    assert!(discovery.rejected.iter().any(|item| item.status == "hidden"));
    assert!(discovery.rejected.iter().any(|item| item.status == "not-relevant"));
    assert!(relevant_for_need(&visible[0], "format-structured-report"));
}

#[test]
fn step_selection_and_execution_helpers_cover_fallback_paths() {
    let scenario = sample_scenario();
    let mut state = sample_state();
    state.source_loaded = true;
    state.structured_facts = vec!["Fact: one.".into()];

    let visible = vec![
        contract_fixtures().into_iter().find(|item| item.name == "SummarizerBasic").unwrap(),
        contract_fixtures().into_iter().find(|item| item.name == "SummarizerAI").unwrap(),
    ];
    let mut rejected = Vec::new();
    let proposal = Some("UnknownCapability".to_string());
    let (selected, validation, proposed_validation) =
        select_contract_for_step(&visible, &scenario, &state, &proposal, &mut rejected);
    assert!(selected.is_some());
    assert!(validation.is_some());
    assert!(proposed_validation.is_none());
    assert!(rejected.is_empty());

    let mut no_input_state = sample_state();
    no_input_state.source_loaded = true;
    let mut rejected = Vec::new();
    let (selected, _, _) = select_contract_for_step(&visible, &scenario, &no_input_state, &None, &mut rejected);
    assert!(selected.is_none());
    assert!(!rejected.is_empty());

    let mut events = Vec::new();
    let mut mode = CapabilityExecutionMode {
        provider: "hosted".into(),
        mode: "runtime-hosted-extractive".into(),
        fallback_reason: None,
        hosted_summary: Some("Hosted summary".into()),
        hosted_translation: None,
        hosted_translated_facts: None,
    };
    let summary = execute_selected_capability(&visible[1], &scenario, &mut state, &mode, &mut events);
    assert_eq!(summary, "Hosted summary");

    mode.mode = "runtime-hosted-translation".into();
    mode.hosted_translation = Some("Rapport en francais: Hosted summary".into());
    mode.hosted_translated_facts = Some(vec!["Fait 1".into()]);
    let translator = contract_fixtures().into_iter().find(|item| item.name == "TranslatorFr").unwrap();
    let translated = execute_selected_capability(&translator, &scenario, &mut state, &mode, &mut events);
    assert!(translated.contains("Rapport en francais"));

    let unknown = CapabilityContract {
        name: "Unknown".into(),
        version: "1".into(),
        intent: "unknown".into(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        constraints: ContractConstraints {
            runtime: Vec::new(),
            requires: Vec::new(),
            excludes: Vec::new(),
        },
        emits_events: Vec::new(),
        metadata: ContractMetadata {
            description: "Unknown".into(),
            tags: Vec::new(),
        },
    };
    let output = execute_selected_capability(&unknown, &scenario, &mut state, &mode, &mut events);
    assert_eq!(output, "No output");
}

#[test]
fn snapshot_event_and_text_helpers_cover_branches() {
    let mut state = sample_state();
    let snapshot = context_snapshot(&state);
    assert_eq!(snapshot.get("summary").map(String::as_str), Some("none"));
    assert_eq!(snapshot.get("translatedFacts").map(String::as_str), Some("none"));

    state.summary = Some("Summary".into());
    state.translated_summary = Some("Resume".into());
    state.translated_facts = vec!["Fait 1".into(), "Fait 2".into()];
    state.report = Some("Report".into());
    let snapshot = context_snapshot(&state);
    assert_eq!(snapshot.get("summary").map(String::as_str), Some("Summary"));
    assert!(snapshot.get("translatedFacts").unwrap().contains("Fait 1"));

    let event = emit_event("CapabilityExecuted", "Formatter", "success", Some("ok".into()), &state);
    assert_eq!(event.event_type, "CapabilityExecuted");
    assert_eq!(event.reason.as_deref(), Some("ok"));

    assert_eq!(enrich_facts(&["One.".into(), "Two".into()]), vec!["Fact: One.".to_string(), "Fact: Two.".to_string()]);
    assert!(summarize_basic("UMA", &["a".into()]).contains("UMA combines"));
    assert!(summarize_ai("UMA", &["a".into()]).contains("UMA shows"));
    assert!(translate_french(
        "combines distributed browser, edge, and cloud evidence into a deterministic operational summary with 2 validated insight(s)."
    )
    .contains("Rapport en francais"));
}

#[test]
fn report_output_and_execution_modes_cover_fallback_and_standard_paths() {
    let mut state = sample_state();
    let empty_output = format_report_output("UMA", &state, "en");
    assert!(empty_output.contains("No summary available"));

    state.summary = Some("Summary".into());
    let output = format_report_output("UMA", &state, "en");
    assert!(output.contains("No structured facts"));
    state.translated_summary = Some("Resume".into());
    state.translated_facts = vec!["Fait 1".into()];
    let output = format_report_output("UMA", &state, "fr");
    assert!(output.contains("Projet: UMA"));
    assert!(output.contains("Points cles: Fait 1"));

    let root = temp_root();
    let basic = contract_fixtures().into_iter().find(|item| item.name == "SummarizerBasic").unwrap();
    let standard = contract_fixtures().into_iter().find(|item| item.name == "Formatter").unwrap();
    let ai = contract_fixtures().into_iter().find(|item| item.name == "SummarizerAI").unwrap();
    let translator = contract_fixtures().into_iter().find(|item| item.name == "TranslatorFr").unwrap();

    let basic_mode = hosted::execution_mode_for(&root, &basic, "UMA", &["Fact".into()], Some("Summary"));
    assert_eq!(basic_mode.mode, "deterministic");
    let standard_mode = hosted::execution_mode_for(&root, &standard, "UMA", &["Fact".into()], Some("Summary"));
    assert_eq!(standard_mode.mode, "standard");
    let ai_mode = hosted::execution_mode_for(&root, &ai, "UMA", &["Fact".into()], Some("Summary"));
    assert_eq!(ai_mode.mode, "fallback");
    let translator_mode = hosted::execution_mode_for(&root, &translator, "UMA", &["Fact".into()], Some("Summary"));
    assert_eq!(translator_mode.mode, "fallback");
}

#[test]
fn run_scenario_and_formatting_cover_error_and_optional_paths() {
    let root = temp_root();
    write_file(
        &root.join("examples/french-required/scenario.json"),
        r#"{
          "id":"french-required",
          "title":"French required",
          "summary":"Need a french result",
          "goal":{"target":"report","targetLanguage":"fr","preferAI":false,"allowDegraded":false,"localOnly":false},
          "context":{
            "projectName":"UMA",
            "sourceFragments":["One source."],
            "availableCapabilities":["DataProviderLocal","InsightEnricher","SummarizerBasic","Formatter"],
            "aiAvailable":false
          }
        }"#,
    );
    let error = run_scenario(&root, "french-required").unwrap_err();
    assert!(error.contains("translator required but unavailable"));

    write_file(
        &root.join("examples/no-capability/scenario.json"),
        r#"{
          "id":"no-capability",
          "title":"No capability",
          "summary":"No capability",
          "goal":{"target":"report","targetLanguage":"en","preferAI":false,"allowDegraded":false,"localOnly":false},
          "context":{
            "projectName":"UMA",
            "sourceFragments":["One source."],
            "availableCapabilities":["DataProviderLocal"],
            "aiAvailable":false
          }
        }"#,
    );
    let error = run_scenario(&root, "no-capability").unwrap_err();
    assert!(error.contains("no compatible capability satisfied the current need"));

    let mut report = run_scenario(&project_root(), "use-case-5-agent-validation").unwrap();
    report.planner_fallback_reason = Some("fallback".into());
    report.summarizer_ai_fallback_reason = Some("ai fallback".into());
    report.translator_ai_fallback_reason = Some("translator fallback".into());
    report.rejected_capabilities.push(ValidationResult {
        capability: "TranslatorFr".into(),
        status: "rejected".into(),
        reasons: vec!["translator unavailable".into()],
    });
    report.steps[0].agent_fallback_reason = Some("agent fallback".into());
    report.steps[0].fallback_reason = Some("execution fallback".into());
    report.steps[0].discovery.rejected.push(DiscoveryCandidate {
        capability: "Hidden".into(),
        intent: "none".into(),
        reason: Some("does not contribute to the current unmet need".into()),
        status: "hidden".into(),
    });
    report.steps[0].events.push(RuntimeEvent {
        event_type: "CapabilityRejected".into(),
        capability: "Hidden".into(),
        status: "rejected".into(),
        reason: Some("capability not available in this scenario".into()),
        context_snapshot: BTreeMap::new(),
    });
    report.steps[0].validation.reasons = vec!["missing input: translated_summary".into()];
    let formatted = format_report(&report);
    assert!(formatted.contains("Rejected Capabilities:"));
    assert!(formatted.contains("agent fallback note: agent fallback"));
    assert!(formatted.contains("fallback note: execution fallback"));
    assert!(formatted.contains("validation reasons:"));

    let list_json = list_scenarios_json();
    assert!(list_json.contains("use-case-1-basic-report"));
    let run_json = run_scenario_json("use-case-1-basic-report");
    assert!(run_json.contains("\"scenario\": \"use-case-1-basic-report\""));
}
