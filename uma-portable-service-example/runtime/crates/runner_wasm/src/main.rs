use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let contract_path = if PathBuf::from("../CONTRACT.json").exists() {
        PathBuf::from("../CONTRACT.json")
    } else {
        repo_root.join("CONTRACT.json")
    };
    let contract = contract::Contract::load_from(contract_path.to_str().unwrap())?;
    let svc = format!("{}:{}", contract.service.name, contract.service.version);

    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| {
        let relative = PathBuf::from("../sample-data/sample.pgm");
        if relative.exists() {
            relative.to_string_lossy().into_owned()
        } else {
            repo_root
                .join("sample-data/sample.pgm")
                .to_string_lossy()
                .into_owned()
        }
    });

    core_service::analyze_image(&path, &svc, &contract)?;
    Ok(())
}
