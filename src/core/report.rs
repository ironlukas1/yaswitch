use serde::Serialize;

use crate::core::planner::{ActionStatus, PreflightPlan, RiskLevel};
use crate::core::result::{ReasonCode, YaswitchError};

#[derive(Debug, Clone, Serialize)]
pub struct ReportSummary {
    pub planned_actions: usize,
    pub high_risk_actions: usize,
    pub skipped_actions: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyReport {
    pub status: String,
    pub reason_codes: Vec<String>,
    pub actions: Vec<super::planner::PlanAction>,
    pub summary: ReportSummary,
}

pub fn build_report_from_plan(plan: &PreflightPlan, status: &str) -> ApplyReport {
    let mut reason_codes = vec![plan.reason_code.clone()];
    for action in &plan.actions {
        if let Some(reason_code) = &action.reason_code {
            if !reason_codes.contains(reason_code) {
                reason_codes.push(reason_code.clone());
            }
        }
    }

    let high_risk_actions = plan
        .actions
        .iter()
        .filter(|action| action.risk_level == RiskLevel::High)
        .count();

    let skipped_actions = plan
        .actions
        .iter()
        .filter(|action| action.action_status == ActionStatus::Skipped)
        .count();

    ApplyReport {
        status: status.to_string(),
        reason_codes,
        actions: plan.actions.clone(),
        summary: ReportSummary {
            planned_actions: plan.actions.len(),
            high_risk_actions,
            skipped_actions,
        },
    }
}

pub fn report_to_json(report: &ApplyReport) -> Result<String, YaswitchError> {
    serde_json::to_string_pretty(report).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ReportSerializationFailed,
            format!("failed to serialize apply report: {error}"),
        )
    })
}
