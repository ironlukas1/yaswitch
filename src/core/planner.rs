use serde::Serialize;
use std::path::Path;

use crate::core::result::{ReasonCode, YaswitchError};
use crate::core::theme_spec::{load_theme_spec_from_dir, TargetMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Planned,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    RenderTemplate,
    ApplyTarget,
    Reload,
    SafeSkip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Planned,
    Applied,
    Skipped,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlanAction {
    pub id: String,
    pub target: String,
    pub action_type: ActionType,
    pub risk_level: RiskLevel,
    pub expected_outcome: String,
    pub reason_code: Option<String>,
    pub action_status: ActionStatus,
    pub remediation_note: Option<String>,
    pub remediation_command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PreflightPlan {
    pub status: PlanStatus,
    pub dry_run: bool,
    pub reason_code: String,
    pub actions: Vec<PlanAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerOptions {
    pub theme_dir: String,
    pub compositor: Option<String>,
    pub target_filter: Option<String>,
    pub dry_run: bool,
}

pub fn build_preflight_plan(options: &PlannerOptions) -> Result<PreflightPlan, YaswitchError> {
    let spec = load_theme_spec_from_dir(Path::new(&options.theme_dir))?;

    let mut actions = Vec::new();
    for (target_id, target) in &spec.targets {
        if let Some(filter) = &options.target_filter {
            if filter != target_id {
                continue;
            }
        }

        let is_vscode_target = matches!(
            target_id.to_ascii_lowercase().as_str(),
            "vscode" | "vscode-insiders"
        );

        actions.push(PlanAction {
            id: format!("render-{target_id}"),
            target: target_id.clone(),
            action_type: ActionType::RenderTemplate,
            risk_level: RiskLevel::Low,
            expected_outcome: format!("render template {}", target.template),
            reason_code: None,
            action_status: ActionStatus::Planned,
            remediation_note: if is_vscode_target {
                Some("VSCode targets may require an application restart to fully apply all theme changes.".to_string())
            } else {
                None
            },
            remediation_command: if is_vscode_target {
                Some("Restart VSCode (or VSCode Insiders) after apply if colors did not refresh.".to_string())
            } else {
                None
            },
        });

        let apply_risk = match target.mode {
            TargetMode::Inject => RiskLevel::Medium,
            TargetMode::Overwrite => RiskLevel::High,
        };

        actions.push(PlanAction {
            id: format!("apply-{target_id}"),
            target: target_id.clone(),
            action_type: ActionType::ApplyTarget,
            risk_level: apply_risk,
            expected_outcome: format!("update {}", target.destination),
            reason_code: None,
            action_status: ActionStatus::Planned,
            remediation_note: if is_vscode_target {
                Some("VSCode targets can require a process restart to guarantee full theme reload.".to_string())
            } else {
                None
            },
            remediation_command: if is_vscode_target {
                Some("Run `yaswitch apply --theme <theme-path> --target vscode --dry-run --json` to confirm action details before restart.".to_string())
            } else {
                None
            },
        });
    }

    if options.target_filter.is_some()
        && !actions
            .iter()
            .any(|a| a.action_type == ActionType::ApplyTarget)
    {
        return Err(YaswitchError::new(
            ReasonCode::PlannerInvalidTarget,
            "requested --target did not match any theme target",
        ));
    }

    let compositor = options
        .compositor
        .as_deref()
        .unwrap_or("sway")
        .to_ascii_lowercase();

    if !matches!(
        compositor.as_str(),
        "sway" | "hyprland" | "niri" | "dwl" | "mangowm"
    ) {
        return Err(YaswitchError::new(
            ReasonCode::PlannerInvalidTarget,
            format!("unsupported compositor '{compositor}'"),
        ));
    }

    if matches!(compositor.as_str(), "dwl" | "mangowm") {
        actions.push(PlanAction {
            id: format!("reload-{compositor}"),
            target: compositor.clone(),
            action_type: ActionType::SafeSkip,
            risk_level: RiskLevel::Low,
            expected_outcome: "skip unsupported reload while preserving apply success".to_string(),
            reason_code: Some(ReasonCode::SkipReloadUnsupported.as_str().to_string()),
            action_status: ActionStatus::Skipped,
            remediation_note: Some(
                "Reload is unsupported for this compositor; switch manually or use a Tier-1 compositor."
                    .to_string(),
            ),
            remediation_command: Some(
                "yaswitch apply --theme <theme-path> --compositor sway --json".to_string(),
            ),
        });
    } else {
        actions.push(PlanAction {
            id: format!("reload-{compositor}"),
            target: compositor,
            action_type: ActionType::Reload,
            risk_level: RiskLevel::Medium,
            expected_outcome: "reload compositor-aware components".to_string(),
            reason_code: None,
            action_status: ActionStatus::Planned,
            remediation_note: None,
            remediation_command: None,
        });
    }

    if actions.is_empty() {
        return Err(YaswitchError::new(
            ReasonCode::PlannerNoActions,
            "preflight generated no actions",
        ));
    }

    Ok(PreflightPlan {
        status: PlanStatus::Planned,
        dry_run: options.dry_run,
        reason_code: if options.dry_run {
            ReasonCode::PlannerDryRun.as_str().to_string()
        } else {
            ReasonCode::PlannerReady.as_str().to_string()
        },
        actions,
    })
}
