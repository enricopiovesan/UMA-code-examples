
use anyhow::Result;

fn main() -> Result<()> {
    let contract = contract::Contract::load_from("../../CONTRACT.json")?;
    let svc = format!("{}:{}", contract.service.name, contract.service.version);

    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "../../sample-data/sample.pgm".to_string());

    core_service::analyze_image(&path, &svc, &contract)?;
    Ok(())
}
