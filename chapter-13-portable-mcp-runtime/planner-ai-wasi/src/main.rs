use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tract_onnx::prelude::*;

#[derive(Debug, Deserialize)]
struct PlannerRequest {
    need: String,
    target_language: String,
    prefer_ai: bool,
    local_only: bool,
    allow_degraded: bool,
    project_name: String,
    source_fragment_count: usize,
    visible_capabilities: Vec<VisibleCapability>,
    model_dir: String,
}

#[derive(Debug, Deserialize)]
struct VisibleCapability {
    name: String,
    intent: String,
    description: String,
    tags: Vec<String>,
    runtime: Vec<String>,
    requires: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PlannerResponse {
    proposal: String,
    provider: String,
    mode: String,
    model_id: String,
    model_revision: String,
    model_checksum: String,
}

#[derive(Debug, Deserialize)]
struct ModelManifest {
    #[serde(rename = "modelId")]
    model_id: String,
    revision: String,
    files: Vec<ModelFile>,
}

#[derive(Debug, Deserialize)]
struct ModelFile {
    name: String,
    sha256: String,
}

struct WordpieceTokenizer {
    vocab: HashMap<String, i64>,
    unk_id: i64,
    cls_id: i64,
    sep_id: i64,
}

fn main() -> Result<(), String> {
    let mut raw = String::new();
    io::stdin().read_to_string(&mut raw).map_err(|err| err.to_string())?;
    let request: PlannerRequest = serde_json::from_str(&raw).map_err(|err| err.to_string())?;
    let response = run(&request)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&response).map_err(|err| err.to_string())?
    );
    Ok(())
}

fn run(request: &PlannerRequest) -> Result<PlannerResponse, String> {
    if request.visible_capabilities.is_empty() {
        return Err("planner request did not include any visible capabilities".to_string());
    }

    let model_dir = Path::new(&request.model_dir);
    let manifest = load_manifest(model_dir)?;
    let tokenizer = WordpieceTokenizer::load(&model_dir.join("vocab.txt"))?;
    let model = tract_onnx::onnx()
        .model_for_path(model_dir.join("model_quantized.onnx"))
        .map_err(|err| err.to_string())?
        .into_optimized()
        .map_err(|err| err.to_string())?
        .into_runnable()
        .map_err(|err| err.to_string())?;

    let query = format!(
        "need: {} target-language: {} prefer-ai: {} local-only: {} allow-degraded: {} project: {} source-fragments: {}",
        request.need,
        request.target_language,
        request.prefer_ai,
        request.local_only,
        request.allow_degraded,
        request.project_name,
        request.source_fragment_count
    );
    let query_embedding = embed_text(&model, &tokenizer, &query)?;

    let mut best: Option<(&VisibleCapability, f32)> = None;
    for capability in &request.visible_capabilities {
        let candidate_text = format_candidate_text(capability);
        let candidate_embedding = embed_text(&model, &tokenizer, &candidate_text)?;
        let mut score = cosine_similarity(&query_embedding, &candidate_embedding);
        score += heuristic_bias(request, capability);

        if best.map(|(_, best_score)| score > best_score).unwrap_or(true) {
            best = Some((capability, score));
        }
    }

    let proposal = best
        .map(|(capability, _)| capability.name.clone())
        .ok_or_else(|| "planner failed to rank any visible capability".to_string())?;

    Ok(PlannerResponse {
        proposal,
        provider: "chapter13-planner-ai-wasi".to_string(),
        mode: "runtime-hosted-ranking".to_string(),
        model_id: manifest.model_id,
        model_revision: manifest.revision,
        model_checksum: manifest
            .files
            .into_iter()
            .find(|file| file.name == "model_quantized.onnx")
            .map(|file| file.sha256)
            .unwrap_or_else(|| "unknown".to_string()),
    })
}

fn format_candidate_text(capability: &VisibleCapability) -> String {
    format!(
        "name: {} intent: {} description: {} tags: {} runtime: {} requires: {} excludes: {}",
        capability.name,
        capability.intent,
        capability.description,
        capability.tags.join(" "),
        capability.runtime.join(" "),
        capability.requires.join(" "),
        capability.excludes.join(" ")
    )
}

fn heuristic_bias(request: &PlannerRequest, capability: &VisibleCapability) -> f32 {
    let mut bias = 0.0f32;

    if request.need == "generate-summary" {
        if request.prefer_ai && capability.name == "SummarizerAI" {
            bias += 0.35;
        }
        if !request.prefer_ai && capability.name == "SummarizerBasic" {
            bias += 0.20;
        }
    }

    if request.need == "translate-to-target-language" && capability.name == "TranslatorFr" {
        bias += 0.35;
    }

    if request.need == "provide-source-fragments" && capability.name == "DataProviderLocal" {
        bias += 0.35;
    }

    if request.need == "derive-structured-insights" && capability.name == "InsightEnricher" {
        bias += 0.35;
    }

    if request.need == "format-structured-report" && capability.name == "Formatter" {
        bias += 0.35;
    }

    if request.local_only && capability.excludes.iter().any(|item| item == "local-only") {
        bias -= 0.50;
    }

    if request.target_language == "fr" && capability.tags.iter().any(|tag| tag == "translation") {
        bias += 0.15;
    }

    bias
}

fn embed_text(
    model: &SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
    tokenizer: &WordpieceTokenizer,
    text: &str,
) -> Result<Vec<f32>, String> {
    let token_ids = tokenizer.encode(text);
    let attention_mask = vec![1i64; token_ids.len()];
    let token_type_ids = vec![0i64; token_ids.len()];

    let outputs = model
        .run(tvec![
            tract_ndarray::Array2::from_shape_vec((1, token_ids.len()), token_ids)
                .map_err(|err| err.to_string())?
                .into_tensor()
                .into(),
            tract_ndarray::Array2::from_shape_vec((1, attention_mask.len()), attention_mask.clone())
                .map_err(|err| err.to_string())?
                .into_tensor()
                .into(),
            tract_ndarray::Array2::from_shape_vec((1, token_type_ids.len()), token_type_ids)
                .map_err(|err| err.to_string())?
                .into_tensor()
                .into(),
        ])
        .map_err(|err| err.to_string())?;

    let hidden = outputs[0]
        .to_array_view::<f32>()
        .map_err(|err| err.to_string())?;
    let shape = hidden.shape().to_vec();
    let seq_len = shape
        .get(1)
        .copied()
        .ok_or_else(|| "embedding output missing seq length".to_string())?;
    let hidden_size = shape
        .get(2)
        .copied()
        .ok_or_else(|| "embedding output missing hidden size".to_string())?;

    let mut pooled = vec![0.0f32; hidden_size];
    let mut active = 0.0f32;
    for token_index in 0..seq_len {
        if attention_mask[token_index] == 0 {
            continue;
        }
        active += 1.0;
        for hidden_index in 0..hidden_size {
            pooled[hidden_index] += hidden[[0, token_index, hidden_index]];
        }
    }

    if active > 0.0 {
        for value in &mut pooled {
            *value /= active;
        }
    }

    Ok(pooled)
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut left_norm = 0.0f32;
    let mut right_norm = 0.0f32;
    for (l, r) in left.iter().zip(right.iter()) {
        dot += l * r;
        left_norm += l * l;
        right_norm += r * r;
    }
    let denom = left_norm.sqrt() * right_norm.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        dot / denom
    }
}

impl WordpieceTokenizer {
    fn load(path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path).map_err(|err| err.to_string())?;
        let mut vocab = HashMap::new();
        for (index, line) in raw.lines().enumerate() {
            vocab.insert(line.trim().to_string(), index as i64);
        }
        Ok(Self {
            unk_id: *vocab.get("[UNK]").ok_or_else(|| "missing [UNK] token".to_string())?,
            cls_id: *vocab.get("[CLS]").ok_or_else(|| "missing [CLS] token".to_string())?,
            sep_id: *vocab.get("[SEP]").ok_or_else(|| "missing [SEP] token".to_string())?,
            vocab,
        })
    }

    fn encode(&self, text: &str) -> Vec<i64> {
        let mut ids = vec![self.cls_id];
        for token in basic_tokenize(text) {
            ids.extend(self.wordpiece_ids(&token));
        }
        ids.push(self.sep_id);
        ids
    }

    fn wordpiece_ids(&self, token: &str) -> Vec<i64> {
        if let Some(id) = self.vocab.get(token) {
            return vec![*id];
        }

        let chars: Vec<char> = token.chars().collect();
        let mut pieces = Vec::new();
        let mut start = 0usize;
        while start < chars.len() {
            let mut end = chars.len();
            let mut found = None;
            while start < end {
                let mut piece: String = chars[start..end].iter().collect();
                if start > 0 {
                    piece = format!("##{piece}");
                }
                if let Some(id) = self.vocab.get(&piece) {
                    found = Some((*id, end));
                    break;
                }
                end -= 1;
            }

            if let Some((id, next)) = found {
                pieces.push(id);
                start = next;
            } else {
                return vec![self.unk_id];
            }
        }

        pieces
    }
}

fn basic_tokenize(text: &str) -> Vec<String> {
    let mut normalized = String::new();
    for ch in text.chars().flat_map(|c| c.to_lowercase()) {
        if ch.is_alphanumeric() {
            normalized.push(ch);
        } else {
            normalized.push(' ');
        }
    }
    normalized
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn load_manifest(model_dir: &Path) -> Result<ModelManifest, String> {
    let raw = fs::read_to_string(model_dir.join("manifest.json")).map_err(|err| err.to_string())?;
    serde_json::from_str(&raw).map_err(|err| err.to_string())
}
