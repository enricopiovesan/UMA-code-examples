//! A simple in-memory caching adapter.  Wraps another network adapter and
//! caches responses by URL.  Only the first request for a given URL hits
//! the underlying adapter; subsequent requests return the cached
//! response.  The cache persists for the lifetime of the adapter.

use service::api::{NetworkAdapter, NetworkResponse};
use anyhow::Result;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct CacheAdapter {
    inner: Box<dyn NetworkAdapter>,
    cache: RefCell<HashMap<String, NetworkResponse>>, 
}

impl CacheAdapter {
    pub fn new(inner: Box<dyn NetworkAdapter>) -> Self {
        Self { inner, cache: RefCell::new(HashMap::new()) }
    }
}

impl NetworkAdapter for CacheAdapter {
    fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        if let Some(resp) = self.cache.borrow().get(url) {
            // Return a clone of the cached response.
            return Ok(NetworkResponse {
                status: resp.status,
                headers: resp.headers.clone(),
                body: resp.body.clone(),
            });
        }
        let resp = self.inner.fetch(url, headers)?;
        self.cache.borrow_mut().insert(url.to_string(), NetworkResponse {
            status: resp.status,
            headers: resp.headers.clone(),
            body: resp.body.clone(),
        });
        Ok(resp)
    }
}