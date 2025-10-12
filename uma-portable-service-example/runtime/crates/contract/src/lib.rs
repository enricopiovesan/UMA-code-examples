
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capability {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventDef {
    pub name: String,
    pub schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionConstraints {
    pub constraints: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    pub service: ServiceInfo,
    pub capabilities: Vec<Capability>,
    pub events: Vec<EventDef>,
    pub execution: ExecutionConstraints,
        #[serde(default)]
        pub parameters: serde_json::Value,
}

impl Contract {
    pub fn load_from(path: &str) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let c: Contract = serde_json::from_str(&data)?;
        Ok(c)
    }
}


impl Contract {
    pub fn parameters(&self) -> Option<&serde_json::Value> {
        if self.parameters.is_null() { None } else { Some(&self.parameters) }
    }
}
