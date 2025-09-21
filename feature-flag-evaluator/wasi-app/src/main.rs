//! WASI executable for the feature flag evaluator.
//!
//! This binary reads a single UTF‑8 JSON document from standard input, evaluates the
//! contained flag against the provided context using the core library, and writes a
//! single UTF‑8 JSON document to standard output.  On parse error it exits with
//! code 1; on success it exits with code 0.

use ff_eval_core::{Context, EvalResult, Flag, Rule, Value};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{Read, Write};

/// Representation of the input JSON for serde deserialization.
#[derive(Debug, Deserialize)]
struct Input {
    flag: FlagJson,
    context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct FlagJson {
    key: String,
    rules: Vec<RuleJson>,
    #[serde(default)]
    default: bool,
}

#[derive(Debug, Deserialize)]
struct RuleJson {
    #[serde(rename = "if")]
    cond: String,
    #[serde(rename = "then")]
    then_value: bool,
}

fn main() {
    // Read all input from stdin into a string.
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_to_string(&mut input) {
        std::process::exit(1);
    }
    // Deserialize the JSON input.  On failure, exit with status 1.
    let parsed: Input = match serde_json::from_str(&input) {
        Ok(val) => val,
        Err(_) => {
            std::process::exit(1);
        }
    };
    // Convert to core types.
    let flag = Flag {
        key: parsed.flag.key,
        rules: parsed
            .flag
            .rules
            .into_iter()
            .map(|r| Rule {
                cond: r.cond,
                then_value: r.then_value,
            })
            .collect(),
        default: parsed.flag.default,
    };
    let mut ctx: Context = Context::new();
    for (k, v) in parsed.context.into_iter() {
        let value = match v {
            serde_json::Value::String(s) => Value::Str(s),
            serde_json::Value::Number(n) => {
                // Convert numbers to f64; JSON numbers may not always fit in f64 but this is sufficient for this example.
                Value::Num(n.as_f64().unwrap_or(0.0))
            }
            serde_json::Value::Bool(b) => Value::Bool(b),
            _ => Value::Null,
        };
        ctx.insert(k, value);
    }
    // Evaluate the flag.
    let result: EvalResult = ff_eval_core::eval_flag(&flag, &ctx);
    // Construct the output JSON.
    let output = serde_json::json!({
        "key": result.key,
        "enabled": result.enabled,
        "matchedRule": result.matched_rule.map(|i| i as i64),
    });
    // Write the JSON to stdout.
    if let Err(_) = std::io::stdout().write_all(output.to_string().as_bytes()) {
        std::process::exit(1);
    }
    std::process::exit(0);
}