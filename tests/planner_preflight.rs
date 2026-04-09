use yaswitch::core::planner::{build_preflight_plan, ActionType, PlanStatus, PlannerOptions};
use yaswitch::core::result::ReasonCode;

#[test]
fn planner_generates_mutation_graph_for_valid_theme() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("sway".to_string()),
        target_filter: None,
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected preflight planning to succeed");

    assert_eq!(plan.status, PlanStatus::Planned);
    assert!(plan.dry_run);
    assert!(!plan.actions.is_empty());
    assert!(plan
        .actions
        .iter()
        .any(|action| action.action_type == ActionType::ApplyTarget));
    assert!(plan
        .actions
        .iter()
        .any(|action| action.action_type == ActionType::Reload));
}

#[test]
fn planner_marks_unsupported_reload_as_safe_skip() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("dwl".to_string()),
        target_filter: None,
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected preflight planning to succeed");

    let skip_action = plan
        .actions
        .iter()
        .find(|action| action.action_type == ActionType::SafeSkip)
        .expect("expected safe-skip action for unsupported reload");

    assert_eq!(
        skip_action.reason_code.as_deref(),
        Some(ReasonCode::SkipReloadUnsupported.as_str())
    );
}
