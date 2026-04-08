use std::fs;

#[test]
fn e2e_apply_lock_contention() {
    let state_root = std::env::temp_dir().join("yaswitch-runtime-state");
    let lock_path = state_root.join("locks").join("apply.lock");

    if lock_path.exists() {
        fs::remove_file(&lock_path).expect("expected stale lock removal");
    }
    fs::create_dir_all(lock_path.parent().expect("lock path must have parent"))
        .expect("expected lock parent dir creation");
    fs::write(&lock_path, "locked").expect("expected lock file creation");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args([
            "apply",
            "--theme",
            "tests/fixtures/themes/valid-minimal",
            "--compositor",
            "sway",
            "--json",
        ])
        .output()
        .expect("expected apply command execution under lock contention");

    let _ = fs::remove_file(&lock_path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("TRANSACTION_LOCK_BUSY"));
}
