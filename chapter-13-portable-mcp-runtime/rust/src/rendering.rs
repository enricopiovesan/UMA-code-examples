use crate::{list_scenarios, project_root, run_scenario, ExecutionReport};
use std::fmt::Write as _;
use wasm_bindgen::prelude::*;

pub fn format_report(report: &ExecutionReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}\n", report.title);
    let _ = writeln!(out, "Scenario: {}", report.scenario);
    let _ = writeln!(out, "Summary: {}", report.summary);
    let _ = writeln!(out, "Status: {}", report.status);
    let _ = writeln!(out, "Target Language: {}\n", report.goal.target_language);

    let _ = writeln!(out, "Goal:");
    let _ = writeln!(out, "- target: {}", report.goal.target);
    let _ = writeln!(out, "- prefer AI: {}", report.goal.prefer_ai);
    let _ = writeln!(out, "- local only: {}", report.goal.local_only);
    let _ = writeln!(out, "- allow degraded: {}\n", report.goal.allow_degraded);

    let _ = writeln!(out, "Initial Context:");
    let _ = writeln!(out, "- project: {}", report.initial_context.project_name);
    let _ = writeln!(out, "- source fragments: {}", report.initial_context.source_fragments.len());
    let _ = writeln!(out, "- available capabilities: {}", report.initial_context.available_capabilities.join(", "));
    let _ = writeln!(out, "- AI available: {}", report.initial_context.ai_available);
    let _ = writeln!(out, "- planner provider: {}", report.planner_provider);
    let _ = writeln!(out, "- planner mode: {}", report.planner_mode);
    let _ = writeln!(
        out,
        "- planner fallback reason: {}\n",
        report
            .planner_fallback_reason
            .as_deref()
            .unwrap_or("none")
    );

    let _ = writeln!(out, "SummarizerAI:");
    let _ = writeln!(out, "- provider: {}", report.summarizer_ai_provider);
    let _ = writeln!(out, "- mode: {}", report.summarizer_ai_mode);
    let _ = writeln!(
        out,
        "- fallback reason: {}\n",
        report
            .summarizer_ai_fallback_reason
            .as_deref()
            .unwrap_or("none")
    );

    let _ = writeln!(out, "TranslatorFr:");
    let _ = writeln!(out, "- provider: {}", report.translator_ai_provider);
    let _ = writeln!(out, "- mode: {}", report.translator_ai_mode);
    let _ = writeln!(
        out,
        "- fallback reason: {}\n",
        report
            .translator_ai_fallback_reason
            .as_deref()
            .unwrap_or("none")
    );

    let _ = writeln!(out, "Selected Path:");
    for item in &report.selected_path {
        let _ = writeln!(out, "- {}", item);
    }
    let _ = writeln!(out);

    if report.rejected_capabilities.is_empty() {
        let _ = writeln!(out, "Rejected Capabilities: none\n");
    } else {
        let _ = writeln!(out, "Rejected Capabilities:");
        for rejection in &report.rejected_capabilities {
            let _ = writeln!(
                out,
                "- {}: {}",
                rejection.capability,
                rejection.reasons.join(" | ")
            );
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "Execution Timeline:");
    for step in &report.steps {
        let _ = writeln!(out, "Step {}: {}", step.index, step.selected_capability);
        let _ = writeln!(out, "  unmet need: {}", step.need);
        let _ = writeln!(out, "  agent provider: {}", step.agent_provider);
        let _ = writeln!(out, "  agent mode: {}", step.agent_mode);
        if let Some(reason) = &step.agent_fallback_reason {
            let _ = writeln!(out, "  agent fallback note: {}", reason);
        }
        let _ = writeln!(out, "  execution provider: {}", step.execution_provider);
        let _ = writeln!(out, "  execution mode: {}", step.execution_mode);
        if let Some(reason) = &step.fallback_reason {
            let _ = writeln!(out, "  fallback note: {}", reason);
        }
        let _ = writeln!(
            out,
            "  agent proposal: {}",
            step.agent_proposal.as_deref().unwrap_or("none")
        );
        if let Some(rejected) = &step.proposed_validation {
            let _ = writeln!(
                out,
                "  proposal rejected: {} ({})",
                rejected.capability,
                rejected.reasons.join(" | ")
            );
        }
        let _ = writeln!(out, "  discovery candidates:");
        for item in &step.discovery.available {
            let _ = writeln!(out, "  - {}", item.capability);
        }
        let visible_rejections = step
            .discovery
            .rejected
            .iter()
            .filter(|item| {
                item.status != "hidden"
                    && item.reason.as_deref() != Some("does not contribute to the current unmet need")
            })
            .collect::<Vec<_>>();
        if !visible_rejections.is_empty() {
            let _ = writeln!(out, "  rejected during discovery:");
            for item in visible_rejections {
                let _ = writeln!(
                    out,
                    "  - {}: {}",
                    item.capability,
                    item.reason.as_deref().unwrap_or("none")
                );
            }
        }
        let _ = writeln!(
            out,
            "  validation: {} ({})",
            step.validation.capability,
            step.validation.status
        );
        if !step.validation.reasons.is_empty() {
            let _ = writeln!(out, "  validation reasons: {}", step.validation.reasons.join(" | "));
        }
        let _ = writeln!(out, "  output: {}", step.output_preview);
        let _ = writeln!(out, "  events:");
        let visible_events = step
            .events
            .iter()
            .filter(|event| {
                !(event.event_type == "CapabilityRejected"
                    && matches!(
                        event.reason.as_deref(),
                        Some("does not contribute to the current unmet need")
                            | Some("capability not available in this scenario")
                    ))
            })
            .collect::<Vec<_>>();
        for event in visible_events {
            let _ = writeln!(
                out,
                "  - {} {} ({})",
                event.event_type, event.capability, event.status
            );
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "Final Output:");
    let _ = writeln!(out, "{}\n", report.final_output);

    let _ = writeln!(out, "Graph Nodes:");
    for node in &report.graph_nodes {
        let _ = writeln!(out, "- {} [{}] {}", node.label, node.kind, node.state);
    }
    let _ = writeln!(out);

    out
}

#[wasm_bindgen]
pub fn list_scenarios_json() -> String {
    serde_json::to_string_pretty(&list_scenarios(&project_root()).expect("list scenarios")).unwrap()
}

#[wasm_bindgen]
pub fn run_scenario_json(id: &str) -> String {
    serde_json::to_string_pretty(&run_scenario(&project_root(), id).expect("run scenario")).unwrap()
}
