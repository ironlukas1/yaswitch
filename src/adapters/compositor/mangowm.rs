use crate::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use crate::core::result::{ReasonCode, YaswitchError};

pub struct MangowmAdapter;

impl ThemeAdapter for MangowmAdapter {
    fn id(&self) -> &'static str {
        "mangowm"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: false,
        }
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        Ok(AdapterOutcome::Skipped {
            reason: ReasonCode::SkipReloadUnsupported,
        })
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
