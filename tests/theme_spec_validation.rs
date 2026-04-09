use yaswitch::core::result::ReasonCode;
use yaswitch::core::theme_spec::load_theme_spec_from_dir;

#[test]
fn theme_spec_validation_accepts_valid_fixture() {
    let theme_dir = "tests/fixtures/themes/valid-minimal";
    let theme = load_theme_spec_from_dir(theme_dir)
        .expect("expected valid-minimal fixture to pass validation");

    assert_eq!(theme.schema_version, 1);
    assert_eq!(theme.theme_name, "Minimal");
    assert_eq!(theme.palette.base00, "#000000");
    assert!(theme.targets.contains_key("kitty"));
    assert_eq!(theme.wallpaper.mode.as_str(), "fill");
}

#[test]
fn theme_spec_validation_rejects_missing_required_keys() {
    let theme_dir = "tests/fixtures/themes/invalid-missing-app-map";
    let error = load_theme_spec_from_dir(theme_dir)
        .expect_err("expected invalid fixture to fail validation");

    assert_eq!(error.code(), ReasonCode::ThemeSchemaInvalid);
}
