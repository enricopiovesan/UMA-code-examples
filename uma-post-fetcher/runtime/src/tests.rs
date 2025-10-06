// Unit tests for the runtime crate.  These tests verify the behaviour of
// the event bus, lifecycle record and run_json function with a stub
// adapter.  Note: the WASI HTTP adapter is not exercised here.

use super::*;
use service::api::{NetworkAdapter, NetworkResponse};
use serde_json::{json, Value};
use std::collections::HashMap;

// A dummy network adapter for testing.  Returns a fixed JSON body.
struct DummyAdapter;

impl NetworkAdapter for DummyAdapter {
    fn fetch(&self, _url: &str, _headers: &HashMap<String, String>) -> anyhow::Result<NetworkResponse> {
        let body = r#"{\"id\":1,\"userId\":2,\"title\":\"t\",\"body\":\"b\"}"#;
        Ok(NetworkResponse {
            status: 200,
            headers: HashMap::new(),
            body: body.to_string(),
        })
    }
}

#[test]
fn test_event_bus_increment() {
    let mut bus = event_bus::EventBus::new();
    assert_eq!(bus.clock, 0);
    bus.emit("start", json!({}));
    assert_eq!(bus.clock, 1);
    bus.emit("next", json!({}));
    assert_eq!(bus.clock, 2);
    assert_eq!(bus.events.len(), 2);
}

#[test]
fn test_lifecycle_record() {
    let binding = adapter_manager::AdapterBinding { impl_name: "test-impl".to_string(), host: "native".to_string() };
    let events = vec![service::model::Event { t: "0".to_string(), type_: "start".to_string(), data: json!({}) }];
    let rec = metadata::LifecycleRecord::new(
        "svc",
        "0.1",
        "policy",
        &binding,
        events.clone(),
        "terminated",
        events.len() as u64,
    );
    let v = rec.to_json();
    assert_eq!(v["service"], "svc");
    assert_eq!(v["bindings"]["network.fetch"]["impl"], "test-impl");
}

#[test]
fn test_run_json_with_dummy_adapter() {
    let input = json!({
        "request": { "url": "https://example.com", "headers": {} },
        "runId": "run-1"
    });
    let input_str = serde_json::to_string(&input).unwrap();
    let adapter = DummyAdapter;
    let (out_json, meta_json) = run_json(&input_str, Some(Box::new(adapter))).expect("run_json should succeed");
    let out_val: Value = serde_json::from_str(&out_json).unwrap();
    let post = out_val.get("normalizedPost").unwrap();
    assert_eq!(post["id"], 1);
    assert_eq!(post["userId"], 2);
    assert_eq!(out_val["events"].as_array().unwrap().len(), 5);
    let meta_val: Value = serde_json::from_str(&meta_json).unwrap();
    assert_eq!(meta_val["logicalClock"], 5);
}

#[test]
fn test_header_validation_and_final_state() {
    // The runtime should emit an error event and set the final state to
    // "failed" when unexpected headers are present.  Use a dummy adapter
    // that returns a successful response so that the only error comes from
    // header validation.
    use std::env;
    let input = json!({
        "request": { "url": "https://example.com", "headers": { "x-foo": "bar" } },
        "runId": "run-2"
    });
    let input_str = serde_json::to_string(&input).unwrap();
    let adapter = DummyAdapter;
    let (out_json, meta_json) = run_json(&input_str, Some(Box::new(adapter))).expect("run_json should succeed");
    let out_val: Value = serde_json::from_str(&out_json).unwrap();
    // normalised post may be present because dummy adapter returns a valid body
    assert!(out_val.get("events").unwrap().as_array().unwrap().iter().any(|e| e["type"] == "error"));
    let meta_val: Value = serde_json::from_str(&meta_json).unwrap();
    assert_eq!(meta_val["state"], "failed");
}

#[test]
fn test_adapter_manager_env_wrappers() {
    // Test that environment variables cause the adapter manager to wrap
    // adapters in retry and cache wrappers.  The binding impl_name should
    // reflect the applied wrappers.
    use std::env;
    // Clear any existing variables to start from a clean state.
    env::remove_var("UMA_ENABLE_RETRY");
    env::remove_var("UMA_ENABLE_CACHE");
    // No wrappers when variables unset
    let mgr = adapter_manager::AdapterManager::new(None);
    assert_eq!(mgr.binding.impl_name.contains("retry"), false);
    assert_eq!(mgr.binding.impl_name.contains("cache"), false);
    // Enable retry
    env::set_var("UMA_ENABLE_RETRY", "1");
    let mgr_retry = adapter_manager::AdapterManager::new(None);
    assert!(mgr_retry.binding.impl_name.contains("retry"));
    env::remove_var("UMA_ENABLE_RETRY");
    // Enable cache
    env::set_var("UMA_ENABLE_CACHE", "1");
    let mgr_cache = adapter_manager::AdapterManager::new(None);
    assert!(mgr_cache.binding.impl_name.contains("cache"));
    // Enable both
    env::set_var("UMA_ENABLE_RETRY", "1");
    let mgr_both = adapter_manager::AdapterManager::new(None);
    assert!(mgr_both.binding.impl_name.contains("retry"));
    assert!(mgr_both.binding.impl_name.contains("cache"));
    // Clean up
    env::remove_var("UMA_ENABLE_RETRY");
    env::remove_var("UMA_ENABLE_CACHE");
}