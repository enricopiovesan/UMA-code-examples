use crate::{CapabilityContract, ExecutionReport, Scenario};
use std::fs;
use std::path::{Path, PathBuf};

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("chapter root")
        .to_path_buf()
}

pub(crate) fn contract_fixtures() -> Vec<CapabilityContract> {
    let raws = [
        include_str!("../../contracts/data-provider-local.json"),
        include_str!("../../contracts/insight-enricher.json"),
        include_str!("../../contracts/planner-ai.json"),
        include_str!("../../contracts/summarizer-basic.json"),
        include_str!("../../contracts/summarizer-ai.json"),
        include_str!("../../contracts/translator-fr.json"),
        include_str!("../../contracts/formatter.json"),
    ];
    let mut contracts = Vec::with_capacity(raws.len());
    for raw in raws {
        contracts.push(serde_json::from_str::<CapabilityContract>(raw).expect("valid contract fixture"));
    }
    contracts
}

pub fn capability_descriptors() -> Vec<CapabilityContract> {
    contract_fixtures()
}

pub fn available_capabilities_for_scenario(root: &Path, id: &str) -> Result<Vec<CapabilityContract>, String> {
    let scenario = load_scenario(root, id)?;
    let mut available = Vec::new();
    for contract in contract_fixtures() {
        if scenario
            .context
            .available_capabilities
            .iter()
            .any(|item| item == &contract.name)
        {
            available.push(contract);
        }
    }
    Ok(available)
}

pub fn run_scenario(root: &Path, id: &str) -> Result<ExecutionReport, String> {
    let scenario = load_scenario(root, id)?;
    crate::run_loaded_scenario(root, scenario)
}

pub fn list_scenarios(root: &Path) -> Result<Vec<Scenario>, String> {
    let mut scenarios = Vec::new();
    for entry in fs::read_dir(root.join("examples")).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        if !entry.file_type().map_err(|err| err.to_string())?.is_dir() {
            continue;
        }
        let path = entry.path().join("scenario.json");
        if path.exists() {
            let raw = fs::read_to_string(&path).map_err(|err| err.to_string())?;
            let scenario = serde_json::from_str::<Scenario>(&raw).map_err(|err| err.to_string())?;
            scenarios.push(scenario);
        }
    }
    scenarios.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(scenarios)
}

pub fn load_scenario(root: &Path, id: &str) -> Result<Scenario, String> {
    for scenario in list_scenarios(root)? {
        if scenario.id == id {
            return Ok(scenario);
        }
    }
    Err(format!("unknown scenario \"{id}\""))
}
