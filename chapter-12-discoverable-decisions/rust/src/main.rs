use chapter12_discoverable_decisions::{
    diff_reports, format_diff, format_report, list_labs, load_report, project_root, validate_all,
};

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  cargo run --manifest-path rust/Cargo.toml -- list");
    eprintln!("  cargo run --manifest-path rust/Cargo.toml -- render <lab> [text|json]");
    eprintln!("  cargo run --manifest-path rust/Cargo.toml -- validate [lab]");
    eprintln!("  cargo run --manifest-path rust/Cargo.toml -- diff <from-lab> <to-lab>");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| usage());
    let root = project_root();

    match command.as_str() {
        "list" => {
            for lab in list_labs(&root).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            }) {
                println!("{lab}");
            }
        }
        "render" => {
            let lab = args.next().unwrap_or_else(|| usage());
            let format = args.next().unwrap_or_else(|| "text".to_string());
            let report = load_report(&root, &lab).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!("{}", format_report(&report));
            }
        }
        "validate" => {
            if let Some(lab) = args.next() {
                let report = load_report(&root, &lab).unwrap_or_else(|err| {
                    eprintln!("{err}");
                    std::process::exit(1);
                });
                println!(
                    "Validated {}: {} surfaces, verdict={}",
                    report.scenario,
                    report.surfaces.len(),
                    report.assessment.verdict
                );
            } else {
                for summary in validate_all(&root).unwrap_or_else(|err| {
                    eprintln!("{err}");
                    std::process::exit(1);
                }) {
                    println!("{summary}");
                }
            }
        }
        "diff" => {
            let from = args.next().unwrap_or_else(|| usage());
            let to = args.next().unwrap_or_else(|| usage());
            let from_report = load_report(&root, &from).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            let to_report = load_report(&root, &to).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            println!("{}", format_diff(&diff_reports(&from_report, &to_report)));
        }
        _ => usage(),
    }
}
