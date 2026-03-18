use anyhow::Result;
use serde_json::json;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let (output_json, lifecycle_json) = uma_runtime::run_json(&input, None)?;
    let output: serde_json::Value = serde_json::from_str(&output_json)?;
    let lifecycle: serde_json::Value = serde_json::from_str(&lifecycle_json)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "output": output,
            "lifecycle": lifecycle,
        }))?
    );
    Ok(())
}
