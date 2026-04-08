use std::path::Path;

use yaswitch::palette::generator::generate_palette_from_wallpaper;

#[test]
fn palette_cli_contract_generates_stable_snapshot_data() {
    let wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png");
    let palette = generate_palette_from_wallpaper(wallpaper)
        .expect("expected palette generation for cli contract");

    assert_eq!(palette.colors.len(), 16);
    for color in &palette.colors {
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 7);
    }

    let json = serde_json::to_string_pretty(&palette).expect("expected palette serialization");
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("expected valid palette json");

    assert!(parsed.get("wallpaper_hash").is_some());
    assert!(parsed.get("colors").is_some());
}
