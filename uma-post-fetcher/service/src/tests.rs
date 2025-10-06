// Unit tests for the service crate.
use super::*;
use serde_json::json;

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