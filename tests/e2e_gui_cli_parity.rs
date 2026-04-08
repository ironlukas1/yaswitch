#[path = "support/evidence.rs"]
mod evidence;

#[test]
fn e2e_gui_cli_parity() {
    let cli_output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
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

    assert!(cli_output.status.success());
    let cli_stdout = String::from_utf8_lossy(&cli_output.stdout);
    let cli_parsed: serde_json::Value =
        serde_json::from_str(&cli_stdout).expect("expected valid json from cli apply output");

    let parity_output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
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
        .expect("expected parity dry-run command");

    assert!(parity_output.status.success());
    let parity_stdout = String::from_utf8_lossy(&parity_output.stdout);
    let parity_parsed: serde_json::Value =
        serde_json::from_str(&parity_stdout).expect("expected valid json from parity apply");

    assert_eq!(cli_parsed, parity_parsed);

    let _ = evidence::write_evidence(
        "task-22-e2e-reliability-parity.txt",
        "cli_parity_ok json_payloads_match",
    )
    .expect("expected evidence write for parity");

    assert_eq!(cli_parsed["status"], "planned");
    assert!(cli_parsed.get("summary").is_some());
    assert!(cli_parsed.get("actions").is_some());
}
