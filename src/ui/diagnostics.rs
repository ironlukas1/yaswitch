use crate::core::report::ApplyReport;

#[must_use]
pub fn render_diagnostics_text(report: &ApplyReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("status: {}", report.status));
    lines.push(format!(
        "planned_actions: {}",
        report.summary.planned_actions
    ));

    if !report.reason_codes.is_empty() {
        lines.push(format!("reason_codes: {}", report.reason_codes.join(",")));
    }

    for action in &report.actions {
        let reason = action
            .reason_code
            .clone()
            .unwrap_or_else(|| "none".to_string());
        let remediation_note = action
            .remediation_note
            .clone()
            .unwrap_or_else(|| "none".to_string());
        let remediation_command = action
            .remediation_command
            .clone()
            .unwrap_or_else(|| "none".to_string());

        lines.push(format!(
            "action={} target={} status={:?} outcome={} reason={} remediation_note={} remediation_command={}",
            action.id,
            action.target,
            action.action_status,
            action.expected_outcome,
            reason,
            remediation_note,
            remediation_command
        ));
    }

    lines.join("\n")
}
