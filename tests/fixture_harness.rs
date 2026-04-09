#[path = "support/fixtures.rs"]
mod fixtures;

use fixtures::{fixtures_root, load_fixture_inventory};

#[test]
fn fixtures_can_be_loaded_for_all_core_suites() {
    let inventory =
        load_fixture_inventory(&fixtures_root()).expect("expected fixtures inventory to load");

    assert!(
        !inventory.themes.is_empty(),
        "expected at least one theme fixture"
    );
    assert!(
        !inventory.wallpapers.is_empty(),
        "expected at least one wallpaper fixture"
    );
    assert!(
        !inventory.adapters.is_empty(),
        "expected at least one adapter fixture"
    );
}
