// Unit tests for the service crate.
use super::*;
use crate::api::{fetch_json, NetworkAdapter, NetworkResponse};
use anyhow::{anyhow, Result};
use serde_json::json;
use std::collections::HashMap;

struct StubAdapter {
    response: Result<NetworkResponse>,
}

impl NetworkAdapter for StubAdapter {
    fn fetch(&self, _url: &str, _headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        match &self.response {
            Ok(resp) => Ok(NetworkResponse {
                status: resp.status,
                headers: resp.headers.clone(),
                body: resp.body.clone(),
            }),
            Err(err) => Err(anyhow!(err.to_string())),
        }
    }
}

#[test]
fn test_normalize_post_success() {
    let input = json!({
        "id": 42,
        "userId": 7,
        "title": "Hello",
        "body": "World",
    });
    let post = normalize_post(&input).expect("should parse correctly");
    assert_eq!(post.id, 42);
    assert_eq!(post.user_id, 7);
    assert_eq!(post.title, "Hello");
    assert_eq!(post.body, "World");
}

#[test]
fn test_normalize_post_missing_fields() {
    let input = json!({
        "id": 1,
        "title": "Missing userId and body"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_error_message_status() {
    let msg = error_message(Some(404), None);
    assert_eq!(msg, "status 404");
}

#[test]
fn test_error_message_parse_error() {
    let parse_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
    let msg = error_message(None, Some(&parse_err));
    assert!(msg.starts_with("parse error"));
}

#[test]
fn test_error_message_unknown_error() {
    let msg = error_message(None, None);
    assert_eq!(msg, "unknown error");
}

#[test]
fn test_normalize_post_wrong_field_types() {
    let input = json!({
        "id": "42",
        "userId": 7,
        "title": "Hello",
        "body": "World"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_missing_id() {
    let input = json!({
        "userId": 7,
        "title": "Hello",
        "body": "World"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_missing_title() {
    let input = json!({
        "id": 42,
        "userId": 7,
        "body": "World"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_missing_body() {
    let input = json!({
        "id": 42,
        "userId": 7,
        "title": "Hello"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_wrong_user_id_type() {
    let input = json!({
        "id": 42,
        "userId": "7",
        "title": "Hello",
        "body": "World"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_wrong_title_type() {
    let input = json!({
        "id": 42,
        "userId": 7,
        "title": 99,
        "body": "World"
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_normalize_post_wrong_body_type() {
    let input = json!({
        "id": 42,
        "userId": 7,
        "title": "Hello",
        "body": false
    });
    assert!(normalize_post(&input).is_none());
}

#[test]
fn test_fetch_json_success() {
    let adapter = StubAdapter {
        response: Ok(NetworkResponse {
            status: 200,
            headers: HashMap::new(),
            body: r#"{"ok":true,"count":2}"#.to_string(),
        }),
    };
    let headers = HashMap::new();
    let (status, value) = fetch_json(&adapter, "https://example.test/data", &headers).unwrap();
    assert_eq!(status, 200);
    assert_eq!(value["ok"], true);
    assert_eq!(value["count"], 2);
}

#[test]
fn test_fetch_json_propagates_adapter_error() {
    let adapter = StubAdapter {
        response: Err(anyhow!("network unavailable")),
    };
    let headers = HashMap::new();
    let error = fetch_json(&adapter, "https://example.test/data", &headers).unwrap_err();
    assert!(error.to_string().contains("network unavailable"));
}

#[test]
fn test_fetch_json_rejects_invalid_json() {
    let adapter = StubAdapter {
        response: Ok(NetworkResponse {
            status: 200,
            headers: HashMap::new(),
            body: "not json".to_string(),
        }),
    };
    let headers = HashMap::new();
    let error = fetch_json(&adapter, "https://example.test/data", &headers).unwrap_err();
    assert!(error.to_string().contains("expected ident") || error.to_string().contains("expected value"));
}
