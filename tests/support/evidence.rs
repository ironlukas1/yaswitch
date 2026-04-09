use std::fs;
use std::path::{Path, PathBuf};

use yaswitch::core::result::{ReasonCode, YaswitchError};

pub fn write_evidence(relative_path: &str, content: &str) -> Result<PathBuf, YaswitchError> {
    let evidence_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".sisyphus/evidence");
    let destination = evidence_root.join(relative_path);

    write_evidence_to_path(&destination, content)?;
    Ok(destination)
}

pub fn write_evidence_to_path(path: &Path, content: &str) -> Result<(), YaswitchError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            YaswitchError::new(
                ReasonCode::EvidenceWriteFailed,
                format!(
                    "failed to create evidence parent {}: {}",
                    parent.display(),
                    error
                ),
            )
        })?;
    }

    fs::write(path, content).map_err(|error| {
        YaswitchError::new(
            ReasonCode::EvidenceWriteFailed,
            format!(
                "failed to write evidence file {}: {}",
                path.display(),
                error
            ),
        )
    })
}
