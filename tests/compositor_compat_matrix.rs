use yaswitch::adapters::compositor::dwl::DwlAdapter;
use yaswitch::adapters::compositor::hyprland::HyprlandAdapter;
use yaswitch::adapters::compositor::mangowm::MangowmAdapter;
use yaswitch::adapters::compositor::niri::NiriAdapter;
use yaswitch::adapters::compositor::sway::SwayAdapter;
use yaswitch::adapters::contract::ThemeAdapter;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CompatibilityRow {
    compositor: String,
    tier: String,
    reload_supported: bool,
}

fn build_matrix() -> Vec<CompatibilityRow> {
    vec![
        CompatibilityRow {
            compositor: "sway".to_string(),
            tier: "tier1".to_string(),
            reload_supported: SwayAdapter::new(true).capabilities().reload_supported,
        },
        CompatibilityRow {
            compositor: "hyprland".to_string(),
            tier: "tier1".to_string(),
            reload_supported: HyprlandAdapter.capabilities().reload_supported,
        },
        CompatibilityRow {
            compositor: "niri".to_string(),
            tier: "tier1".to_string(),
            reload_supported: NiriAdapter.capabilities().reload_supported,
        },
        CompatibilityRow {
            compositor: "dwl".to_string(),
            tier: "tier2".to_string(),
            reload_supported: DwlAdapter.capabilities().reload_supported,
        },
        CompatibilityRow {
            compositor: "mangowm".to_string(),
            tier: "tier2".to_string(),
            reload_supported: MangowmAdapter.capabilities().reload_supported,
        },
    ]
}

#[test]
fn compositor_compatibility_matrix_reports_expected_capabilities() {
    let matrix = build_matrix();

    let sway = matrix.iter().find(|row| row.compositor == "sway").unwrap();
    let hyprland = matrix
        .iter()
        .find(|row| row.compositor == "hyprland")
        .unwrap();
    let niri = matrix.iter().find(|row| row.compositor == "niri").unwrap();
    let dwl = matrix.iter().find(|row| row.compositor == "dwl").unwrap();
    let mangowm = matrix
        .iter()
        .find(|row| row.compositor == "mangowm")
        .unwrap();

    assert!(sway.reload_supported);
    assert!(hyprland.reload_supported);
    assert!(niri.reload_supported);
    assert!(!dwl.reload_supported);
    assert!(!mangowm.reload_supported);

    let artifact = std::env::temp_dir().join("yaswitch-compatibility-matrix.json");
    let payload = serde_json::to_string_pretty(&matrix).expect("expected matrix serialization");
    std::fs::write(&artifact, payload).expect("expected matrix artifact write");
    assert!(artifact.exists());
}
