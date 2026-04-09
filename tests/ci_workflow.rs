use std::fs;

#[test]
fn ci_workflow_contains_required_jobs() {
    let workflow_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/workflows/ci.yml");

    let workflow = fs::read_to_string(&workflow_path).expect("expected ci workflow file to exist");

    assert!(workflow.contains("jobs:"));
    assert!(workflow.contains("fmt:"));
    assert!(workflow.contains("clippy:"));
    assert!(workflow.contains("test:"));
    assert!(workflow.contains("actions/upload-artifact@v4"));
}
