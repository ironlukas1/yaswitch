use std::fs;

#[test]
fn docs_consistency_includes_supported_compositors() {
    let compatibility =
        fs::read_to_string("docs/compatibility.md").expect("expected compatibility doc");
    let shortcuts = fs::read_to_string("docs/shortcuts.md").expect("expected shortcuts doc");

    assert!(compatibility.contains("Sway"));
    assert!(compatibility.contains("Hyprland"));
    assert!(compatibility.contains("Niri"));
    assert!(compatibility.contains("DWL"));
    assert!(compatibility.contains("MangoWM"));

    assert!(compatibility.contains("Tier-1"));
    assert!(compatibility.contains("Tier-2"));
    assert!(compatibility.contains("SKIP_RELOAD_UNSUPPORTED"));

    assert!(shortcuts.contains("yaswitch apply --theme"));
    assert!(shortcuts.contains("yaswitch cycle --compositor sway --json"));
}
