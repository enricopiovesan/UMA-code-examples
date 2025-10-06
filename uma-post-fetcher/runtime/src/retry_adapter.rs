//! A wrapper adapter that retries failed network requests.  Retries are
//! deterministic: the maximum number of retries and retry behaviour are
//! fixed by configuration.  Backoff delays are not implemented in this
//! example because the runtime must remain deterministic and avoid
//! timers.

use service::api::{NetworkAdapter, NetworkResponse};
use anyhow::{Result};
use std::collections::HashMap;

pub struct RetryAdapter {
    inner: Box<dyn NetworkAdapter>,
    max_retries: u32,
}

impl RetryAdapter {
    pub fn new(inner: Box<dyn NetworkAdapter>, max_retries: u32) -> Self {
        Self { inner, max_retries }
    }
}

impl NetworkAdapter for RetryAdapter {
    fn fetch(&self, url: &str, headers: &HashMap<String, String>) -> Result<NetworkResponse> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            match self.inner.fetch(url, headers) {
                Ok(resp) => {
                    // Consider any 2xx status a success.
                    if resp.status >= 200 && resp.status < 300 {
                        return Ok(resp);
                    }
                    if attempts > self.max_retries {
                        return Ok(resp);
                    }
                }
                Err(err) => {
                    if attempts > self.max_retries {
                        return Err(err);
                    }
                }
            }
        }
    }
}