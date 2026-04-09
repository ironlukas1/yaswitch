use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::result::ReasonCode;

#[test]
fn planner_returns_error_for_unknown_target_filter() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("sway".to_string()),
        target_filter: Some("does-not-exist".to_string()),
        dry_run: true,
    };

    let error = build_preflight_plan(&options)
        .expect_err("expected unknown --target value to fail preflight planning");

    assert_eq!(error.code(), ReasonCode::PlannerInvalidTarget);
}
