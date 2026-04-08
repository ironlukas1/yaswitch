use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::report::{build_report_from_plan, report_to_json};
use yaswitch::core::result::ReasonCode;

#[test]
fn report_schema_contains_required_fields() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("dwl".to_string()),
        target_filter: None,
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected planner to succeed");
    let report = build_report_from_plan(&plan, "planned");
    let json = report_to_json(&report).expect("expected report serialization");

    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("expected valid report json");
    assert_eq!(parsed["status"], "planned");
    assert!(parsed.get("actions").is_some());
    assert!(parsed.get("summary").is_some());
    assert!(parsed.get("reason_codes").is_some());
    assert!(parsed["summary"].get("skipped_actions").is_some());

    let actions = parsed["actions"]
        .as_array()
        .expect("expected report actions to be array");
    assert!(!actions.is_empty());
    for action in actions {
        assert!(action.get("action_status").is_some());
        assert!(action.get("remediation_note").is_some());
        assert!(action.get("remediation_command").is_some());
    }
}

#[test]
fn report_includes_reason_code_for_safe_skip() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("dwl".to_string()),
        target_filter: None,
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected planner to succeed");
    let report = build_report_from_plan(&plan, "planned");

    assert!(report
        .reason_codes
        .iter()
        .any(|code| code == ReasonCode::SkipReloadUnsupported.as_str()));

    let skip_action = report
        .actions
        .iter()
        .find(|action| {
            action.reason_code.as_deref() == Some(ReasonCode::SkipReloadUnsupported.as_str())
        })
        .expect("expected a safe-skip action in report");

    assert!(skip_action.remediation_note.is_some());
    assert!(skip_action.remediation_command.is_some());
}
