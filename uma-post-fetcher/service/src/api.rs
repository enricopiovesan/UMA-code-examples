use anyhow::Result;
use serde_json::Value;

/// Trait representing a network capability.  The UMA runtime will provide an implementation
/// of this trait at runtime, either via a `wasi-http` binding or a host‑provided fetch.
pub trait NetworkAdapter {
    fn fetch(&self, url: &str, headers: &std::collections::HashMap<String, String>) -> Result<NetworkResponse>;
}

/// Response returned by the network adapter.  The body is returned as a string to
/// simplify JSON parsing; if the underlying implementation returns bytes, it should
/// decode them as UTF‑8.
pub struct NetworkResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}

/// Fetch a JSON document from the given URL using the provided network adapter and
/// parse it into a JSON value.  Returns the status code and the parsed value on success.
pub fn fetch_json<A: NetworkAdapter>(
    adapter: &A,
    url: &str,
    headers: &std::collections::HashMap<String, String>,
) -> Result<(u16, Value)> {
    let resp = adapter.fetch(url, headers)?;
    let status = resp.status;
    let body = resp.body;
    let value: Value = serde_json::from_str(&body)?;
    Ok((status, value))
}