use serde::{Deserialize, Serialize};

/// Represents the JSON structure of the incoming request.
#[derive(Debug, Deserialize)]
pub struct Input {
    pub request: Request,
    #[serde(rename = "runId")]
    pub run_id: String,
}

/// HTTP request parameters (currently only URL and optional headers).
#[derive(Debug, Deserialize)]
pub struct Request {
    pub url: String,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

/// Canonical representation of a Post from JSONPlaceholder.
#[derive(Debug, Serialize)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub body: String,
}

/// An event in the deterministic event log.
#[derive(Debug, Serialize)]
pub struct Event {
    pub t: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: serde_json::Value,
}

/// Output returned by the service.  Either `normalized_post` or `null`, plus the event log.
#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(rename = "normalizedPost")]
    pub normalized_post: Option<Post>,
    pub events: Vec<Event>,
}