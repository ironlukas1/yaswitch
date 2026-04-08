use yaswitch::adapters::compositor::hyprland::HyprlandAdapter;
use yaswitch::adapters::compositor::niri::NiriAdapter;
use yaswitch::adapters::compositor::sway::SwayAdapter;
use yaswitch::adapters::contract::{validate_adapter_contract, AdapterOutcome, ThemeAdapter};
use yaswitch::core::result::ReasonCode;

#[test]
fn sway_adapter_contract_suite() {
    let adapter = SwayAdapter::new(true);
    validate_adapter_contract(&adapter).expect("expected sway adapter contract compliance");
}

#[test]
fn hyprland_adapter_contract_suite() {
    let adapter = HyprlandAdapter;
    validate_adapter_contract(&adapter).expect("expected hyprland adapter contract compliance");
}

#[test]
fn niri_adapter_contract_suite() {
    let adapter = NiriAdapter;
    validate_adapter_contract(&adapter).expect("expected niri adapter contract compliance");
}

#[test]
fn compositor_adapter_handles_unavailable_socket() {
    let adapter = SwayAdapter::new(false);
    assert!(!adapter.socket_available());
    assert!(!adapter.capabilities().reload_supported);

    let outcome = adapter
        .apply()
        .expect("expected unavailable socket to return safe-skip outcome");

    assert_eq!(
        outcome,
        AdapterOutcome::Skipped {
            reason: ReasonCode::CompositorSocketUnavailable
        }
    );

    let verify_error = adapter
        .verify()
        .expect_err("expected verify to fail without sway socket");
    assert_eq!(verify_error.code(), ReasonCode::CompositorSocketUnavailable);
}

#[test]
fn tier1_compositors_report_reload_support() {
    let sway = SwayAdapter::new(true);
    let hyprland = HyprlandAdapter;
    let niri = NiriAdapter;

    assert!(sway.capabilities().reload_supported);
    assert!(hyprland.capabilities().reload_supported);
    assert!(niri.capabilities().reload_supported);
}
