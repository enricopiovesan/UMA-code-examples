//! WASI HTTP network adapter.
//!
//! This example keeps the adapter shape in place for UMA runtime selection,
//! but does not ship a concrete WASI HTTP client integration.  When selected
//! under `wasm32`, the adapter returns a deterministic error explaining that
//! outbound HTTP is not wired in for this sample.

#[cfg(target_arch = "wasm32")]
use anyhow::{anyhow, Result};
#[cfg(target_arch = "wasm32")]
use service::api::{NetworkAdapter, NetworkResponse};
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

/// The WASI HTTP adapter.  This adapter is only compiled on the
/// `wasm32` architecture.  The current sample leaves outbound HTTP to
/// host-provided adapters, so this implementation fails closed with a
/// stable error instead of attempting an unavailable preview API.
#[cfg(target_arch = "wasm32")]
pub struct WasiHttpAdapter;

#[cfg(target_arch = "wasm32")]
impl NetworkAdapter for WasiHttpAdapter {
    fn fetch(&self, _url: &str, _headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        Err(anyhow!(
            "wasi-http adapter is not implemented in this example; provide a host adapter instead"
        ))
    }
}
