//! Host network adapter implementation.  This crate provides a
//! `HostFetch` struct implementing the `NetworkAdapter` trait defined in the
//! `service` crate.  It uses `reqwest::blocking` to perform synchronous
//! HTTP GET requests on native targets.  When compiled to WebAssembly,
//! callers must supply an alternative implementation (this crate will not
//! compile to wasm32 by default).

use service::api::{NetworkAdapter, NetworkResponse};
use anyhow::Result;
use std::collections::HashMap;

pub struct HostFetch;

impl NetworkAdapter for HostFetch {
    fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        let client = reqwest::blocking::Client::new();
        let mut req = client.get(url);
        for (k, v) in headers {
            req = req.header(k.as_str(), v.as_str());
        }
        let resp = req.send()?;
        let status = resp.status().as_u16();
        let mut resp_headers = HashMap::new();
        for (k, v) in resp.headers().iter() {
            let val = v.to_str().unwrap_or("").to_string();
            resp_headers.insert(k.to_string(), val);
        }
        let body = resp.text()?;
        Ok(NetworkResponse { status, headers: resp_headers, body })
    }
}