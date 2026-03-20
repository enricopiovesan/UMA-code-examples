use chapter12_discoverable_decisions::{
    diff_reports, format_diff, format_report, list_labs, load_report, project_root, validate_all,
};

fn usage_text() -> String {
    [
        "Usage:",
        "  cargo run --manifest-path rust/Cargo.toml -- list",
        "  cargo run --manifest-path rust/Cargo.toml -- render <lab> [text|json]",
        "  cargo run --manifest-path rust/Cargo.toml -- validate [lab]",
        "  cargo run --manifest-path rust/Cargo.toml -- diff <from-lab> <to-lab>",
    ]
    .join("\n")
}

fn render_list() -> Result<String, String> {
    let root = project_root();
    let labs = list_labs(&root)?;
    Ok(labs.join("\n"))
}

fn render_lab(lab: &str, format: &str) -> Result<String, String> {
    let root = project_root();
    let report = load_report(&root, lab)?;
    if format == "json" {
        serde_json::to_string_pretty(&report).map_err(|err| err.to_string())
    } else {
        Ok(format_report(&report))
    }
}

fn render_validation(lab: Option<&str>) -> Result<String, String> {
    let root = project_root();
    if let Some(lab) = lab {
        let report = load_report(&root, lab)?;
        Ok(format!(
            "Validated {}: {} surfaces, verdict={}",
            report.scenario,
            report.surfaces.len(),
            report.assessment.verdict
        ))
    } else {
        Ok(validate_all(&root)?.join("\n"))
    }
}

fn render_diff(from: &str, to: &str) -> Result<String, String> {
    let root = project_root();
    let from_report = load_report(&root, from)?;
    let to_report = load_report(&root, to)?;
    Ok(format_diff(&diff_reports(&from_report, &to_report)))
}

fn run<I>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let command = args.next().ok_or_else(usage_text)?;

    match command.as_str() {
        "list" => render_list(),
        "render" => {
            let lab = args.next().ok_or_else(usage_text)?;
            let format = args.next().unwrap_or_else(|| "text".to_string());
            render_lab(&lab, &format)
        }
        "validate" => render_validation(args.next().as_deref()),
        "diff" => {
            let from = args.next().ok_or_else(usage_text)?;
            let to = args.next().ok_or_else(usage_text)?;
            render_diff(&from, &to)
        }
        _ => Err(usage_text()),
    }
}

fn main() {
    match run(std::env::args().skip(1)) {
        Ok(output) => println!("{output}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_command_includes_known_lab() {
        let output = run(["list".to_string()]).unwrap();
        assert!(output.contains("lab1-capability-projection"));
        assert!(output.contains("lab6-queryable-trace"));
    }

    #[test]
    fn render_command_supports_json() {
        let output = run([
            "render".to_string(),
            "lab6-queryable-trace".to_string(),
            "json".to_string(),
        ])
        .unwrap();
        assert!(output.contains("\"scenario\": \"lab6-queryable-trace\""));
        assert!(output.contains("\"verdict\": \"governed\""));
    }

    #[test]
    fn diff_command_reports_verdict_transition() {
        let output = run([
            "diff".to_string(),
            "lab5-approved-execution".to_string(),
            "lab6-queryable-trace".to_string(),
        ])
        .unwrap();
        assert!(output.contains("Verdict: discoverable -> governed"));
        assert!(output.contains("Removed Warnings: partial_trace"));
    }

    #[test]
    fn unknown_command_returns_usage() {
        let err = run(["unknown".to_string()]).unwrap_err();
        assert!(err.contains("Usage:"));
        assert!(err.contains("render <lab> [text|json]"));
    }
}
