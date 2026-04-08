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

    let fixture = fs::read_to_string("tests/fixtures/compatibility/matrix.json")
        .expect("expected compatibility fixture");
    let fixture_matrix: Vec<serde_json::Value> =
        serde_json::from_str(&fixture).expect("expected valid fixture json");
    assert_eq!(fixture_matrix.len(), 5);

    for row in &fixture_matrix {
        let compositor = row["compositor"].as_str().expect("compositor field");
        assert!(compatibility.to_lowercase().contains(compositor));
    }

    assert!(shortcuts.contains("yaswitch apply --theme"));
    assert!(shortcuts.contains("yaswitch cycle --compositor sway --json"));
}
