use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::adapters::contract::{AdapterOutcome, ThemeAdapter};
use crate::core::paths::ensure_path_within_allowed_roots;
use crate::core::planner::{ActionType, PlannerOptions, PreflightPlan};
use crate::core::report::{build_report_from_plan, ApplyReport};
use crate::core::result::{ReasonCode, YaswitchError};
use crate::core::template_engine::{inject_managed_block, render_template};
use crate::core::theme_spec::{load_theme_spec_from_dir, TargetMode};
use crate::core::transaction::Transaction;

pub fn execute_plan(
    plan: &PreflightPlan,
    options: &PlannerOptions,
    adapter: &dyn ThemeAdapter,
    state_root: &Path,
    allowed_roots: &[PathBuf],
) -> Result<ApplyReport, YaswitchError> {
    if options.dry_run {
        return Ok(build_report_from_plan(plan, "planned"));
    }

    let spec = load_theme_spec_from_dir(&options.theme_dir)?;
    let theme_root = PathBuf::from(&options.theme_dir);

    let mut tx = Transaction::begin(state_root)?;

    for action in &plan.actions {
        if action.action_type != ActionType::ApplyTarget {
            continue;
        }

        let target = spec.targets.get(&action.target).ok_or_else(|| {
            YaswitchError::new(
                ReasonCode::PlannerInvalidTarget,
                format!("target '{}' not found in theme spec", action.target),
            )
        })?;

        let template_path = theme_root.join(&target.template);
        let destination_path = theme_root.join(&target.destination);
        ensure_path_within_allowed_roots(&destination_path, allowed_roots)?;

        let template_content = fs::read_to_string(&template_path).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed reading template {}: {error}",
                    template_path.display()
                ),
            )
        })?;

        let render_context = palette_context(&spec.palette);
        let rendered = render_template(&template_content, &render_context)?;

        let next_content = match target.mode {
            TargetMode::Inject => {
                let existing_content = fs::read_to_string(&destination_path).unwrap_or_default();
                inject_managed_block(&existing_content, &rendered, true)?
            }
            TargetMode::Overwrite => rendered,
        };

        tx.write_file_atomic(&destination_path, &next_content)?;
    }

    let apply_outcome = match adapter.apply() {
        Ok(outcome) => outcome,
        Err(error) => {
            tx.rollback()?;
            return Err(error);
        }
    };

    match apply_outcome {
        AdapterOutcome::Applied => {
            if let Err(error) = adapter.verify() {
                tx.rollback()?;
                return Err(error);
            }
            tx.commit()?;
        }
        AdapterOutcome::Skipped { reason } => {
            if !matches!(
                reason,
                ReasonCode::SkipReloadUnsupported
                    | ReasonCode::CompositorSocketUnavailable
                    | ReasonCode::SkipRestartPolicy
            ) {
                tx.rollback()?;
                return Err(YaswitchError::new(
                    ReasonCode::AdapterCapabilityMissing,
                    format!(
                        "adapter '{}' returned unsupported skip reason {}",
                        adapter.id(),
                        reason.as_str()
                    ),
                ));
            }
            tx.commit()?;
        }
    }

    Ok(build_report_from_plan(plan, "applied"))
}

fn palette_context(palette: &crate::core::theme_spec::Palette) -> HashMap<String, String> {
    HashMap::from([
        ("base00".to_string(), palette.base00.clone()),
        ("base01".to_string(), palette.base01.clone()),
        ("base02".to_string(), palette.base02.clone()),
        ("base03".to_string(), palette.base03.clone()),
        ("base04".to_string(), palette.base04.clone()),
        ("base05".to_string(), palette.base05.clone()),
        ("base06".to_string(), palette.base06.clone()),
        ("base07".to_string(), palette.base07.clone()),
        ("base08".to_string(), palette.base08.clone()),
        ("base09".to_string(), palette.base09.clone()),
        ("base0A".to_string(), palette.base0a.clone()),
        ("base0B".to_string(), palette.base0b.clone()),
        ("base0C".to_string(), palette.base0c.clone()),
        ("base0D".to_string(), palette.base0d.clone()),
        ("base0E".to_string(), palette.base0e.clone()),
        ("base0F".to_string(), palette.base0f.clone()),
    ])
}
