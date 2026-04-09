use std::fs;
use std::path::PathBuf;
use yaswitch::core::paths::{ensure_path_within_allowed_roots, PathEnv, RuntimePaths};
use yaswitch::core::result::ReasonCode;

#[test]
fn xdg_path_resolution_uses_expected_defaults() {
    let home = PathBuf::from("/tmp/yaswitch-home-defaults");
    let env = PathEnv {
        home: Some(home.clone()),
        xdg_config_home: None,
        xdg_state_home: None,
        xdg_cache_home: None,
    };

    let paths = RuntimePaths::from_env(&env).expect("expected runtime path resolution to work");

    assert_eq!(paths.config_dir, home.join(".config").join("yaswitch"));
    assert_eq!(paths.state_dir, home.join(".local/state").join("yaswitch"));
    assert_eq!(paths.cache_dir, home.join(".cache").join("yaswitch"));
    assert_eq!(
        paths.backup_dir,
        home.join(".local/state").join("yaswitch").join("backups")
    );
}

#[test]
fn path_sandbox_blocks_dotdot_escape() {
    let allowed_root = PathBuf::from("/tmp/yaswitch-root");
    let candidate = PathBuf::from("/tmp/yaswitch-root/subdir/../../etc/passwd");

    let error = ensure_path_within_allowed_roots(candidate, &[allowed_root])
        .expect_err("expected dotdot traversal to be rejected");

    assert_eq!(error.code(), ReasonCode::PathOutsideAllowedRoot);
    assert_eq!(error.code().as_str(), "PATH_OUTSIDE_ALLOWED_ROOT");
}

#[test]
fn path_sandbox_blocks_symlink_escape() {
    let workspace_root = temp_test_dir("symlink-escape");
    let sandbox_root = workspace_root.join("sandbox");
    let outside_root = workspace_root.join("outside");
    let symlinked_dir = sandbox_root.join("link-out");

    fs::create_dir_all(&sandbox_root).expect("expected sandbox root directory to be created");
    fs::create_dir_all(&outside_root).expect("expected outside root directory to be created");

    create_symlink_dir(&outside_root, &symlinked_dir)
        .expect("expected symlink creation for test fixture");

    let escaped_target = symlinked_dir.join("stolen.conf");

    let error = ensure_path_within_allowed_roots(escaped_target, &[sandbox_root])
        .expect_err("expected symlink escape path to be rejected");

    assert_eq!(error.code(), ReasonCode::PathOutsideAllowedRoot);
}

#[test]
fn doctor_output_contains_required_path_keys() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .arg("doctor")
        .arg("--json")
        .output()
        .expect("expected doctor command to execute");

    assert!(
        output.status.success(),
        "doctor command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("expected doctor output to be valid JSON");

    assert!(json.get("config_dir").is_some());
    assert!(json.get("state_dir").is_some());
    assert!(json.get("cache_dir").is_some());
    assert!(json.get("backup_dir").is_some());
}

fn temp_test_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "yaswitch-path-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("expected existing test dir to be removable");
    }
    fs::create_dir_all(&dir).expect("expected temp test dir to be created");
    dir
}

#[cfg(unix)]
fn create_symlink_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(not(unix))]
fn create_symlink_dir(_src: &std::path::Path, _dst: &std::path::Path) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "symlink tests require unix support",
    ))
}
