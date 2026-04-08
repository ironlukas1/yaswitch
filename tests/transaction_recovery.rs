use std::fs;
use std::path::PathBuf;

use yaswitch::core::result::ReasonCode;
use yaswitch::core::transaction::Transaction;

#[test]
fn transaction_writes_are_atomic() {
    let root = temp_test_root("atomic");
    let state_root = root.join("state");
    let target_file = root.join("config").join("kitty.conf");

    fs::create_dir_all(target_file.parent().expect("expected parent directory"))
        .expect("expected target parent directory creation");
    fs::write(&target_file, "old-content").expect("expected initial file to be written");

    let mut tx = Transaction::begin(&state_root).expect("expected transaction to begin");
    tx.write_file_atomic(&target_file, "new-content")
        .expect("expected transaction write to succeed");
    tx.commit().expect("expected transaction commit");

    let file_content = fs::read_to_string(&target_file).expect("expected target to be readable");
    assert_eq!(file_content, "new-content");
}

#[test]
fn transaction_rolls_back_on_mid_apply_failure() {
    let root = temp_test_root("rollback-failure");
    let state_root = root.join("state");
    let target_file = root.join("config").join("waybar.css");

    fs::create_dir_all(target_file.parent().expect("expected parent directory"))
        .expect("expected target parent directory creation");
    fs::write(&target_file, "before-apply").expect("expected initial file content");

    let mut tx = Transaction::begin(&state_root).expect("expected transaction to begin");
    tx.write_file_atomic(&target_file, "partially-applied")
        .expect("expected in-transaction write");

    tx.rollback().expect("expected rollback to succeed");

    let restored = fs::read_to_string(&target_file).expect("expected rollback-restored file");
    assert_eq!(restored, "before-apply");
}

#[test]
fn transaction_recovers_after_simulated_crash() {
    let root = temp_test_root("crash-recovery");
    let state_root = root.join("state");
    let target_file = root.join("config").join("niri-theme.conf");

    fs::create_dir_all(target_file.parent().expect("expected parent directory"))
        .expect("expected target parent directory creation");
    fs::write(&target_file, "stable-before").expect("expected baseline file content");

    let mut tx = Transaction::begin(&state_root).expect("expected transaction to begin");
    tx.write_file_atomic(&target_file, "in-flight")
        .expect("expected write before simulated crash");

    let rollback_token = tx.rollback_token();
    rollback_token
        .recover()
        .expect("expected crash recovery to rollback from journal");

    let restored = fs::read_to_string(&target_file).expect("expected recovered file content");
    assert_eq!(restored, "stable-before");
}

#[test]
fn transaction_invalid_journal_is_reported() {
    let root = temp_test_root("invalid-journal");
    let state_root = root.join("state");
    let target_file = root.join("config").join("dummy.conf");

    fs::create_dir_all(target_file.parent().expect("expected parent directory"))
        .expect("expected target parent directory creation");

    let tx = Transaction::begin(&state_root).expect("expected transaction to begin");
    let token = tx.rollback_token();

    let tx_dir = tx
        .journal()
        .backup_root
        .parent()
        .expect("backup root should have transaction directory");
    let journal_path = tx_dir.join("journal.json");
    fs::write(&journal_path, "{ definitely-not-valid-json")
        .expect("expected invalid journal fixture write");

    let error = token
        .recover()
        .expect_err("expected invalid journal recovery to fail");
    assert_eq!(error.code(), ReasonCode::TransactionJournalInvalid);
}

#[test]
fn transaction_recover_releases_apply_lock() {
    let root = temp_test_root("recover-lock-release");
    let state_root = root.join("state");
    let target_file = root.join("config").join("niri.conf");

    fs::create_dir_all(target_file.parent().expect("expected parent directory"))
        .expect("expected target parent creation");
    fs::write(&target_file, "before").expect("expected baseline target write");

    let mut tx = Transaction::begin(&state_root).expect("expected transaction begin");
    tx.write_file_atomic(&target_file, "during")
        .expect("expected transactional write");

    let token = tx.rollback_token();
    token.recover().expect("expected crash recovery");

    let lock_file = state_root.join("locks").join("apply.lock");
    assert!(
        !lock_file.exists(),
        "expected lock file to be released after recover"
    );
}

fn temp_test_root(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "yaswitch-transaction-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("expected previous test directory removal");
    }
    fs::create_dir_all(&dir).expect("expected test directory creation");
    dir
}
