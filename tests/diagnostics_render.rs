use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::report::build_report_from_plan;
use yaswitch::ui::diagnostics::render_diagnostics_text;

#[test]
fn diagnostics_render_contains_reason_codes() {
    let options = PlannerOptions {
        theme_dir: "tests/fixtures/themes/valid-minimal".to_string(),
        compositor: Some("dwl".to_string()),
        target_filter: None,
        dry_run: true,
    };

    let plan = build_preflight_plan(&options).expect("expected planner success");
    let report = build_report_from_plan(&plan, "planned");
    let rendered = render_diagnostics_text(&report);

    assert!(rendered.contains("reason_codes:"));
    assert!(rendered.contains("SKIP_RELOAD_UNSUPPORTED"));
    assert!(rendered.contains("remediation_note="));
    assert!(rendered.contains("remediation_command="));
}
