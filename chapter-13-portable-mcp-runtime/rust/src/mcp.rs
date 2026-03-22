use crate::{
    available_capabilities_for_scenario, format_report, list_scenarios, load_scenario, project_root,
    run_scenario, CapabilityContract, Scenario,
};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;

const SERVER_NAME: &str = "chapter13-portable-mcp-runtime";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

fn tool_definitions() -> Value {
    json!([
        {
            "name": "list_scenarios",
            "description": "List the Chapter 13 reference scenarios available in the portable MCP runtime lab.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "describe_scenario",
            "description": "Return the goal, constraints, and summary for one Chapter 13 scenario.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scenario": { "type": "string" }
                },
                "required": ["scenario"],
                "additionalProperties": false
            }
        },
        {
            "name": "list_capabilities",
            "description": "List the capability contracts available to a specific Chapter 13 scenario.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scenario": { "type": "string" }
                },
                "required": ["scenario"],
                "additionalProperties": false
            }
        },
        {
            "name": "run_scenario",
            "description": "Execute a Chapter 13 scenario and return the structured execution report.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scenario": { "type": "string" }
                },
                "required": ["scenario"],
                "additionalProperties": false
            }
        },
        {
            "name": "validate_scenario",
            "description": "Validate a scenario and return a compact execution summary.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scenario": { "type": "string" }
                },
                "required": ["scenario"],
                "additionalProperties": false
            }
        },
        {
            "name": "render_report",
            "description": "Render a scenario report as text or JSON.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "scenario": { "type": "string" },
                    "format": { "type": "string", "enum": ["text", "json"] }
                },
                "required": ["scenario"],
                "additionalProperties": false
            }
        }
    ])
}

fn extract_id(request: &Value) -> Value {
    request.get("id").cloned().unwrap_or(Value::Null)
}

fn success_response(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

fn error_response(id: Value, code: i64, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    })
}

fn text_tool_result(text: String) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": text
            }
        ],
        "isError": false
    })
}

fn structured_tool_result(text: String, structured: Value) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": text
            }
        ],
        "structuredContent": structured,
        "isError": false
    })
}

fn scenario_arg(params: &Value) -> Result<String, String> {
    params
        .get("arguments")
        .and_then(|args| args.get("scenario"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| "missing required argument: scenario".to_string())
}

fn format_arg(params: &Value) -> String {
    params
        .get("arguments")
        .and_then(|args| args.get("format"))
        .and_then(Value::as_str)
        .unwrap_or("text")
        .to_string()
}

fn scenario_list_payload(root: &Path) -> Result<Vec<Value>, String> {
    list_scenarios(root).map(|items| {
        items.into_iter()
            .map(|scenario| {
                json!({
                    "id": scenario.id,
                    "title": scenario.title,
                    "summary": scenario.summary,
                    "goal": scenario.goal,
                    "context": scenario.context
                })
            })
            .collect()
    })
}

fn capabilities_payload(items: Vec<CapabilityContract>) -> Vec<Value> {
    items.into_iter()
        .map(|contract| {
            json!({
                "name": contract.name,
                "version": contract.version,
                "intent": contract.intent,
                "inputs": contract.inputs,
                "outputs": contract.outputs,
                "constraints": contract.constraints,
                "emitsEvents": contract.emits_events,
                "metadata": contract.metadata
            })
        })
        .collect()
}

fn describe_scenario_payload(scenario: Scenario) -> Value {
    json!({
        "id": scenario.id,
        "title": scenario.title,
        "summary": scenario.summary,
        "goal": scenario.goal,
        "context": scenario.context
    })
}

fn call_tool(root: &Path, params: &Value) -> Result<Value, String> {
    let tool_name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing tool name".to_string())?;

    match tool_name {
        "list_scenarios" => {
            let scenarios = scenario_list_payload(root)?;
            let text = scenarios
                .iter()
                .filter_map(|item| {
                    Some(format!(
                        "{} - {}",
                        item.get("id")?.as_str()?,
                        item.get("title")?.as_str()?
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(structured_tool_result(text, json!({ "scenarios": scenarios })))
        }
        "describe_scenario" => {
            let id = scenario_arg(params)?;
            let scenario = load_scenario(root, &id)?;
            Ok(structured_tool_result(
                format!("{} - {}", scenario.id, scenario.title),
                describe_scenario_payload(scenario),
            ))
        }
        "list_capabilities" => {
            let id = scenario_arg(params)?;
            let capabilities = available_capabilities_for_scenario(root, &id)?;
            let text = capabilities
                .iter()
                .map(|contract| format!("{} - {}", contract.name, contract.metadata.description))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(structured_tool_result(
                text,
                json!({ "scenario": id, "capabilities": capabilities_payload(capabilities) }),
            ))
        }
        "run_scenario" => {
            let id = scenario_arg(params)?;
            let report = run_scenario(root, &id)?;
            Ok(structured_tool_result(
                format!(
                    "{} completed with {} step(s), status={}",
                    report.scenario,
                    report.steps.len(),
                    report.status
                ),
                serde_json::to_value(report).map_err(|err| err.to_string())?,
            ))
        }
        "validate_scenario" => {
            let id = scenario_arg(params)?;
            let report = run_scenario(root, &id)?;
            Ok(structured_tool_result(
                format!(
                    "Validated {}: {} step(s), status={}",
                    report.scenario,
                    report.steps.len(),
                    report.status
                ),
                json!({
                    "scenario": report.scenario,
                    "status": report.status,
                    "steps": report.steps.len(),
                    "selectedPath": report.selected_path,
                    "rejectedCapabilities": report.rejected_capabilities
                }),
            ))
        }
        "render_report" => {
            let id = scenario_arg(params)?;
            let format = format_arg(params);
            let report = run_scenario(root, &id)?;
            if format == "json" {
                Ok(structured_tool_result(
                    format!("Rendered {} as JSON", report.scenario),
                    serde_json::to_value(report).map_err(|err| err.to_string())?,
                ))
            } else {
                Ok(text_tool_result(format_report(&report)))
            }
        }
        _ => Err(format!("unknown tool: {tool_name}")),
    }
}

pub fn handle_request(root: &Path, request: &Value) -> Result<Option<Value>, String> {
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing method".to_string())?;
    let id = extract_id(request);

    let response = match method {
        "initialize" => success_response(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": SERVER_NAME,
                    "version": SERVER_VERSION
                },
                "capabilities": {
                    "tools": {}
                }
            }),
        ),
        "notifications/initialized" => return Ok(None),
        "ping" => success_response(id, json!({})),
        "tools/list" => success_response(id, json!({ "tools": tool_definitions() })),
        "tools/call" => match call_tool(root, request.get("params").unwrap_or(&Value::Null)) {
            Ok(result) => success_response(id, result),
            Err(message) => error_response(id, -32001, &message),
        },
        _ => error_response(id, -32601, "method not found"),
    };

    Ok(Some(response))
}

fn read_message<R: BufRead>(reader: &mut R) -> Result<Option<String>, String> {
    let mut content_length: Option<usize> = None;
    let mut saw_header = false;

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).map_err(|err| err.to_string())?;
        if bytes == 0 {
            if saw_header {
                return Err("unexpected EOF while reading MCP headers".to_string());
            }
            return Ok(None);
        }

        if line == "\r\n" || line == "\n" {
            break;
        }

        saw_header = true;
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if let Some(value) = trimmed.strip_prefix("Content-Length:") {
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|err| format!("invalid Content-Length: {err}"))?,
            );
        }
    }

    let length = content_length.ok_or_else(|| "missing Content-Length header".to_string())?;
    let mut payload = vec![0u8; length];
    reader.read_exact(&mut payload).map_err(|err| err.to_string())?;
    String::from_utf8(payload).map_err(|err| err.to_string())
        .map(Some)
}

fn write_message<W: Write>(writer: &mut W, value: &Value) -> Result<(), String> {
    let body = serde_json::to_vec(value).map_err(|err| err.to_string())?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len()).map_err(|err| err.to_string())?;
    writer.write_all(&body).map_err(|err| err.to_string())?;
    writer.flush().map_err(|err| err.to_string())
}

pub fn serve_stdio() -> Result<(), String> {
    let root = project_root();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut writer = stdout.lock();

    while let Some(payload) = read_message(&mut reader)? {
        let request: Value = serde_json::from_str(&payload).map_err(|err| err.to_string())?;
        if let Some(response) = handle_request(&root, &request)? {
            write_message(&mut writer, &response)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_root;

    #[test]
    fn initialize_advertises_tools_capability() {
        let response = handle_request(
            &project_root(),
            &json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {}
            }),
        )
        .unwrap()
        .unwrap();

        assert_eq!(response["result"]["serverInfo"]["name"], SERVER_NAME);
        assert!(response["result"]["capabilities"]["tools"].is_object());
    }

    #[test]
    fn tools_list_returns_runtime_tools() {
        let response = handle_request(
            &project_root(),
            &json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/list",
                "params": {}
            }),
        )
        .unwrap()
        .unwrap();

        let tools = response["result"]["tools"].as_array().unwrap();
        assert!(tools.iter().any(|tool| tool["name"] == "run_scenario"));
        assert!(tools.iter().any(|tool| tool["name"] == "list_capabilities"));
    }

    #[test]
    fn tool_call_can_run_a_scenario() {
        let response = handle_request(
            &project_root(),
            &json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {
                    "name": "run_scenario",
                    "arguments": {
                        "scenario": "use-case-3-french-report"
                    }
                }
            }),
        )
        .unwrap()
        .unwrap();

        assert_eq!(response["result"]["structuredContent"]["scenario"], "use-case-3-french-report");
        assert_eq!(response["result"]["structuredContent"]["final_language"], "fr");
    }

    #[test]
    fn can_round_trip_framed_message() {
        let mut buffer = Vec::new();
        write_message(&mut buffer, &json!({"jsonrpc": "2.0", "id": 1, "result": {"ok": true}})).unwrap();
        let mut reader = io::BufReader::new(buffer.as_slice());
        let payload = read_message(&mut reader).unwrap().unwrap();
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(value["result"]["ok"], true);
    }
}
