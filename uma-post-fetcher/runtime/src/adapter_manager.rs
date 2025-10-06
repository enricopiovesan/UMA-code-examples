use service::api::{NetworkAdapter, NetworkResponse};

#[cfg(target_arch = "wasm32")]
use crate::wasi_http_adapter::WasiHttpAdapter;
use serde_json::Value;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::retry_adapter::RetryAdapter;
use crate::cache_adapter::CacheAdapter;

/// Metadata persisted about the adapter selection.  Records which
/// implementation was chosen and the host environment.
#[derive(Debug, Clone)]
pub struct AdapterBinding {
    pub impl_name: String,
    pub host: String,
}

/// A simple adapter manager that selects a concrete network adapter at
/// runtime.  For non‑wasm targets this uses a host fetch implementation
/// (`reqwest`).  For wasm targets, it expects the host to provide an
/// externally‑defined implementation via imports; in that case this struct
/// merely holds a reference.
pub struct AdapterManager {
    adapter: Box<dyn NetworkAdapter>,
    pub binding: AdapterBinding,
}

impl AdapterManager {
    /// Create a new adapter manager by selecting the appropriate adapter.
    /// On non‑wasm targets this constructs a `HostFetchAdapter`.  On wasm
    /// targets, the caller must supply a host‑provided implementation.
    pub fn new(adapter: Option<Box<dyn NetworkAdapter>>) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            // On wasm targets we attempt to use a WASI HTTP adapter if none was
            // explicitly provided.  This requires a host runtime that
            // implements the `wasi:http` proposal.  Failing that, a
            // host-provided adapter must be supplied.
            if let Some(adapter) = adapter {
                let binding = AdapterBinding { impl_name: "custom".to_string(), host: "wasm32".to_string() };
                return Self { adapter, binding };
            }
            // Attempt to select a WasiHttpAdapter.  Note that this adapter
            // currently returns an error because the WASI HTTP API is not
            // implemented in this example.  If you enable a working
            // implementation, set the impl_name accordingly.
            let adapter = Box::new(crate::wasi_http_adapter::WasiHttpAdapter {}) as Box<dyn NetworkAdapter>;
            let binding = AdapterBinding { impl_name: "wasi-http".to_string(), host: "wasm32".to_string() };
            return Self { adapter, binding };
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(adapter) = adapter {
                // Respect externally provided adapter but apply optional retry/cache wrappers.
                let mut adapter: Box<dyn NetworkAdapter> = adapter;
                let mut impl_name = "custom".to_string();
                let enable_retry = std::env::var("UMA_ENABLE_RETRY").is_ok();
                let enable_cache = std::env::var("UMA_ENABLE_CACHE").is_ok();
                if enable_retry {
                    adapter = Box::new(RetryAdapter::new(adapter, 3));
                    impl_name = format!("retry-{}", impl_name);
                }
                if enable_cache {
                    adapter = Box::new(CacheAdapter::new(adapter));
                    impl_name = format!("cache-{}", impl_name);
                }
                let binding = AdapterBinding { impl_name, host: std::env::consts::OS.to_string() };
                return Self { adapter, binding };
            }
            // Default host fetch adapter with optional wrappers.
            let mut adapter: Box<dyn NetworkAdapter> = Box::new(HostFetchAdapter {});
            let mut impl_name = "host-fetch".to_string();
            let enable_retry = std::env::var("UMA_ENABLE_RETRY").is_ok();
            let enable_cache = std::env::var("UMA_ENABLE_CACHE").is_ok();
            if enable_retry {
                adapter = Box::new(RetryAdapter::new(adapter, 3));
                impl_name = format!("retry-{}", impl_name);
            }
            if enable_cache {
                adapter = Box::new(CacheAdapter::new(adapter));
                impl_name = format!("cache-{}", impl_name);
            }
            let binding = AdapterBinding { impl_name, host: std::env::consts::OS.to_string() };
            Self { adapter, binding }
        }
    }

    /// Perform a network fetch.  Delegates to the underlying adapter.
    pub fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        self.adapter.fetch(url, headers)
    }
}

/// A simple host fetch adapter using `reqwest::blocking`.  Only available on
/// non‑wasm targets.
#[cfg(not(target_arch = "wasm32"))]
pub struct HostFetchAdapter;

#[cfg(not(target_arch = "wasm32"))]
impl NetworkAdapter for HostFetchAdapter {
    fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        // Use reqwest::blocking to perform a GET request.
        // Note: for demonstration purposes only; proper error handling and
        // limits should be implemented in a real adapter.
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