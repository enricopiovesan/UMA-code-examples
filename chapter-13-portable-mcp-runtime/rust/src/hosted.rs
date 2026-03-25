use crate::{
    CapabilityContract, CapabilityExecutionMode, PlannerCapabilityDescriptor, RuntimeHostedPlannerResult,
    RuntimeHostedSummarizerResult, RuntimeHostedTranslatorResult, Scenario,
};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn planner_descriptor_for(contract: &CapabilityContract) -> PlannerCapabilityDescriptor {
    PlannerCapabilityDescriptor {
        name: contract.name.clone(),
        intent: contract.intent.clone(),
        description: contract.metadata.description.clone(),
        tags: contract.metadata.tags.clone(),
        runtime: contract.constraints.runtime.clone(),
        requires: contract.constraints.requires.clone(),
        excludes: contract.constraints.excludes.clone(),
    }
}

pub(crate) fn invoke_runtime_hosted_planner(
    root: &Path,
    need: &str,
    scenario: &Scenario,
    visible: &[CapabilityContract],
) -> Result<RuntimeHostedPlannerResult, String> {
    let module_path =
        root.join("planner-ai-wasi/target/wasm32-wasip1/debug/chapter13_planner_ai_wasi.wasm");
    if !module_path.exists() {
        return Err("runtime-hosted PlannerAI module is not built yet".to_string());
    }

    let models_dir = root.join("models/planner");
    if !models_dir.join("model_quantized.onnx").exists() || !models_dir.join("manifest.json").exists() {
        return Err("runtime-hosted PlannerAI model artifacts are not installed".to_string());
    }

    let repo_root = root
        .parent()
        .ok_or_else(|| "missing repo root for Chapter 13 runtime".to_string())?;
    let embedded_wasmtime = repo_root.join(".bin/wasmtime-v39.0.0-aarch64-macos/wasmtime");
    let wasmtime = if embedded_wasmtime.exists() {
        embedded_wasmtime
    } else {
        PathBuf::from("wasmtime")
    };

    let request = serde_json::json!({
        "need": need,
        "target_language": scenario.goal.target_language,
        "prefer_ai": scenario.goal.prefer_ai,
        "local_only": scenario.goal.local_only,
        "allow_degraded": scenario.goal.allow_degraded,
        "project_name": scenario.context.project_name,
        "source_fragment_count": scenario.context.source_fragments.len(),
        "visible_capabilities": visible.iter().map(planner_descriptor_for).collect::<Vec<_>>(),
        "model_dir": "/models"
    });

    let mut child = Command::new(wasmtime)
        .arg("run")
        .arg("--dir")
        .arg(format!("{}::/models", models_dir.display()))
        .arg(module_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to start runtime-hosted PlannerAI: {err}"))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "failed to open runtime-hosted PlannerAI stdin".to_string())?;
        stdin
            .write_all(request.to_string().as_bytes())
            .map_err(|err| format!("failed to send runtime-hosted PlannerAI request: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to wait for runtime-hosted PlannerAI: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "runtime-hosted PlannerAI returned a non-zero exit status".to_string()
        } else {
            format!("runtime-hosted PlannerAI failed: {stderr}")
        });
    }

    serde_json::from_slice::<RuntimeHostedPlannerResult>(&output.stdout)
        .map_err(|err| format!("failed to parse runtime-hosted PlannerAI output: {err}"))
}

pub(crate) fn execution_mode_for(
    root: &Path,
    contract: &CapabilityContract,
    project: &str,
    facts: &[String],
    summary: Option<&str>,
) -> CapabilityExecutionMode {
    match contract.name.as_str() {
        "SummarizerAI" => runtime_hosted_summarizer_execution_mode(root, project, facts),
        "TranslatorFr" => runtime_hosted_translator_execution_mode(root, summary.unwrap_or_default(), facts),
        "SummarizerBasic" => CapabilityExecutionMode {
            provider: "deterministic-local-summarizer".to_string(),
            mode: "deterministic".to_string(),
            fallback_reason: None,
            hosted_summary: None,
            hosted_translation: None,
            hosted_translated_facts: None,
        },
        _ => CapabilityExecutionMode {
            provider: contract.name.clone(),
            mode: "standard".to_string(),
            fallback_reason: None,
            hosted_summary: None,
            hosted_translation: None,
            hosted_translated_facts: None,
        },
    }
}

fn runtime_hosted_summarizer_execution_mode(
    root: &Path,
    project: &str,
    facts: &[String],
) -> CapabilityExecutionMode {
    match invoke_runtime_hosted_summarizer(root, project, facts) {
        Ok(result) => CapabilityExecutionMode {
            provider: result.provider,
            mode: result.mode,
            fallback_reason: None,
            hosted_summary: Some(result.summary),
            hosted_translation: None,
            hosted_translated_facts: None,
        },
        Err(reason) => CapabilityExecutionMode {
            provider: "deterministic-fallback-summarizer".to_string(),
            mode: "fallback".to_string(),
            fallback_reason: Some(reason),
            hosted_summary: None,
            hosted_translation: None,
            hosted_translated_facts: None,
        },
    }
}

fn runtime_hosted_translator_execution_mode(root: &Path, summary: &str, facts: &[String]) -> CapabilityExecutionMode {
    match invoke_runtime_hosted_translator(root, summary, facts) {
        Ok(result) => CapabilityExecutionMode {
            provider: result.provider,
            mode: result.mode,
            fallback_reason: None,
            hosted_summary: None,
            hosted_translation: Some(result.translated_text),
            hosted_translated_facts: Some(result.translated_facts),
        },
        Err(reason) => CapabilityExecutionMode {
            provider: "deterministic-fallback-translator".to_string(),
            mode: "fallback".to_string(),
            fallback_reason: Some(reason),
            hosted_summary: None,
            hosted_translation: None,
            hosted_translated_facts: None,
        },
    }
}

pub(crate) fn invoke_runtime_hosted_summarizer(
    root: &Path,
    project: &str,
    facts: &[String],
) -> Result<RuntimeHostedSummarizerResult, String> {
    let module_path = root.join(
        "summarizer-ai-wasi/target/wasm32-wasip1/debug/chapter13_summarizer_ai_wasi.wasm",
    );
    if !module_path.exists() {
        return Err("runtime-hosted SummarizerAI module is not built yet".to_string());
    }

    let models_dir = root.join("models");
    if !models_dir.join("model_quantized.onnx").exists() || !models_dir.join("manifest.json").exists() {
        return Err("runtime-hosted SummarizerAI model artifacts are not installed".to_string());
    }

    let repo_root = root
        .parent()
        .ok_or_else(|| "missing repo root for Chapter 13 runtime".to_string())?;
    let embedded_wasmtime = repo_root.join(".bin/wasmtime-v39.0.0-aarch64-macos/wasmtime");
    let wasmtime = if embedded_wasmtime.exists() {
        embedded_wasmtime
    } else {
        PathBuf::from("wasmtime")
    };

    let request = serde_json::json!({
        "project_name": project,
        "structured_facts": facts,
        "model_dir": "/models",
        "max_length": 2
    });

    let mut child = Command::new(wasmtime)
        .arg("run")
        .arg("--dir")
        .arg(format!("{}::/models", models_dir.display()))
        .arg(module_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to start runtime-hosted SummarizerAI: {err}"))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "failed to open runtime-hosted SummarizerAI stdin".to_string())?;
        stdin
            .write_all(request.to_string().as_bytes())
            .map_err(|err| format!("failed to send runtime-hosted SummarizerAI request: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to wait for runtime-hosted SummarizerAI: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "runtime-hosted SummarizerAI returned a non-zero exit status".to_string()
        } else {
            format!("runtime-hosted SummarizerAI failed: {stderr}")
        });
    }

    serde_json::from_slice::<RuntimeHostedSummarizerResult>(&output.stdout)
        .map_err(|err| format!("failed to parse runtime-hosted SummarizerAI output: {err}"))
}

pub(crate) fn invoke_runtime_hosted_translator(
    root: &Path,
    summary: &str,
    facts: &[String],
) -> Result<RuntimeHostedTranslatorResult, String> {
    let module_path = root.join(
        "translator-ai-wasi/target/wasm32-wasip1/debug/chapter13_translator_ai_wasi.wasm",
    );
    if !module_path.exists() {
        return Err("runtime-hosted TranslatorFr module is not built yet".to_string());
    }

    let models_dir = root.join("models/translator");
    if !models_dir.join("model_quantized.onnx").exists() || !models_dir.join("manifest.json").exists() {
        return Err("runtime-hosted TranslatorFr model artifacts are not installed".to_string());
    }

    let repo_root = root
        .parent()
        .ok_or_else(|| "missing repo root for Chapter 13 runtime".to_string())?;
    let embedded_wasmtime = repo_root.join(".bin/wasmtime-v39.0.0-aarch64-macos/wasmtime");
    let wasmtime = if embedded_wasmtime.exists() {
        embedded_wasmtime
    } else {
        PathBuf::from("wasmtime")
    };

    let request = serde_json::json!({
        "summary": summary,
        "structured_facts": facts,
        "model_dir": "/models"
    });

    let mut child = Command::new(wasmtime)
        .arg("run")
        .arg("--dir")
        .arg(format!("{}::/models", models_dir.display()))
        .arg(module_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to start runtime-hosted TranslatorFr: {err}"))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "failed to open runtime-hosted TranslatorFr stdin".to_string())?;
        stdin
            .write_all(request.to_string().as_bytes())
            .map_err(|err| format!("failed to send runtime-hosted TranslatorFr request: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to wait for runtime-hosted TranslatorFr: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "runtime-hosted TranslatorFr returned a non-zero exit status".to_string()
        } else {
            format!("runtime-hosted TranslatorFr failed: {stderr}")
        });
    }

    serde_json::from_slice::<RuntimeHostedTranslatorResult>(&output.stdout)
        .map_err(|err| format!("failed to parse runtime-hosted TranslatorFr output: {err}"))
}
