use anyhow::Result;
use std::fs;
use std::path::Path;
use serde_json::Value;

/// Simple loader for JSON contracts.  Given a path relative to the project
/// root, reads and parses the file into a `serde_json::Value`.  In a full
/// implementation this would perform schema validation and report helpful
/// diagnostics.
pub fn load_contract<P: AsRef<Path>>(path: P) -> Result<Value> {
    let contents = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&contents)?;
    Ok(value)
}