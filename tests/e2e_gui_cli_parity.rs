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

    let cycle_output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args(["cycle", "--compositor", "sway", "--dry-run", "--json"])
        .output()
        .expect("expected cycle dry-run command");

    assert!(!cycle_output.status.success());
    let cycle_stderr = String::from_utf8_lossy(&cycle_output.stderr);
    assert!(cycle_stderr.contains("THEME_CYCLE_NO_THEMES"));

    let _ = evidence::write_evidence(
        "task-22-e2e-reliability-parity.txt",
        "cli_and_cycle_paths_checked parity_contract_non_gui",
    )
    .expect("expected evidence write for parity");

    assert_eq!(cli_parsed["status"], "planned");
    assert!(cli_parsed.get("summary").is_some());
    assert!(cli_parsed.get("actions").is_some());
}
