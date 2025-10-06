//! WASI HTTP network adapter.  This adapter uses the experimental
//! `wasi-experimental-http-client` crate to perform outbound HTTP
//! requests when running under a WASI runtime that implements the
//! `wasi:http` proposal.  If your runtime does not provide HTTP
//! support, the adapter will behave like any other network adapter and
//! return an error on failure.

use service::api::{NetworkAdapter, NetworkResponse};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// The WASI HTTP adapter.  This adapter is only compiled on the
/// `wasm32` architecture.  It leverages the
/// [`wasi-experimental-http-client`](https://crates.io/crates/wasi-experimental-http-client)
/// crate to issue outbound HTTP requests.  The crate exposes a
/// `request` function that accepts an [`http::Request`]
/// and returns an [`http::Response`].  We convert between the
/// service’s `NetworkResponse` type and the HTTP types here.  If the
/// runtime does not support outgoing requests, an error is returned.
#[cfg(target_arch = "wasm32")]
pub struct WasiHttpAdapter;

#[cfg(target_arch = "wasm32")]
impl NetworkAdapter for WasiHttpAdapter {
    fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        use http::{Request, header::HeaderName, header::HeaderValue};
        use wasi_experimental_http_client::request as wasi_request;

        // Build a GET request with the provided headers.  The body is empty
        // because only GET requests are supported in this example.
        let mut req_builder = Request::builder()
            .method("GET")
            .uri(url);
        // Attach headers to the request.  Invalid header names or values
        // result in an error.
        for (k, v) in headers {
            let name = HeaderName::from_bytes(k.as_bytes()).map_err(|e| anyhow!(e.to_string()))?;
            let value = HeaderValue::from_str(v).map_err(|e| anyhow!(e.to_string()))?;
            req_builder = req_builder.header(name, value);
        }
        let request = req_builder.body(Vec::new()).map_err(|e| anyhow!(e.to_string()))?;

        // Dispatch the request.  The wasi-experimental-http-client crate
        // performs the network call via the WASI HTTP preview interface.
        let response = wasi_request(request).map_err(|e| anyhow!(e.to_string()))?;
        let status = response.status().as_u16();

        // Collect response headers into a HashMap<String, String>.
        let mut resp_headers = HashMap::new();
        for (name, value) in response.headers().iter() {
            let key = name.to_string();
            let val = value.to_str().unwrap_or("").to_string();
            resp_headers.insert(key, val);
        }

        // The response body is a vector of bytes.  Convert to a UTF‑8
        // string.  If the body is not valid UTF‑8, return an error.
        let body_bytes = response
            .into_body()
            .map_err(|e| anyhow!(e.to_string()))?;
        let body = String::from_utf8(body_bytes).map_err(|e| anyhow!(e.to_string()))?;

        Ok(NetworkResponse { status, headers: resp_headers, body })
    }
}