use std::fs;

use yaswitch::core::cycle::write_cycle_state_for_profile;

#[test]
fn cycle_state_profile_writes_isolated_state_files() {
    let root = temp_test_root("cycle-profiles");
    let state_root = root.join("state");
    let theme_a = root.join("themes").join("a");
    let theme_b = root.join("themes").join("b");

    fs::create_dir_all(&theme_a).expect("expected theme_a dir creation");
    fs::create_dir_all(&theme_b).expect("expected theme_b dir creation");

    let work_state = write_cycle_state_for_profile(&state_root, "work", &theme_a)
        .expect("expected work profile state write");
    let night_state = write_cycle_state_for_profile(&state_root, "night", &theme_b)
        .expect("expected night profile state write");

    assert_ne!(work_state, night_state);
    assert!(work_state.exists());
    assert!(night_state.exists());
}

fn temp_test_root(suffix: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "yaswitch-cycle-profile-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if root.exists() {
        fs::remove_dir_all(&root).expect("expected previous test root removal");
    }
    fs::create_dir_all(&root).expect("expected test root creation");
    root
}
