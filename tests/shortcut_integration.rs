use std::fs;

#[test]
fn shortcuts_doc_routes_to_shared_command_path() {
    let doc = fs::read_to_string("docs/integration/shortcuts.md")
        .expect("expected shortcuts integration doc to exist");

    assert!(doc.contains("yaswitch apply --theme"));
    assert!(doc.contains("--compositor"));
}

#[test]
fn shortcuts_doc_mentions_safe_skip_for_tier2() {
    let doc = fs::read_to_string("docs/integration/shortcuts.md")
        .expect("expected shortcuts integration doc to exist");

    assert!(doc.contains("SKIP_RELOAD_UNSUPPORTED"));
}

#[test]
fn shortcuts_doc_mentions_cycle_binding() {
    let doc = fs::read_to_string("docs/integration/shortcuts.md")
        .expect("expected shortcuts integration doc to exist");

    assert!(doc.contains("yaswitch cycle"));
    assert!(doc.contains("Mod1+Tab"));
}
