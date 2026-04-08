use yaswitch::adapters::apps::vscode::VscodeAdapter;
use yaswitch::adapters::contract::{AdapterOutcome, ThemeAdapter};
use yaswitch::core::result::ReasonCode;

#[test]
fn vscode_adapter_reports_restart_risk() {
    let adapter = VscodeAdapter::new(true);
    assert!(adapter.restart_required());
    assert!(adapter.capabilities().reload_supported);
}

#[test]
fn vscode_adapter_safe_skip_when_restart_disallowed() {
    let adapter = VscodeAdapter::new(false);
    assert!(adapter.restart_required());
    assert!(!adapter.capabilities().reload_supported);

    let outcome = adapter
        .apply()
        .expect("expected safe-skip outcome when restart is disabled");

    match outcome {
        AdapterOutcome::Skipped { reason } => {
            assert_eq!(reason, ReasonCode::SkipReloadUnsupported);
        }
        _ => panic!("expected skipped outcome"),
    }
}
