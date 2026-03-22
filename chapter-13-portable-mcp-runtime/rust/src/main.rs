use chapter13_portable_mcp_runtime::{format_report, list_scenarios, mcp, project_root, run_scenario};

fn usage_text() -> String {
    [
        "Usage:",
        "  cargo run --manifest-path rust/Cargo.toml -- list",
        "  cargo run --manifest-path rust/Cargo.toml -- render <scenario> [text|json]",
        "  cargo run --manifest-path rust/Cargo.toml -- validate [scenario]",
        "  cargo run --manifest-path rust/Cargo.toml -- mcp-serve",
    ]
    .join("\n")
}

fn run<I>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let command = args.next().ok_or_else(usage_text)?;
    let root = project_root();

    match command.as_str() {
        "list" => {
            let lines = list_scenarios(&root)?
                .into_iter()
                .map(|item| item.id)
                .collect::<Vec<_>>();
            Ok(lines.join("\n"))
        }
        "render" => {
            let id = args.next().ok_or_else(usage_text)?;
            let format = args.next().unwrap_or_else(|| "text".to_string());
            let report = run_scenario(&root, &id)?;
            if format == "json" {
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())
            } else {
                Ok(format_report(&report))
            }
        }
        "validate" => {
            if let Some(id) = args.next() {
                let report = run_scenario(&root, &id)?;
                Ok(format!(
                    "Validated {}: {} steps, status={}",
                    report.scenario,
                    report.steps.len(),
                    report.status
                ))
            } else {
                let summaries = list_scenarios(&root)?
                    .into_iter()
                    .map(|scenario| {
                        let report = run_scenario(&root, &scenario.id)?;
                        Ok(format!(
                            "Validated {}: {} steps, status={}",
                            report.scenario,
                            report.steps.len(),
                            report.status
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(summaries.join("\n"))
            }
        }
        "mcp-serve" => {
            mcp::serve_stdio()?;
            Ok(String::new())
        }
        _ => Err(usage_text()),
    }
}

fn main() {
    match run(std::env::args().skip(1)) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
        }
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
    fn list_command_includes_agent_validation() {
        let output = run(["list".to_string()]).unwrap();
        assert!(output.contains("use-case-5-agent-validation"));
    }

    #[test]
    fn render_json_includes_selected_path() {
        let output = run([
            "render".to_string(),
            "use-case-3-french-report".to_string(),
            "json".to_string(),
        ])
        .unwrap();
        assert!(output.contains("\"selected_path\""));
        assert!(output.contains("\"TranslatorFr\""));
    }

    #[test]
    fn usage_text_mentions_mcp_server() {
        assert!(usage_text().contains("mcp-serve"));
    }
}
