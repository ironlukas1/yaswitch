use std::fs;
use std::path::PathBuf;

use yaswitch::core::transaction::Transaction;

#[test]
fn e2e_atomic_recovery() {
    let root = temp_test_root("e2e-atomic-recovery");
    let state_root = root.join("state");
    let target_file = root.join("config").join("kitty.conf");

    fs::create_dir_all(target_file.parent().expect("target should have parent"))
        .expect("expected target parent creation");
    fs::write(&target_file, "before-apply").expect("expected baseline write");

    let mut tx = Transaction::begin(&state_root).expect("expected transaction begin");
    tx.write_file_atomic(&target_file, "during-apply")
        .expect("expected write during transaction");

    let rollback_token = tx.rollback_token();
    rollback_token
        .recover()
        .expect("expected recovery from in-flight transaction");

    let after = fs::read_to_string(&target_file).expect("expected restored target content");
    assert_eq!(after, "before-apply");
}

fn temp_test_root(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "yaswitch-e2e-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("expected stale dir removal");
    }
    fs::create_dir_all(&dir).expect("expected test root creation");
    dir
}
