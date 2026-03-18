use chapter9_trust_boundaries::{
    diff_reports, evaluate_trust, format_report, format_trust_diff, list_scenarios, load_scenario,
    project_root,
};

fn print_usage() {
    println!("Usage:");
    println!("  cargo run --quiet -- list");
    println!("  cargo run --quiet -- render <scenario> [text|json]");
    println!("  cargo run --quiet -- validate [scenario ...]");
    println!("  cargo run --quiet -- trust-diff <from-scenario> <to-scenario>");
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = project_root();

    match args.first().map(String::as_str) {
        Some("--help") | Some("-h") | None => {
            print_usage();
            Ok(())
        }
        Some("list") => {
            for scenario in list_scenarios(&root)? {
                println!("{scenario}");
            }
            Ok(())
        }
        Some("render") => {
            let scenario = args.get(1).map(String::as_str).unwrap_or("lab1-trusted-service");
            let format = args.get(2).map(String::as_str).unwrap_or("text");
            let report = evaluate_trust(&load_scenario(&root, scenario)?);
            if format == "json" {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
                );
            } else {
                print!("{}", format_report(&report));
            }
            Ok(())
        }
        Some("validate") => {
            let scenarios = if args.len() > 1 {
                args[1..].to_vec()
            } else {
                list_scenarios(&root)?
            };
            for scenario in scenarios {
                let loaded = load_scenario(&root, &scenario)?;
                println!(
                    "Validated {}: {} services, {} executions, {} communications",
                    loaded.name,
                    loaded.services.len(),
                    loaded.executions.len(),
                    loaded.communications.len()
                );
            }
            Ok(())
        }
        Some("trust-diff") => {
            let from = args
                .get(1)
                .ok_or_else(|| "Usage: cargo run --quiet -- trust-diff <from-scenario> <to-scenario>".to_string())?;
            let to = args
                .get(2)
                .ok_or_else(|| "Usage: cargo run --quiet -- trust-diff <from-scenario> <to-scenario>".to_string())?;
            let left = evaluate_trust(&load_scenario(&root, from)?);
            let right = evaluate_trust(&load_scenario(&root, to)?);
            print!("{}", format_trust_diff(&diff_reports(&left, &right)));
            Ok(())
        }
        Some(other) => Err(format!("unknown command \"{other}\"")),
    }
}
