//! Core service logic for the UMA post fetcher.  This crate contains pure
//! functions that do not depend on timers, randomness or host capabilities.  The
//! runtime crate orchestrates adapter binding, event emission and lifecycle
//! persistence around these pure functions.

pub mod api;
pub mod model;

use api::{NetworkAdapter, NetworkResponse};
use model::{Post};
use serde_json::Value;
use anyhow::{anyhow, Result};

/// Normalise a JSONPlaceholder post into a canonical shape.  The input must be
/// a JSON object with `id`, `userId`, `title` and `body` fields.  Returns
/// `None` if any of the required fields are missing or have the wrong type.
pub fn normalize_post(json: &Value) -> Option<Post> {
    let id = json.get("id")?.as_u64()?;
    let user_id = json.get("userId")?.as_u64()?;
    let title = json.get("title")?.as_str()?.to_string();
    let body = json.get("body")?.as_str()?.to_string();
    Some(Post { id, user_id, title, body })
}

/// Pure helper to extract an error value for error events.  Returns a string
/// describing the error given a status code or a parsing failure.
pub fn error_message(status: Option<u16>, parse_error: Option<&serde_json::Error>) -> String {
    if let Some(code) = status {
        format!("status {}", code)
    } else if let Some(err) = parse_error {
        format!("parse error: {}", err)
    } else {
        "unknown error".to_string()
    }
}