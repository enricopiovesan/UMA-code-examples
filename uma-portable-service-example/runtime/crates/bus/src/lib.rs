
use anyhow::{Context, Result};
use jsonschema::{Draft, JSONSchema};
use serde::Serialize;
use tracing::info;

pub struct JsonlBus;

fn schema_for<'a>(c: &'a contract::Contract, event: &str) -> Result<&'a serde_json::Value> {
    c.events.iter()
        .find(|e| e.name == event)
        .map(|e| &e.schema)
        .context(format!("schema not found for event '{}'", event))
}

pub fn format_event<T: Serialize>(event_name: &str, payload: &T) -> Result<String> {
    let wrapper = serde_json::json!({
        "event": event_name,
        "payload": payload,
    });
    Ok(serde_json::to_string(&wrapper)?)
}

pub fn publish_validated<T: Serialize>(c: &contract::Contract, event: &str, payload: &T) -> Result<()> {
    let schema_val = schema_for(c, event)?;
    let compiled = JSONSchema::options().with_draft(Draft::Draft7).compile(schema_val)?;
    let json = serde_json::to_value(payload)?;
    compiled.validate(&json)
        .map_err(|errs| anyhow::anyhow!(format!("payload failed schema: {:#?}", errs.collect::<Vec<_>>())))?;
    let line = format_event(event, &payload)?;
    // Distinguish events from logs in stdout
    println!("{}", line);
    info!(target: "uma.bus", event = event, "published");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct P { a: i32 }

    #[test]
    fn formats_wrapper() {
        let s = format_event("x.y", &P{a:1}).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["event"], "x.y");
        assert_eq!(v["payload"]["a"], 1);
    }
}
