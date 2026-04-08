use crate::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use crate::core::result::{ReasonCode, YaswitchError};

pub struct VscodeAdapter {
    allow_restart: bool,
}

impl VscodeAdapter {
    pub fn new(allow_restart: bool) -> Self {
        Self { allow_restart }
    }

    pub fn restart_required(&self) -> bool {
        true
    }
}

impl ThemeAdapter for VscodeAdapter {
    fn id(&self) -> &'static str {
        "vscode"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: self.allow_restart,
        }
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        if self.allow_restart {
            Ok(AdapterOutcome::Applied)
        } else {
            Ok(AdapterOutcome::Skipped {
                reason: ReasonCode::SkipRestartPolicy,
            })
        }
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
