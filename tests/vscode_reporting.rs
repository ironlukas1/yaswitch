use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::report::build_report_from_plan;

#[test]
fn vscode_plan_actions_include_restart_risk_remediation() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-vscode".to_string(),
        compositor: Some("sway".to_string()),
        target_filter: Some("vscode".to_string()),
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected vscode plan generation");
    let report = build_report_from_plan(&plan, "planned");

    let vscode_actions: Vec<_> = report
        .actions
        .iter()
        .filter(|action| action.target == "vscode")
        .collect();

    assert!(!vscode_actions.is_empty());
    assert!(vscode_actions
        .iter()
        .all(|action| action.remediation_note.is_some()));
    assert!(vscode_actions
        .iter()
        .all(|action| action.remediation_command.is_some()));
}
