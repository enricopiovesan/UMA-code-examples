//! UMA runtime implementation for the post fetcher example.  This module
//! orchestrates contract loading, adapter binding, deterministic event
//! emission, lifecycle metadata and service execution.

mod loader;
mod adapter_manager;
mod thread_manager;
mod event_bus;
mod metadata;
mod wasi_http_adapter;
mod retry_adapter;
mod cache_adapter;

use crate::adapter_manager::AdapterManager;
use crate::event_bus::EventBus;
use crate::thread_manager::ThreadManager;
use crate::metadata::LifecycleRecord;

use serde_json::{json, Value};
use anyhow::{Result};
use service::api::NetworkAdapter;
use service::model::{Input, Output, Event, Post};
use service::{normalize_post, error_message};

/// Run the UMA post fetcher with the given JSON input.  Returns a pair of
/// strings: the service output JSON and the lifecycle metadata JSON.  The
/// runtime is deterministic: given the same input and adapter implementation
/// it will emit the same sequence of events and the same logical clock.
pub fn run_json(input_json: &str, adapter: Option<Box<dyn NetworkAdapter>>) -> Result<(String, String)> {
    // Parse the input according to the service contract.
    let input: Input = serde_json::from_str(input_json)?;

    let thread_manager = ThreadManager::new();
    let mut event_bus = EventBus::new();
    // Emit start event
    event_bus.emit("start", json!({ "runId": input.run_id.clone() }));

    // Validate request headers before proceeding.  Only allow a small set of
    // recognised header names and values under 1024 characters.  If
    // validation fails, emit an error and skip the network fetch.
    let allowed_headers = ["accept", "content-type", "authorization"];
    for (key, value) in &input.request.headers {
        let lower = key.to_ascii_lowercase();
        if !allowed_headers.contains(&lower.as_str()) {
            event_bus.emit("error", json!({ "error": format!("unexpected header {}", key) }));
        }
        if value.len() > 1024 {
            event_bus.emit("error", json!({ "error": format!("header {} too long", key) }));
        }
    }

    // Prepare variables for the normalised post and final state.  The final
    // state will be set to "failed" if any error events are emitted.
    let mut normalized_post: Option<Post> = None;
    let mut final_state = "terminated".to_string();

    // Instantiate the adapter manager and issue the fetch request.
    let adapter_manager = AdapterManager::new(adapter);
    // Record fetch_request event
    event_bus.emit("fetch_request", json!({ "url": input.request.url.clone() }));
    // Perform network request.  Capture status and body.
    let fetch_result = thread_manager.run_sync(|| {
        adapter_manager.fetch(&input.request.url, &input.request.headers)
    });
    match fetch_result {
        Ok(resp) => {
            // Emit fetch_response event
            event_bus.emit("fetch_response", json!({ "status": resp.status }));
            // Parse body into JSON
            let body_str = resp.body;
            let value: Result<Value, _> = serde_json::from_str(&body_str);
            match value {
                Ok(json_val) => {
                    // Normalise the post
                    normalized_post = normalize_post(&json_val);
                    if let Some(ref post) = normalized_post {
                        event_bus.emit("normalized", json!({ "id": post.id }));
                    } else {
                        // Emit parse error event when fields missing
                        let err_msg = error_message(Some(resp.status), None);
                        event_bus.emit("error", json!({ "error": err_msg }));
                    }
                }
                Err(parse_err) => {
                    // Invalid JSON
                    let err_msg = error_message(Some(resp.status), Some(&parse_err));
                    event_bus.emit("error", json!({ "error": err_msg }));
                    normalized_post = None;
                }
            }
        }
        Err(err) => {
            // Network error
            let err_msg = err.to_string();
            event_bus.emit("fetch_response", json!({ "status": 0 }));
            event_bus.emit("error", json!({ "error": err_msg }));
            normalized_post = None;
        }
    }

    // Determine final state based on whether any error events were emitted.
    if event_bus.events.iter().any(|e| e.type_ == "error") {
        final_state = "failed".to_string();
    }

    // End event
    event_bus.emit("end", json!({}));

    // Build service output
    let output = Output {
        normalized_post,
        events: event_bus.events.clone(),
    };
    let output_json = serde_json::to_string(&output)?;

    // Build lifecycle record
    let lifecycle = LifecycleRecord::new(
        "uma-post-fetcher.service",
        "1.0.0",
        "default.runtime.policy",
        &adapter_manager.binding,
        event_bus.events.clone(),
        &final_state,
        event_bus.clock,
    );
    let lifecycle_json = serde_json::to_string(&lifecycle.to_json())?;

    Ok((output_json, lifecycle_json))
}