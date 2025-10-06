use serde::{Serialize};
use serde_json::Value;

use crate::adapter_manager::AdapterBinding;
use service::model::Event;

/// Lifecycle record persisted after each run.  Matches the
/// `metadata.schema.json` contract.
#[derive(Serialize)]
pub struct LifecycleRecord {
    pub service: String,
    pub service_version: String,
    pub policy_ref: String,
    pub bindings: Bindings,
    pub events: Vec<Event>,
    pub state: String,
    #[serde(rename = "logicalClock")]
    pub logical_clock: u64,
}

#[derive(Serialize)]
pub struct Bindings {
    #[serde(rename = "network.fetch")]
    pub network_fetch: BindingImpl,
}

#[derive(Serialize)]
pub struct BindingImpl {
    /// The adapter implementation name.  Use a raw identifier rename to avoid
    /// clashing with the Rust keyword `impl`.
    #[serde(rename = "impl")]
    pub impl_name: String,
    pub host: String,
}

impl LifecycleRecord {
    /// Construct a new lifecycle record from the given parameters.  This helper
    /// adapts the `AdapterBinding` into the shape expected by the schema.
    pub fn new(
        service: &str,
        version: &str,
        policy_ref: &str,
        adapter_binding: &AdapterBinding,
        events: Vec<Event>,
        state: &str,
        logical_clock: u64,
    ) -> Self {
        Self {
            service: service.to_string(),
            service_version: version.to_string(),
            policy_ref: policy_ref.to_string(),
            bindings: Bindings {
                network_fetch: BindingImpl {
                    impl_name: adapter_binding.impl_name.clone(),
                    host: adapter_binding.host.clone(),
                },
            },
            events,
            state: state.to_string(),
            logical_clock,
        }
    }

    /// Convert the lifecycle record into a JSON value.
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("LifecycleRecord should serialize")
    }
}