
use anyhow::Result;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct Telemetry {
    adapter: String,
    backend: String,
    timestamp: String,
}

#[cfg(feature = "gpu")]
async fn gpu_info() -> Result<Option<Telemetry>> {
    let instance = wgpu::Instance::default();
    if let Some(adapter) = instance.enumerate_adapters(wgpu::Backends::all()).next() {
        let info = adapter.get_info();
        Ok(Some(Telemetry {
            adapter: info.name,
            backend: format!("{:?}", info.backend),
            timestamp: Utc::now().to_rfc3339(),
        }))
    } else {
        Ok(None)
    }
}

#[cfg(not(feature = "gpu"))]
async fn gpu_info() -> Result<Option<Telemetry>> { Ok(None) }

fn main() -> Result<()> {
    let contract = contract::Contract::load_from("../../CONTRACT.json")?;
    let svc = format!("{}:{}", contract.service.name, contract.service.version);

    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "../../sample-data/sample.pgm".to_string());

    core_service::analyze_image(&path, &svc, &contract)?;

    // Enforce capability gate by contract scope
    let allow_gpu = contract.execution.constraints["native-gpu"]["compatibility"] == "target-specific";

    let telemetry = if allow_gpu { pollster::block_on(gpu_info())? } else { None };
    if let Some(t) = telemetry {
        bus::publish_validated(&contract, "gpu.telemetry.reported", &t)?;
    } else {
        #[derive(Serialize)]
        struct TelemetryErr { timestamp: String, reason: String }
        let err = TelemetryErr { timestamp: Utc::now().to_rfc3339(), reason: "gpu feature not enabled or adapter not found".into() };
        bus::publish_validated(&contract, "gpu.telemetry.reported", &err)?;
    }

    Ok(())
}
