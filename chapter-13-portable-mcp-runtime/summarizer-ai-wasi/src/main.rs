use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tract_onnx::prelude::*;

#[derive(Debug, Deserialize)]
struct SummarizerRequest {
    project_name: String,
    structured_facts: Vec<String>,
    model_dir: String,
    max_length: Option<usize>,
}

#[derive(Debug, Serialize)]
struct SummarizerResponse {
    summary: String,
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
    let request: SummarizerRequest = serde_json::from_str(&raw).map_err(|err| err.to_string())?;
    let response = run(&request)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&response).map_err(|err| err.to_string())?
    );
    Ok(())
}

fn run(request: &SummarizerRequest) -> Result<SummarizerResponse, String> {
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

    let document_text = format!("{} {}", request.project_name, request.structured_facts.join(" "));
    let document_embedding = embed_text(&model, &tokenizer, &document_text)?;

    let mut ranked = Vec::new();
    for (index, fact) in request.structured_facts.iter().enumerate() {
        let embedding = embed_text(&model, &tokenizer, fact)?;
        let score = cosine_similarity(&document_embedding, &embedding);
        ranked.push((index, score, fact.clone()));
    }

    ranked.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap_or(std::cmp::Ordering::Equal));
    let keep = request.max_length.unwrap_or(2).clamp(1, request.structured_facts.len());
    let mut selected = ranked.into_iter().take(keep).collect::<Vec<_>>();
    selected.sort_by_key(|item| item.0);

    let summary = selected
        .into_iter()
        .map(|(_, _, fact)| fact)
        .collect::<Vec<_>>()
        .join(" ");

    Ok(SummarizerResponse {
        summary,
        provider: "chapter13-summarizer-ai-wasi".to_string(),
        mode: "runtime-hosted-extractive".to_string(),
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
    let seq_len = shape.get(1).copied().ok_or_else(|| "embedding output missing seq length".to_string())?;
    let hidden_size = shape.get(2).copied().ok_or_else(|| "embedding output missing hidden size".to_string())?;

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
        .map(|token| token.to_string())
        .collect()
}

fn load_manifest(model_dir: &Path) -> Result<ModelManifest, String> {
    let raw = fs::read_to_string(model_dir.join("manifest.json")).map_err(|err| err.to_string())?;
    serde_json::from_str(&raw).map_err(|err| err.to_string())
}
