#[test]
fn e2e_gui_cli_parity() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args([
            "apply",
            "--theme",
            "tests/fixtures/themes/valid-minimal",
            "--compositor",
            "sway",
            "--dry-run",
            "--json",
        ])
        .output()
        .expect("expected dry-run json command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("expected valid json from apply output");

    assert_eq!(parsed["status"], "planned");
    assert!(parsed.get("summary").is_some());
    assert!(parsed.get("actions").is_some());
}
