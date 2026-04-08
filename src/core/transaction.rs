use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::result::{ReasonCode, YaswitchError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub target_path: PathBuf,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournal {
    pub id: String,
    pub backup_root: PathBuf,
    pub entries: Vec<JournalEntry>,
    pub committed: bool,
    pub lock_path: PathBuf,
}

pub struct Transaction {
    journal_path: PathBuf,
    journal: TransactionJournal,
    staged_backups: HashMap<PathBuf, Option<PathBuf>>,
    lock_path: PathBuf,
    lock_released: bool,
}

impl Transaction {
    pub fn begin(state_root: &Path) -> Result<Self, YaswitchError> {
        fs::create_dir_all(state_root).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!("failed to create transaction state root: {error}"),
            )
        })?;

        let id = unique_id();
        let tx_root = state_root.join("transactions").join(&id);
        let backup_root = tx_root.join("backups");
        let lock_root = state_root.join("locks");
        fs::create_dir_all(&backup_root).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed to create backup root {}: {error}",
                    backup_root.display()
                ),
            )
        })?;

        fs::create_dir_all(&lock_root).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed to create lock root {}: {error}",
                    lock_root.display()
                ),
            )
        })?;

        let lock_path = lock_root.join("apply.lock");
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(YaswitchError::new(
                    ReasonCode::TransactionLockBusy,
                    format!(
                        "another apply transaction is in progress (lock: {})",
                        lock_path.display()
                    ),
                ));
            }
            Err(error) => {
                return Err(YaswitchError::new(
                    ReasonCode::TransactionIoFailed,
                    format!(
                        "failed to create transaction lock {}: {error}",
                        lock_path.display()
                    ),
                ));
            }
        }

        let journal = TransactionJournal {
            id,
            backup_root: backup_root.clone(),
            entries: Vec::new(),
            committed: false,
            lock_path: lock_path.clone(),
        };

        let journal_path = tx_root.join("journal.json");
        persist_journal(&journal_path, &journal)?;

        Ok(Self {
            journal_path,
            journal,
            staged_backups: HashMap::new(),
            lock_path,
            lock_released: false,
        })
    }

    pub fn write_file_atomic(
        &mut self,
        target_path: &Path,
        content: &str,
    ) -> Result<(), YaswitchError> {
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                YaswitchError::new(
                    ReasonCode::TransactionIoFailed,
                    format!(
                        "failed creating parent directory {}: {error}",
                        parent.display()
                    ),
                )
            })?;
        }

        if !self.staged_backups.contains_key(target_path) {
            let backup_path = if target_path.exists() {
                let backup_name = format!(
                    "{}-{}.bak",
                    sanitize_path(target_path),
                    self.journal.entries.len()
                );
                let backup_path = self.journal.backup_root.join(backup_name);
                fs::copy(target_path, &backup_path).map_err(|error| {
                    YaswitchError::new(
                        ReasonCode::TransactionIoFailed,
                        format!(
                            "failed backing up {} to {}: {error}",
                            target_path.display(),
                            backup_path.display()
                        ),
                    )
                })?;
                Some(backup_path)
            } else {
                None
            };

            self.staged_backups
                .insert(target_path.to_path_buf(), backup_path.clone());
            self.journal.entries.push(JournalEntry {
                target_path: target_path.to_path_buf(),
                backup_path,
            });
            persist_journal(&self.journal_path, &self.journal)?;
        }

        let tmp_path = target_path.with_extension("yaswitch.tmp");
        {
            let mut file = fs::File::create(&tmp_path).map_err(|error| {
                YaswitchError::new(
                    ReasonCode::TransactionIoFailed,
                    format!(
                        "failed creating temporary file {}: {error}",
                        tmp_path.display()
                    ),
                )
            })?;

            file.write_all(content.as_bytes()).map_err(|error| {
                YaswitchError::new(
                    ReasonCode::TransactionIoFailed,
                    format!(
                        "failed writing temporary file {}: {error}",
                        tmp_path.display()
                    ),
                )
            })?;
            file.sync_all().map_err(|error| {
                YaswitchError::new(
                    ReasonCode::TransactionIoFailed,
                    format!(
                        "failed syncing temporary file {}: {error}",
                        tmp_path.display()
                    ),
                )
            })?;
        }

        fs::rename(&tmp_path, target_path).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed atomic rename {} -> {}: {error}",
                    tmp_path.display(),
                    target_path.display()
                ),
            )
        })?;

        Ok(())
    }

    pub fn commit(mut self) -> Result<(), YaswitchError> {
        self.journal.committed = true;
        persist_journal(&self.journal_path, &self.journal)?;
        self.release_lock()?;
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<(), YaswitchError> {
        rollback_from_journal(&self.journal)?;
        self.release_lock()
    }

    pub fn rollback_token(&self) -> RollbackToken {
        RollbackToken {
            journal_path: self.journal_path.clone(),
        }
    }

    pub fn journal(&self) -> &TransactionJournal {
        &self.journal
    }
}

#[derive(Debug, Clone)]
pub struct RollbackToken {
    journal_path: PathBuf,
}

impl RollbackToken {
    pub fn recover(self) -> Result<(), YaswitchError> {
        let journal_content = fs::read_to_string(&self.journal_path).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed reading journal {}: {error}",
                    self.journal_path.display()
                ),
            )
        })?;

        let journal: TransactionJournal =
            serde_json::from_str(&journal_content).map_err(|error| {
                YaswitchError::new(
                    ReasonCode::TransactionJournalInvalid,
                    format!(
                        "invalid transaction journal {}: {error}",
                        self.journal_path.display()
                    ),
                )
            })?;

        if journal.committed {
            let _ = release_lock_file(&journal.lock_path);
            return Ok(());
        }

        rollback_from_journal(&journal)?;
        release_lock_file(&journal.lock_path)
    }
}

pub fn rollback_from_journal(journal: &TransactionJournal) -> Result<(), YaswitchError> {
    for entry in journal.entries.iter().rev() {
        match &entry.backup_path {
            Some(backup) => {
                if let Some(parent) = entry.target_path.parent() {
                    fs::create_dir_all(parent).map_err(|error| {
                        YaswitchError::new(
                            ReasonCode::TransactionIoFailed,
                            format!(
                                "failed creating rollback parent {}: {error}",
                                parent.display()
                            ),
                        )
                    })?;
                }
                fs::copy(backup, &entry.target_path).map_err(|error| {
                    YaswitchError::new(
                        ReasonCode::TransactionIoFailed,
                        format!(
                            "failed restoring backup {} to {}: {error}",
                            backup.display(),
                            entry.target_path.display()
                        ),
                    )
                })?;
            }
            None => {
                if entry.target_path.exists() {
                    fs::remove_file(&entry.target_path).map_err(|error| {
                        YaswitchError::new(
                            ReasonCode::TransactionIoFailed,
                            format!(
                                "failed removing newly-created file {} during rollback: {error}",
                                entry.target_path.display()
                            ),
                        )
                    })?;
                }
            }
        }
    }

    Ok(())
}

fn persist_journal(path: &Path, journal: &TransactionJournal) -> Result<(), YaswitchError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            YaswitchError::new(
                ReasonCode::TransactionIoFailed,
                format!(
                    "failed creating journal parent {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }

    let content = serde_json::to_string_pretty(journal).map_err(|error| {
        YaswitchError::new(
            ReasonCode::TransactionIoFailed,
            format!("failed serializing transaction journal: {error}"),
        )
    })?;

    fs::write(path, content).map_err(|error| {
        YaswitchError::new(
            ReasonCode::TransactionIoFailed,
            format!(
                "failed writing transaction journal {}: {error}",
                path.display()
            ),
        )
    })
}

fn sanitize_path(path: &Path) -> String {
    path.to_string_lossy()
        .chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' => '_',
            _ => ch,
        })
        .collect()
}

fn unique_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!("tx-{nanos}")
}

impl Transaction {
    fn release_lock(&mut self) -> Result<(), YaswitchError> {
        if self.lock_released {
            return Ok(());
        }

        release_lock_file(&self.lock_path)?;
        self.lock_released = true;
        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.lock_released {
            return;
        }
        let _ = release_lock_file(&self.lock_path);
        self.lock_released = true;
    }
}

fn release_lock_file(lock_path: &Path) -> Result<(), YaswitchError> {
    if !lock_path.exists() {
        return Ok(());
    }

    fs::remove_file(lock_path).map_err(|error| {
        YaswitchError::new(
            ReasonCode::TransactionIoFailed,
            format!(
                "failed removing transaction lock {}: {error}",
                lock_path.display()
            ),
        )
    })
}
