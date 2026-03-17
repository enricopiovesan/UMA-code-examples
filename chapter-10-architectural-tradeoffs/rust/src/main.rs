use chapter10_architectural_tradeoffs::{
    diff_reports, format_diff, format_report, list_labs, load_report, project_root, validate_all,
};

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  cargo run -- list");
    eprintln!("  cargo run -- render <lab> [text|json]");
    eprintln!("  cargo run -- validate [lab]");
    eprintln!("  cargo run -- diff <from-lab> <to-lab>");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
    };
    let root = project_root();

    match command.as_str() {
        "list" => {
            for lab in list_labs(&root).unwrap() {
                println!("{lab}");
            }
        }
        "render" => {
            let Some(lab) = args.next() else {
                usage();
            };
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
                    "Validated {}: {} services, {} interactions, verdict={}",
                    report.scenario,
                    report.services.len(),
                    report.interactions.len(),
                    report.assessment.verdict
                );
            } else {
                for summary in validate_all(&root).unwrap() {
                    println!("{summary}");
                }
            }
        }
        "diff" => {
            let Some(from_lab) = args.next() else {
                usage();
            };
            let Some(to_lab) = args.next() else {
                usage();
            };
            let from = load_report(&root, &from_lab).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            let to = load_report(&root, &to_lab).unwrap_or_else(|err| {
                eprintln!("{err}");
                std::process::exit(1);
            });
            println!("{}", format_diff(&diff_reports(&from, &to)));
        }
        _ => usage(),
    }
}
