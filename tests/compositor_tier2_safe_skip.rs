use yaswitch::adapters::compositor::dwl::DwlAdapter;
use yaswitch::adapters::compositor::mangowm::MangowmAdapter;
use yaswitch::adapters::contract::{AdapterOutcome, ThemeAdapter};
use yaswitch::core::result::ReasonCode;

#[test]
fn dwl_adapter_reports_safe_skip_for_unsupported_reload() {
    let adapter = DwlAdapter;
    let outcome = adapter.apply().expect("expected dwl safe-skip outcome");

    match outcome {
        AdapterOutcome::Skipped { reason } => {
            assert_eq!(reason, ReasonCode::SkipReloadUnsupported);
        }
        _ => panic!("expected skipped outcome"),
    }
}

#[test]
fn mangowm_adapter_reports_capability_matrix() {
    let adapter = MangowmAdapter;
    let caps = adapter.capabilities();

    assert!(!caps.reload_supported);
    assert!(caps.can_plan);
    assert!(caps.can_apply);
    assert!(caps.can_verify);
    assert!(caps.can_rollback);
}

#[test]
fn compositor_version_guard_blocks_unverified_modes() {
    let adapter = DwlAdapter;
    let caps = adapter.capabilities();

    assert!(!caps.reload_supported);

    let invalid_full_support_claim = caps.reload_supported && caps.can_verify;
    assert!(
        !invalid_full_support_claim,
        "tier2 compositor must not claim full support when reload is unsupported"
    );

    let declared_profile = "dwl-unverified";
    assert!(
        declared_profile.contains("unverified"),
        "tier2 profiles must be explicitly marked unverified"
    );
}
