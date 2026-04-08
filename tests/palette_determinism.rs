use std::fs;
use std::path::Path;

use yaswitch::core::result::ReasonCode;
use yaswitch::palette::generator::{
    generate_palette_from_wallpaper, palette_cache_key, write_palette_cache,
};

#[test]
fn palette_is_deterministic_for_same_image_and_settings() {
    let wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png");

    let first = generate_palette_from_wallpaper(wallpaper)
        .expect("expected first palette generation to succeed");
    let second = generate_palette_from_wallpaper(wallpaper)
        .expect("expected second palette generation to succeed");

    assert_eq!(first, second);
}

#[test]
fn palette_cache_key_changes_when_inputs_change() {
    let wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png");
    let key_a = palette_cache_key(wallpaper, "mode=dark").expect("expected first cache key");
    let key_b = palette_cache_key(wallpaper, "mode=light").expect("expected second cache key");

    assert_ne!(key_a, key_b);
}

#[test]
fn palette_cache_key_changes_when_wallpaper_changes() {
    let wallpaper_a = Path::new("tests/fixtures/wallpapers/sample-a.png");
    let root = temp_test_root("palette-wallpaper-variant");
    let wallpaper_b = root.join("sample-b.png");

    let mut bytes = fs::read(wallpaper_a).expect("expected source wallpaper bytes");
    if let Some(first) = bytes.first_mut() {
        *first ^= 0x01;
    }
    fs::write(&wallpaper_b, bytes).expect("expected variant wallpaper write");

    let key_a = palette_cache_key(wallpaper_a, "mode=dark").expect("expected cache key a");
    let key_b = palette_cache_key(&wallpaper_b, "mode=dark").expect("expected cache key b");

    assert_ne!(key_a, key_b);
}

#[test]
fn palette_cache_write_succeeds() {
    let root = temp_test_root("palette-cache");
    let wallpaper = Path::new("tests/fixtures/wallpapers/sample-a.png");

    let palette = generate_palette_from_wallpaper(wallpaper)
        .expect("expected palette generation for cache write");
    let key = palette_cache_key(wallpaper, "mode=dark").expect("expected cache key");

    let cache_file =
        write_palette_cache(&root, &key, &palette).expect("expected cache write to succeed");
    assert!(cache_file.exists());
}

#[test]
fn corrupt_image_returns_decode_error() {
    let corrupt = Path::new("tests/fixtures/wallpapers/corrupt-image.png");
    let error = generate_palette_from_wallpaper(corrupt)
        .expect_err("expected corrupt wallpaper to fail decode");

    assert_eq!(error.code(), ReasonCode::WallpaperDecodeFailed);
}

fn temp_test_root(suffix: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "yaswitch-palette-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if root.exists() {
        fs::remove_dir_all(&root).expect("expected previous test root removal");
    }
    fs::create_dir_all(&root).expect("expected test root creation");
    root
}
