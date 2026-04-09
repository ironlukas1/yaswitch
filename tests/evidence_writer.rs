#[path = "support/evidence.rs"]
mod evidence;

use std::fs;

use evidence::{write_evidence, write_evidence_to_path};
use yaswitch::core::result::ReasonCode;

#[test]
fn evidence_writer_creates_expected_paths() {
    let evidence_path = write_evidence(
        "task-5/evidence_writer_creates_expected_paths.txt",
        "fixture harness online",
    )
    .expect("expected evidence writing to succeed");

    assert!(evidence_path.exists());

    let content =
        fs::read_to_string(&evidence_path).expect("expected created evidence file to be readable");
    assert_eq!(content, "fixture harness online");
}

#[test]
fn evidence_writer_fails_on_unwritable_path() {
    let failure_path = std::path::Path::new("/proc/yaswitch-evidence-denied/output.txt");
    let error = write_evidence_to_path(failure_path, "should fail")
        .expect_err("expected writing to /proc to be rejected");

    assert_eq!(error.code(), ReasonCode::EvidenceWriteFailed);
}
