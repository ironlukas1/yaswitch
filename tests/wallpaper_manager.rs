use std::fs;
use std::path::Path;

use yaswitch::core::result::ReasonCode;
use yaswitch::wallpaper::manager::{
    build_safe_wallpaper_command, default_state_file, load_wallpaper_state,
    persist_wallpaper_state, validate_wallpaper_path,
};

#[test]
fn wallpaper_manager_accepts_supported_image_paths() {
    let fixture = Path::new("tests/fixtures/wallpapers/sample-a.png");
    validate_wallpaper_path(fixture).expect("expected sample wallpaper to be valid");
}

#[test]
fn wallpaper_manager_persists_selected_wallpaper_state() {
    let root = temp_test_root("wallpaper-state");
    let state_file = default_state_file(&root);
    let wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png");

    persist_wallpaper_state(&state_file, wallpaper).expect("expected state persistence");
    let loaded = load_wallpaper_state(&state_file).expect("expected state to load");

    assert_eq!(loaded.current_wallpaper, wallpaper.to_string_lossy());
}

#[test]
fn wallpaper_command_wrapper_rejects_unsafe_arguments() {
    let unsafe_wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png;rm -rf /");
    let error = build_safe_wallpaper_command("swww", unsafe_wallpaper, "fill")
        .expect_err("expected unsafe command args rejection");

    assert_eq!(error.code(), ReasonCode::WallpaperCommandUnsafe);
}

fn temp_test_root(suffix: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "yaswitch-wallpaper-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if root.exists() {
        fs::remove_dir_all(&root).expect("expected previous test root removal");
    }
    fs::create_dir_all(&root).expect("expected test root creation");
    root
}
